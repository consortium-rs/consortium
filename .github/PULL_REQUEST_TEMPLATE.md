<!--
Thanks for contributing. Keep one logical change per PR, and see CONTRIBUTING.md
for the conventions this checklist refers to.
-->

## Summary

<!-- What changes, and why. Link the issue it closes, if any. -->

## Subsystem

<!-- core / ipc / tee / hmi / pac / hal / cfg / builder / cli / docs -->

## Testing

<!--
Which commands you ran, and on what host (Linux, macOS native, macOS via
`just linux '...'`, WSL). Say plainly what you could not test — hardware, an SDK,
or a target you do not have — rather than leaving it implied.
-->

```text

```

## Checklist

- [ ] `just lint` and `just check` pass (via the Linux container recipes on macOS).
- [ ] Tests cover the change and fail without it.
- [ ] Snapshots under `crates/consortium-cfg/tests/` were read, not just re-blessed.
- [ ] Generated output (PAC, `src/peripherals/<chip>.rs`, chip databases) was regenerated
      rather than hand-edited.
- [ ] New or changed `unsafe` states its hardware, memory, aliasing, or ABI invariant.
- [ ] New files carry the Apache-2.0 header and an `SPDX-License-Identifier`.
- [ ] New dependencies are Apache-2.0-compatible and respect `no_std`/`alloc`/`std`
      constraints.
- [ ] Public API and schema changes are reflected in the docs under `docs/book/`.

## Breaking changes

<!-- Wire formats, descriptor or memory layouts, manifest schema, public APIs. "None" is a fine answer. -->

## AI assistance

<!--
Consortium allows and encourages AI assistants; see the policy in CONTRIBUTING.md.
If a tool helped write this patch, add an `Assisted-by:` trailer to the commit and note
below which parts are assistant-written and how you reviewed them. Only humans sign off:
an agent must never add `Signed-off-by:`.
-->

- [ ] Any assistant-written code was reviewed to the standard in CONTRIBUTING.md, and
      commits carry an `Assisted-by:` trailer where applicable.
