# HMI Subsystem

Consortium is moving toward a PocketJS-first HMI. PocketJS should be the primary
runtime for native embedded interfaces; the existing Cog/WPE WebKit backend
remains useful as a compatibility path for applications that require browser
APIs or an existing web frontend.

The first backend split is implemented: `consortium-hmi` owns the portable
command registry, `consortium-hmi-webkit` owns the existing Cog shell, and
`consortium-hmi-pocket` hosts PocketJS with a headless renderer and
frame-boundary service delivery. `[hmi].engine` and the builder still accept
only the Cog/web directory pipeline, so PocketJS is not yet the packaged or
default engine.

- [PocketJS repository and integration notes](pocketjs.md)
