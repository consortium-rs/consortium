# Human-Machine Interface (HMI)

A Human-Machine Interface (HMI) provides the user-facing control and visualization layer of an embedded system. In HAMPU systems, the HMI commonly runs in the CA domain, where Linux provides graphics, input, windowing, and application framework support.

## Traditional Path: Qt

A common HMI development stack is Qt. In this model, the Linux side runs a Qt application that renders the user interface, handles user input, and communicates with lower-level services or processors.

```text
User input / display
        |
        v
Qt application
        |
        |  Qt Widgets / Qt Quick / QML
        v
Linux graphics and input stack
        |
        |  Wayland / DRM / framebuffer / evdev
        v
Display and touch hardware
```

Qt applications are typically built using either Qt Widgets or Qt Quick/QML. Qt Widgets is commonly used for conventional desktop-style interfaces. Qt Quick/QML is commonly used for touch-oriented, animated, and embedded HMI designs.

The HMI application usually does not directly control safety-critical or real-time functions. Instead, it communicates with backend services, IPC endpoints, device drivers, or MCU-side firmware to display system state and submit user commands.

## Chromium-based HMI

With modern MPUs providing stronger GPU and DSP capability, web-based HMI is becoming common in embedded systems. A typical approach is to run a Chromium-based application stack, including frameworks such as Tauri, for HTML, CSS, and JavaScript-based interfaces.

However, embedded HMI kiosk deployments usually need only a small subset of the full desktop/web-app feature set. Features such as complex window management, mobile targets, plugin systems, X11 support, and broad host integration are often unnecessary. This can make Chromium-based stacks relatively heavy for constrained embedded platforms.

## WPE WebKit-based HMI

WPE WebKit provides a lighter web-based HMI option for embedded kiosk systems. It is built around `libwpe`, `wpebackend-fdo`, and `wpewebkit`. `cog` is a WPE launcher and web application container designed for kiosk-mode use.

```text id="v2t4ne"
HMI web assets
        |
        v
cog
        |
        v
WPE WebKit
        |
        v
Wayland or direct rendering
        |
        v
GPU / display hardware
```

Compared with a full Chromium stack, this approach is smaller and more focused. Through Wayland or direct GPU rendering, `cog` can provide a practical embedded web HMI runtime with fewer unnecessary desktop-oriented components.

Consortium provides Rust bindings around `cog`. The `cogcore-sys` crate exposes direct `glib`-based FFI access, while `cogcore` provides a safer Rust wrapper over `cogcore`.

For HMI development, the model is inspired by Tauri but remains kiosk-focused. It supports separation between debug mode using a localhost page and release mode using bundled assets, as well as controlled browser-host communication for interaction between the web UI and the embedded backend.
