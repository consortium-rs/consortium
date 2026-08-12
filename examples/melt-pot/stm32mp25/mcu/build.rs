fn main() {
    // `csti build` generates this from the lowered remoteproc layout before
    // Cargo starts; changing the manifest must therefore relink the firmware.
    println!("cargo:rerun-if-changed=memory.x");

    println!("cargo:rustc-link-arg-bins=--nmagic");
    println!("cargo:rustc-link-arg-bins=-Tlink.x");
    println!("cargo:rustc-link-arg-bins=-Tdefmt.x");
}
