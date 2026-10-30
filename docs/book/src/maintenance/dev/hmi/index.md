# HMI Subsystem

Consortium is moving toward a PocketJS-first HMI. PocketJS should be the primary
runtime for native embedded interfaces; the existing Cog/WPE WebKit backend
remains useful as a compatibility path for applications that require browser
APIs or an existing web frontend.

The first backend split is implemented: `consortium-hmi` owns the portable
command registry, `consortium-hmi-webkit` owns the existing Cog shell, and
`consortium-hmi-pocket` hosts PocketJS with a headless renderer and
frame-boundary service delivery. `[hmi] engine = "pocket"` is supported by
`csti build` end to end (bun build, bundle/pak staging under `dist/hmi/`, and env
injection), and the optional `shell` feature hosts it in a `winit` window. The
production DRM/KMS + `libinput` shell is not implemented yet, so `pocket` with
`backend = "drm"` is rejected.

- [PocketJS repository and integration notes](pocketjs.md)
