# cogcore-sys

Raw FFI bindings to Cog, WPE, WebKit, and GLib/GObject, plus vendored Cog Core C sources.

**Generated. Linux-only. Not meant for direct use.**

## Depend on `cogcore` instead

[`cogcore`](../cogcore) wraps a focused subset of this surface with owned handles, GObject
refcounting, Rust string conversion, and `GError` conversion. This crate exists to be
wrapped, not called.

```text
consortium-hmi-webkit      KioskApp + bridge mounting
  └── cogcore                 safe subset  ← use this
        └── cogcore-sys          raw bindgen output + vendored C
```

## What `build.rs` does

1. Probes the system Cog/WPE/WebKit/GLib/libsoup headers with `pkg-config`.
2. Runs `bindgen` over `wrapper.h` against those include paths plus the vendored ones.
3. Compiles the **vendored** Cog Core C sources under `vendor/include/` into a static
   archive this crate owns.

### Why the C is vendored and rebuilt

Step 3 is the load-bearing one. Linking a system `libcogcore` would mean the headers and
the library could disagree — and in practice they do: the packaged Cog does not export
everything the vendored headers declare, `cog_init` and `cog_view_new` among them. So the
sources are compiled here to match the headers bindgen read.

The source list mirrors `cogcore_sources` in the vendored `meson.build`, minus
`cog-gamepad-manette.c`, which the checked-in `cog-config.h` disables
(`COG_ENABLE_GAMEPAD_MANETTE 0`).

Note that the *system* WPE, WebKit, GLib, and libsoup libraries are still linked — only
Cog Core itself is vendored.

## Build requirements

Cog, GLib/GIO, WPE, WebKit, and libsoup development packages, on **Linux**. Every item is
`cfg(target_os = "linux")` and the crate **cannot build on macOS at all** — which is why
`just publish-dry` requires Linux and `just publish-dry-host` exists as the partial macOS
run.

For a working environment use `.github/workflows/cogcore.yml`, a provisioned Linux
machine, or `just linux-image-full`, which layers the WPE WebKit/Cog `.deb` from
`consortium-rs/wpe-kiosk-stack-prebuilt` onto the container image. That `.deb` needs
Ubuntu 24.04 (GLIBC_2.38 and libicu74), not Debian bookworm.

## Lints

`lib.rs` allows `non_upper_case_globals`, `non_camel_case_types`, `non_snake_case`,
`dead_code`, and `clippy::all` crate-wide. These are exemptions for generated code —
bindgen output does not follow Rust naming conventions and not every declaration is
referenced in every build — not a relaxation of the workspace's discipline for
hand-written crates.

## License

`Apache-2.0 AND MIT` — **not** the workspace `Apache-2.0`, because the vendored Cog Core
sources and headers under `vendor/include/` are redistributed (see
`vendor/LICENSE-COG`) under their own terms. See this
crate's `LICENSE` file, and [LICENSE](../../LICENSE) for the project licence.
