from __future__ import annotations

import importlib.util
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path
from unittest.mock import patch


def load_script(name: str):
    path = Path(__file__).parents[1] / f"{name}.py"
    spec = importlib.util.spec_from_file_location(f"test_{name}_module", path)
    module = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = module
    spec.loader.exec_module(module)
    return module


publish = load_script("publish")
release = load_script("release")


class FakeRegistry:
    def __init__(self, versions):
        self._versions = versions

    def versions(self, _name):
        return self._versions


class PublishTests(unittest.TestCase):
    def test_semver_orders_prereleases_before_releases(self):
        self.assertLess(
            publish.SemVer.parse("1.0.0-rc.1"), publish.SemVer.parse("1.0.0")
        )
        self.assertLess(
            publish.SemVer.parse("1.0.0-1"), publish.SemVer.parse("1.0.0-alpha")
        )

    def test_classifies_new_update_skip_and_stale(self):
        cases = [
            (None, "new"),
            (["0.9.0"], "update"),
            (["1.0.0"], "skip"),
        ]
        for versions, expected in cases:
            package = publish.Package("demo", "1.0.0", Path("Cargo.toml"))
            publish.classify(package, FakeRegistry(versions))
            self.assertEqual(package.mode, expected)

        package = publish.Package("demo", "1.0.0", Path("Cargo.toml"))
        with self.assertRaises(RuntimeError):
            publish.classify(package, FakeRegistry(["1.0.0", "1.1.0"]))

    def test_rate_limit_scopes_are_independent(self):
        new = publish.TokenBucket("new", 1, 600, tokens=0, updated_at=100)
        update = publish.TokenBucket("update", 30, 60, tokens=30, updated_at=100)
        self.assertEqual(new.seconds_until_token(now=100), 600)
        self.assertEqual(update.seconds_until_token(now=100), 0)

    def test_scheduler_uses_another_scope_instead_of_waiting(self):
        now = publish.time.monotonic()
        buckets = {
            "new": publish.TokenBucket("new", 5, 600, tokens=0, updated_at=now),
            "update": publish.TokenBucket("update", 30, 60, tokens=30, updated_at=now),
        }
        new = publish.Package("a-new", "1.0.0", Path("new"), mode="new")
        update = publish.Package("z-update", "1.0.0", Path("update"), mode="update")
        chosen = publish.choose_next_task({new.name: new, update.name: update}, buckets)
        self.assertEqual(chosen.name, "z-update")

    def test_topological_order(self):
        leaf = publish.Package("leaf", "1.0.0", Path("leaf"))
        root = publish.Package("root", "1.0.0", Path("root"), dependencies={"leaf"})
        self.assertEqual(
            [item.name for item in publish.topological_order([root, leaf])],
            ["leaf", "root"],
        )

    def test_macos_publish_verifies_in_linux_then_uploads_on_host(self):
        package = publish.Package("demo", "1.0.0", Path("Cargo.toml"))
        completed = subprocess.CompletedProcess([], 0, stdout="", stderr="")
        with (
            patch.object(publish.sys, "platform", "darwin"),
            patch.object(publish.subprocess, "run", return_value=completed) as run,
        ):
            ok, _output = publish.cargo_publish(Path("."), package, dry_run=False)

        self.assertTrue(ok)
        self.assertEqual(
            run.call_args_list[0].args[0],
            [
                "just",
                "linux-full",
                "cargo publish --locked --package demo --dry-run",
            ],
        )
        self.assertEqual(
            run.call_args_list[1].args[0],
            [
                "cargo",
                "publish",
                "--locked",
                "--package",
                "demo",
                "--no-verify",
            ],
        )


class ReleaseTests(unittest.TestCase):
    def test_dirty_tree_requires_explicit_permission(self):
        with patch.object(release, "run", return_value=" M patched-header.h"):
            with self.assertRaisesRegex(RuntimeError, "--allow-dirty"):
                release.require_clean_tree(Path("."), allow_dirty=False)
            release.require_clean_tree(Path("."), allow_dirty=True)

    def test_github_login_falls_back_when_gh_is_unavailable(self):
        commit = release.Commit(
            "a" * 40,
            "Contributor",
            "123+developer@users.noreply.github.com",
            "fix: example",
        )
        with patch.object(release.shutil, "which", return_value=None):
            login = release.github_login(Path("."), "owner/repo", commit, {})
        self.assertEqual(login, "developer")

    def test_macos_rust_recipe_runs_in_linux(self):
        with (
            patch.object(release.sys, "platform", "darwin"),
            patch.object(release, "run") as run,
        ):
            release.run_rust_recipe("cargo-lint")
        run.assert_called_once_with(
            "just",
            "linux-full",
            "just cargo-lint",
            cwd=release.ROOT,
            dry_run=False,
        )

    def test_release_formats_generated_versions_before_linting(self):
        events = []
        with (
            patch.object(
                release,
                "apply_release_fixes",
                side_effect=lambda: events.append("fix"),
            ),
            patch.object(
                release,
                "update_all_manifests",
                side_effect=lambda _root, _version: events.append("version") or [],
            ),
            patch.object(
                release,
                "regenerate_lockfiles",
                side_effect=lambda _root, dry_run: events.append("locks"),
            ),
            patch.object(
                release,
                "format_release_files",
                side_effect=lambda: events.append("format"),
            ),
            patch.object(
                release,
                "run_release_linters",
                side_effect=lambda: events.append("lint"),
            ),
        ):
            release.prepare_release("0.2.0")
        self.assertEqual(events, ["fix", "version", "locks", "format", "lint"])

    def test_default_bump_before_and_after_one_point_zero(self):
        zero = release.Version.parse("0.4.2")
        stable = release.Version.parse("1.4.2")
        self.assertEqual(zero.default_component(), "patch")
        self.assertEqual(str(zero.bump("patch")), "0.4.3")
        self.assertEqual(stable.default_component(), "minor")
        self.assertEqual(str(stable.bump("minor")), "1.5.0")

    def test_updates_workspace_and_local_dependency_versions(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            dependency = root / "crates" / "dependency"
            consumer = root / "crates" / "consumer"
            dependency.mkdir(parents=True)
            consumer.mkdir(parents=True)
            (root / "examples").mkdir()
            (root / "Cargo.toml").write_text(
                '[workspace]\nmembers = ["crates/*"]\n\n'
                '[workspace.package]\nversion = "0.1.0"\n'
            )
            (dependency / "Cargo.toml").write_text(
                '[package]\nname = "dependency"\nversion.workspace = true\n'
            )
            (consumer / "Cargo.toml").write_text(
                '[package]\nname = "consumer"\nversion = "0.1.0"\n\n'
                "[dependencies]\n"
                'dependency = { path = "../dependency", version = "^0.1" }\n\n'
                '# dependency = { path = "../dependency" }\n\n'
                "[dev-dependencies]\n"
                'dependency = { path = "../dependency" }\n\n'
                "[target.'cfg(unix)'.dependencies]\n"
                'dependency = { features = ["std"], path = "../dependency" }\n'
            )

            release.update_cargo_versions(root, "0.2.0")

            self.assertIn('version = "0.2.0"', (root / "Cargo.toml").read_text())
            consumer_text = (consumer / "Cargo.toml").read_text()
            self.assertIn('version = "0.2.0"', consumer_text)
            self.assertIn('dependency = { path = "../dependency" }', consumer_text)
            self.assertIn('# dependency = { path = "../dependency" }', consumer_text)
            consumer_document = release.tomlkit.parse(consumer_text)
            self.assertEqual(
                consumer_document["target"]["cfg(unix)"]["dependencies"]["dependency"][
                    "version"
                ],
                "0.2.0",
            )

    def test_release_notes_categorize_conventional_commits(self):
        commits = [
            release.Commit("a" * 40, "Dev", "dev@example.com", "feat(ipc): connect"),
            release.Commit("b" * 40, "Dev", "dev@example.com", "unusual subject"),
        ]
        with (
            patch.object(release, "last_release_tag", return_value="v0.1.0"),
            patch.object(release, "commits_since", return_value=commits),
            patch.object(release, "github_login", return_value="developer"),
        ):
            notes = release.release_notes(Path("."), "owner/repo", "0.2.0")
        self.assertIn("# Features", notes)
        self.assertIn("- [ipc] [aaaaaaa] by @developer: connect", notes)
        self.assertIn("# Other contributions", notes)

    def test_release_workflow_has_required_matrix(self):
        root = Path(__file__).parents[2]
        workflow = root / ".github" / "workflows" / "release-build.yml"
        release.validate_release_workflow(workflow)
        release.validate_binstall_manifest(
            root / "crates" / "consortium-cli" / "Cargo.toml"
        )


if __name__ == "__main__":
    unittest.main()
