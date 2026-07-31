# Human-Machine Interface

An HMI belongs on the Linux-capable side of a heterogeneous system, where display,
input, graphics, and application services are available. Real-time or safety-critical
control remains behind typed service and IPC boundaries instead of moving into the UI
process.

Consortium separates the application-service bridge from the rendering engine.
`consortium-hmi` is a backend-neutral command registry: a UI sends a command and string
payload, and the bridge returns a response future. It contains no WebKit, GLib, Pocket,
or Tokio runtime state.

## Cog and WPE WebKit

`consortium-hmi-webkit` adapts the bridge to Cog/WPE WebKit. It is useful for existing
web applications and products that need browser APIs while retaining a kiosk-focused
embedded runtime.

```text
web application
      |
Consortium command bridge
      |
Cog / WPE WebKit
      |
Wayland or direct display backend
```

In development, the application can load the `[hmi].debug` URL from a local frontend
server. Release builds package the generated assets from `[hmi].build` and stage them
under `dist/www/`.

## Pocket

`consortium-hmi-pocket` is the native path. PocketJS compiles a reactive Solid or Vue
Vapor UI into a JavaScript bundle and resource pack consumed by a retained Rust UI
runtime. Consortium provides windowed and headless shells around the same frame
contract, plus the common service bridge.

The build pipeline accepts `engine = "pocket"`, invokes the Pocket build, and stages
`<app>.js` with `<app>.pak` under `dist/hmi/`. The manifest also carries the logical
viewport. Pocket currently supports Wayland and headless modes; a direct DRM/KMS shell
is not yet available.

## Choosing an engine

Choose Cog when compatibility with browser APIs or an existing web application matters
most. Choose Pocket when a native retained renderer, smaller browser-free runtime, and
headless visual testing fit the product better.

Both engines keep product services behind the same bridge, which limits engine-specific
code and makes it possible to change the renderer without redesigning backend commands.
See the [Project Manifest](../guide/manifest.md) for the current `[hmi]` fields.
