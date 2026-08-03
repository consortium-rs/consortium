# Security Policy

## Supported versions

Consortium is pre-1.0 and moving quickly. Only the latest release and `main` receive
security fixes; there are no maintained backport branches yet.

| Version      | Supported |
| ------------ | --------- |
| `main`       | ✅        |
| latest `0.x` | ✅        |
| older `0.x`  | ❌        |

## Reporting a vulnerability

**Do not open a public issue for a vulnerability.**

Report privately through
[GitHub Security Advisories](https://github.com/consortium-rs/consortium/security/advisories/new),
or by email to <7086cmd@gmail.com> if you cannot use GitHub.

Please include:

- What an attacker can do, and what access they need to do it — a local process on the
  A-core, a compromised coprocessor, physical access to a UART, a hostile TA.
- The affected crate and version or commit.
- Steps to reproduce, ideally a manifest plus commands.
- Any suggested fix, if you have one.

You should get an acknowledgement within a few days. This is a small project without a
paid security team, so please be patient with the fix timeline; we will keep you updated
and credit you in the advisory unless you prefer otherwise.

## Scope

Consortium sits between a Linux application, a real-time coprocessor, and in some
configurations a trusted execution environment. Findings that are in scope include:

- Memory-safety violations reachable from a peer core's data — malformed descriptors,
  slot headers, or framed UART packets that cause out-of-bounds access, aliasing
  violations, or unsound `unsafe` in the transport, doorbell, or codec paths.
- Validation gaps in `consortium-cfg` that let a manifest generate a system with
  overlapping regions, unowned peripherals, or a secure peripheral exposed to the
  non-secure alias.
- Generated DTS, UIO, Kconfig, or systemd output that grants more privilege or wider
  mappings than the manifest describes.
- TEE parameter conversion flaws that let a client application reach memory a trusted
  application did not intend to expose.
- Build-pipeline issues in `csti build` that stage or execute untrusted content.

Out of scope:

- Vulnerabilities in vendor firmware, OP-TEE, the Linux kernel, WebKit/WPE, or other
  upstream projects. Report those upstream; tell us if Consortium's defaults make them
  materially easier to hit.
- Attacks assuming an already-compromised A-core with root, where the coprocessor is not
  itself a privilege boundary in that configuration.
- The `consortium-ort` and `consortium-ffi` stubs, which export nothing.

## Disclosure

We prefer coordinated disclosure: we will work with you on a fix, publish an advisory
with a CVE where warranted, and release before or alongside public disclosure.
