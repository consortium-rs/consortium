{
  description = "Consortium: A-core/M-core IPC framework for heterogeneous SoCs";

  inputs = {
    nixpkgs.url      = "github:NixOS/nixpkgs/nixos-24.11";
    rust-overlay     = {
      url    = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    crane            = {
      url    = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url  = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, crane, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlays.default ];
        };

        # Stable Rust with both runtime targets.
        #   aarch64-unknown-linux-gnu  — A-core userspace (consortium-uio)
        #   thumbv7em-none-eabihf      — M-core firmware  (consortium-core, no_std)
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          targets = [
            "aarch64-unknown-linux-gnu"
            "thumbv7em-none-eabihf"
          ];
          extensions = [ "rust-src" "rust-analyzer" "clippy" ];
        };

        craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;

        # Cross-compilation package set for A-core.
        pkgsAarch64 = import nixpkgs {
          localSystem  = system;
          crossSystem  = { config = "aarch64-unknown-linux-gnu"; };
        };

        aarch64Env = {
          CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER =
            "${pkgsAarch64.stdenv.cc}/bin/${pkgsAarch64.stdenv.cc.targetPrefix}cc";
        };

        # ── Package outputs ──────────────────────────────────────────────────

        # consortium-uio built for the A-core (aarch64 Linux userspace).
        # This is the crate that links against the kernel UIO driver.
        consortiumUio = craneLib.buildPackage (aarch64Env // {
          src           = craneLib.cleanCargoSource ./.;
          cargoExtraArgs = "-p consortium-uio --target aarch64-unknown-linux-gnu";
          doCheck       = false; # host cannot execute aarch64 binaries
        });

        # consortium-core built for the M-core (Cortex-M7, no_std).
        # Produces a staticlib suitable for linking into M-core firmware.
        consortiumCoreM = craneLib.buildPackage {
          src           = craneLib.cleanCargoSource ./.;
          cargoExtraArgs = "-p consortium-core --target thumbv7em-none-eabihf";
          doCheck       = false;
        };

        # consortium-core built for the host (tests, CI).
        consortiumCoreHost = craneLib.buildPackage {
          src           = craneLib.cleanCargoSource ./.;
          cargoExtraArgs = "-p consortium-core --features std";
        };

      in {
        # ── packages ────────────────────────────────────────────────────────
        packages = {
          consortium-uio      = consortiumUio;
          consortium-core-m   = consortiumCoreM;
          consortium-core     = consortiumCoreHost;
          default             = consortiumUio;
        };

        # ── checks (run with `nix flake check`) ─────────────────────────────
        checks = {
          consortium-core     = consortiumCoreHost;
          # Clippy for the host workspace
          clippy = craneLib.cargoClippy {
            src             = craneLib.cleanCargoSource ./.;
            cargoExtraArgs  = "--features std";
            cargoClippyExtraArgs = "-- -D warnings";
          };
        };

        # ── devShells ────────────────────────────────────────────────────────
        devShells.default = pkgs.mkShell (aarch64Env // {
          buildInputs = [
            rustToolchain
            pkgs.pkg-config

            # A-core cross-compiler  (aarch64-unknown-linux-gnu-gcc)
            pkgsAarch64.stdenv.cc

            # M-core bare-metal toolchain  (arm-none-eabi-*)
            pkgs.gcc-arm-embedded

            # Useful extras
            pkgs.gdb
            pkgs.openocd
          ];

          shellHook = ''
            echo "consortium dev shell"
            echo "  A-core : aarch64-unknown-linux-gnu  (UIO userspace)"
            echo "  M-core : thumbv7em-none-eabihf      (bare-metal firmware)"
            echo ""
            echo "  cargo build -p consortium-uio    --target aarch64-unknown-linux-gnu"
            echo "  cargo build -p consortium-core   --target thumbv7em-none-eabihf"
          '';
        });
      }
    );
}
