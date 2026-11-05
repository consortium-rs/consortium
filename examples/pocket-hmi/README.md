# Pocket HMI example

A minimal PocketJS project for the `pocket` HMI engine. It depends on the npm
`@pocketjs/framework` package, pinned to the version that matches the PocketJS
crates `consortium-hmi-pocket` uses (`0.16.0`). Bump both together.

```bash
bun install
bun run build        # dist/status-main.{js,pak}
bun run build:hero   # dist/hero-main.{js,pak}, the demo shipped in the package
```

Run the built app in a desktop window from the repository root:

```bash
cargo run -p consortium-hmi-pocket --features shell --example kiosk -- examples/pocket-hmi/dist/status-main.js examples/pocket-hmi/dist/status-main.pak
```

In a manifest, point the HMI at this project (paths are relative to
`Consortium.toml`; these assume it sits at the repository root):

```toml
[hmi]
engine  = "pocket"
backend = "wayland"
source  = "examples/pocket-hmi"
build   = "examples/pocket-hmi/dist"
app     = "status-main"
```
