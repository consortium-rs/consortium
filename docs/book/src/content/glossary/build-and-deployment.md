# Build and Deployment

**BitBake hand-off**  
The optional final pipeline step that stages `dist/` into a Yocto recipe and may invoke
BitBake. Deployment to hardware remains the product integrator's responsibility.

**`csti`**  
The Consortium command-line tool. `csti build` drives validation, generation,
compilation, post-processing, and staging.

**Deployment**  
Installation and activation of built artifacts on a target. `csti` prepares deployment
inputs but does not modify a live device.

**`dist/`**  
The local staging tree produced by `csti build`. It can contain applications, firmware,
TEE and HMI artifacts, device-tree and kernel inputs, boot files, a systemd unit, and a
BitBake recipe.

**DTS / DTB**  
Device Tree Source and its compiled Device Tree Blob. Consortium can emit either a
merged board tree or an overlay containing reserved-memory, UIO, and remoteproc
integration.

**Remoteproc**  
The Linux framework that loads and controls remote-processor firmware. Consortium
post-processes firmware into the section layout expected by remoteproc.

**Staging**  
Assembling built artifacts and generated integration files under `dist/` without
installing them on the target.

**Sysroot**  
A target filesystem and development environment used when compiling the Linux
application for another architecture or distribution.
