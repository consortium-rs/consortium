m8_target := "thumbv8m.main-none-eabihf"
m7_target := "thumbv7em-none-eabihf"
m6_target := "thumbv6m-none-eabi"
a64_target := "aarch64-unknown-linux-gnu"
a32_target := "armv7-unknown-linux-gnueabihf"
rv32imc_target := "riscv32imc-unknown-none-elf"
rv32imac_target := "riscv32imac-unknown-none-elf"
rv32imafcp_target := "riscv32imafc-unknown-none-elf"
lint_crates := "consortium-builder consortium-cfg csti consortium-codec consortium-codec-macros consortium-codec-macros-impl consortium-dbg consortium-dts consortium-ffi consortium-hal-imx9 consortium-hal-stm32mp2 consortium-ipc consortium-ipc-doorbell-gpio consortium-ipc-doorbell-hsem consortium-ipc-doorbell-ipcc consortium-ipc-doorbell-mu consortium-ipc-macros consortium-ipc-macros-impl consortium-ipc-transport-fdcan consortium-ipc-transport-fdcan-embedded consortium-ipc-transport-memory consortium-ipc-transport-uart consortium-ipc-transport-uart-embedded consortium-ipc-transport-uart-unix consortium-ort consortium-runtime-mcu consortium-tee-macros consortium-tee-macros-impl melt-pot-shared"
rust_tools := "taplo-cli typos-cli cargo-deny cargo-expand cargo-insta cargo-shear"
ls_lint_version := "2.3.1"

# List available recipes
default:
    @just --list

# ── Core Commands (from CLAUDE.md) ───────────────────────────────────────────

# Install local development tools
setup: setup-uv setup-vp setup-rust-tools setup-rust-targets setup-ls-lint setup-chiptool

# Install uv
setup-uv:
    @if command -v uv >/dev/null 2>&1; then \
        echo "uv is already installed"; \
    else \
        curl -LsSf https://astral.sh/uv/install.sh | sh; \
    fi

# Install Vite+
setup-vp:
    @if command -v vp >/dev/null 2>&1; then \
        echo "vp is already installed"; \
    else \
        curl -fsSL https://vite.plus | bash; \
    fi

# Install Cargo-hosted development tools
setup-rust-tools: setup-cargo-binstall
    cargo binstall -y {{ rust_tools }}

# Install Rust targets used by cross-build, check, and test recipes
setup-rust-targets:
    rustup target add {{ a64_target }}
    rustup target add {{ a32_target }}
    rustup target add {{ m8_target }}
    rustup target add {{ m7_target }}
    rustup target add {{ rv32imc_target }}
    rustup target add {{ rv32imac_target }}
    rustup target add {{ rv32imafcp_target }}

# Install cargo-binstall, used for fast binary installs of Rust tooling
setup-cargo-binstall:
    @if command -v cargo-binstall >/dev/null 2>&1; then \
        echo "cargo-binstall is already installed"; \
    else \
        cargo install cargo-binstall; \
    fi

# Prime ls-lint through vp dlx
setup-ls-lint:
    vp dlx --yes @ls-lint/ls-lint@{{ ls_lint_version }} --version

# Install chiptool, used to validate/regenerate the peripheral access crates
setup-chiptool:
    @if command -v chiptool >/dev/null 2>&1; then \
        echo "chiptool is already installed"; \
    else \
        cargo install --git https://github.com/embassy-rs/chiptool --locked; \
    fi

# Build all crates
build:
    cargo build

# Run tests. With args: `just test ipc host`, `just test hmi host`, `just test tee host`, `just test cfg host`, `just test dbg host`, `just test codec host`.
test area="" scope="":
    @case "{{ area }}:{{ scope }}" in \
        ":") cargo test ;; \
        "ipc:"|"ipc:host") just test-ipc-host ;; \
        "ipc:a64") just test-ipc-a64 ;; \
        "ipc:a32") just test-ipc-a32 ;; \
        "ipc:m8") just test-ipc-m8 ;; \
        "ipc:m7") just test-ipc-m7 ;; \
        "hmi:"|"hmi:host") just test-hmi-host ;; \
        "hmi:a64") just test-hmi-a64 ;; \
        "hmi:a32") just test-hmi-a32 ;; \
        "tee:"|"tee:host") just test-tee-host ;; \
        "tee:a64") just test-tee-a64 ;; \
        "tee:a32") just test-tee-a32 ;; \
        "cfg:"|"cfg:host") just test-cfg-host ;; \
        "dbg:"|"dbg:host") just test-dbg-host ;; \
        "codec:"|"codec:host") just test-codec-host ;; \
        *) echo "Error: unknown test scope '{{ area }} {{ scope }}'"; echo "Try: just test ipc host | a64 | a32 | m8 | m7, just test hmi host | a64 | a32, just test tee host | a64 | a32, just test cfg host, just test dbg host, or just test codec host"; exit 1 ;; \
    esac

# Run tests for a specific crate (e.g. `just test-crate ipc`)
test-crate crate:
    @crate={{ quote(crate) }}; case "$crate" in consortium-*) ;; *) crate="consortium-$crate";; esac; cargo test -p "$crate"

# Run a single test by name (e.g. `just test-fn ipc my_test`)
test-fn crate test_name:
    @crate={{ quote(crate) }}; test_name={{ quote(test_name) }}; case "$crate" in consortium-*) ;; *) crate="consortium-$crate";; esac; cargo test -p "$crate" "$test_name"

# Lint with Clippy
clippy:
    cargo clippy

# Run all host-side linters
lint: fmt-check cargo-lint taplo-check typos ls-lint oxlint ruff-lint

# Run Rust lint checks that avoid HMI and OP-TEE CA/TA link dependencies
cargo-lint:
    @for crate in {{ lint_crates }}; do \
        cargo clippy -p "$crate" --all-targets -- -D warnings || exit 1; \
    done
    cargo clippy -p consortium-runtime-app --no-default-features --features mimx9x,stm32mp25x --all-targets -- -D warnings
    cargo clippy -p consortium-tee --no-default-features --features ta --all-targets -- -D warnings

# Lint JavaScript and TypeScript through Vite+
oxlint:
    vp lint

# Lint Python scripts
ruff-lint:
    uvx ruff check .

# Auto-fix lint and format issues
fix: cargo-fix oxfix ruff-fix fmt

# Apply Clippy autofixes, mirroring cargo-lint
cargo-fix:
    @for crate in {{ lint_crates }}; do \
        cargo clippy --fix --allow-dirty --allow-staged -p "$crate" --all-targets || exit 1; \
    done
    cargo clippy --fix --allow-dirty --allow-staged -p consortium-runtime-app --no-default-features --features mimx9x,stm32mp25x --all-targets
    cargo clippy --fix --allow-dirty --allow-staged -p consortium-tee --no-default-features --features ta --all-targets

# Apply JavaScript and TypeScript autofixes through Vite+
oxfix:
    vp lint --fix

# Apply Python autofixes
ruff-fix:
    uvx ruff check --fix .

# Format code
fmt:
    just rustfmt
    vp fmt
    uvx ruff format
    taplo format

# Check code formatting
fmt-check:
    just rustfmt --check
    vp fmt --check
    uvx ruff format --check
    taplo format --check

# Format only Consortium workspace crates. Running `cargo fmt` from the virtual
# workspace also discovers local path dependencies, including vendor submodules.
rustfmt *args:
    @for manifest in crates/*/Cargo.toml examples/melt-pot/shared/Cargo.toml; do \
        cargo fmt --manifest-path "$manifest" {{ args }} || exit 1; \
    done

# Format TOML files
taplo-fmt:
    taplo format

# Check TOML validity
taplo-check:
    taplo lint

# Check spelling
typos:
    typos

# Check file and directory names
ls-lint:
    vp dlx --yes @ls-lint/ls-lint@{{ ls_lint_version }}

# ── Host (dev / CI) ──────────────────────────────────────────────────────────

# Check all crates on the host
check:
    cargo check --workspace --features consortium-ipc/std

# ── Release (crates.io) ──────────────────────────────────────────────────────

# Crates that cannot build on macOS; excluded from the macOS-only dry run
nonlinux_excludes := "--exclude cogcore --exclude cogcore-sys --exclude consortium-hmi-webkit --exclude consortium-runtime-app"

# Report which crate names are already claimed on crates.io
publish-names:
    @for d in crates/*/; do \
        n=$(basename "$d"); \
        curl -sf "https://crates.io/api/v1/crates/$n" >/dev/null \
            && echo "TAKEN: $n"; \
    done; \
    echo "(the CLI publishes as 'csti', not 'consortium-cli' — see docs/PUBLISHING.md)"

# Authoritative publish dry run; route Rust verification through Linux on macOS
publish-dry:
    @if [ "$(uname)" = "Darwin" ]; then \
        just linux-full 'cargo publish --workspace --dry-run --allow-dirty'; \
    else \
        cargo publish --workspace --dry-run --allow-dirty; \
    fi

# Partial dry run for macOS: skips the crates that cannot compile there
publish-dry-host:
    cargo publish --workspace --dry-run {{ nonlinux_excludes }}

# Publish every publishable crate with registry-aware rate limiting
publish:
    uv run --project scripts --locked python scripts/publish.py

# ── A-core (aarch64 / Linux) ─────────────────────────────────────────────────

check-ipc-a64:
    @[ "$(uname)" = "Linux" ] || { echo "Error: A-core targets require Linux"; exit 1; }
    cargo check -p consortium-ipc --target {{ a64_target }} -F std
    cargo check -p consortium-ipc-transport-memory --target {{ a64_target }} -F std
    cargo check -p consortium-runtime-app --target {{ a64_target }}
    cargo check -p consortium-ipc-doorbell-hsem --target {{ a64_target }} -F std
    cargo check -p consortium-ipc-doorbell-ipcc --target {{ a64_target }} -F std
    cargo check -p consortium-ipc-doorbell-mu --target {{ a64_target }} -F std

build-ipc-a64:
    @[ "$(uname)" = "Linux" ] || { echo "Error: A-core targets require Linux"; exit 1; }
    cargo build -p consortium-ipc --release --target {{ a64_target }} -F std
    cargo build -p consortium-ipc-transport-memory --release --target {{ a64_target }} -F std
    cargo build -p consortium-runtime-app --release --target {{ a64_target }}
    cargo build -p consortium-ipc-doorbell-hsem --release --target {{ a64_target }} -F std
    cargo build -p consortium-ipc-doorbell-ipcc --release --target {{ a64_target }} -F std
    cargo build -p consortium-ipc-doorbell-mu --release --target {{ a64_target }} -F std

# Run/compile A-core IPC tests for aarch64 Linux.
test-ipc-a64:
    @[ "$(uname)" = "Linux" ] || { echo "Error: A-core tests require Linux"; exit 1; }
    cargo test -p consortium-ipc --target {{ a64_target }} -F std
    cargo test -p consortium-ipc-transport-memory --target {{ a64_target }} -F std
    cargo test -p consortium-ipc-doorbell-hsem --target {{ a64_target }} -F std
    cargo test -p consortium-ipc-doorbell-ipcc --target {{ a64_target }} -F std
    cargo test -p consortium-ipc-doorbell-mu --target {{ a64_target }} -F std

# ── A-core (armv7 / Linux) ──────────────────────────────────────────────────

check-ipc-a32:
    @[ "$(uname)" = "Linux" ] || { echo "Error: A-core targets require Linux"; exit 1; }
    cargo check -p consortium-ipc --target {{ a32_target }} -F std
    cargo check -p consortium-ipc-transport-memory --target {{ a32_target }} -F std
    cargo check -p consortium-runtime-app --target {{ a32_target }}
    cargo check -p consortium-ipc-doorbell-hsem --target {{ a32_target }} -F std
    cargo check -p consortium-ipc-doorbell-ipcc --target {{ a32_target }} -F std
    cargo check -p consortium-ipc-doorbell-mu --target {{ a32_target }} -F std

build-ipc-a32:
    @[ "$(uname)" = "Linux" ] || { echo "Error: A-core targets require Linux"; exit 1; }
    cargo build -p consortium-ipc --release --target {{ a32_target }} -F std
    cargo build -p consortium-ipc-transport-memory --release --target {{ a32_target }} -F std
    cargo build -p consortium-runtime-app --release --target {{ a32_target }}
    cargo build -p consortium-ipc-doorbell-hsem --release --target {{ a32_target }} -F std
    cargo build -p consortium-ipc-doorbell-ipcc --release --target {{ a32_target }} -F std
    cargo build -p consortium-ipc-doorbell-mu --release --target {{ a32_target }} -F std

# Run/compile A-core IPC tests for armv7 Linux.
test-ipc-a32:
    @[ "$(uname)" = "Linux" ] || { echo "Error: A-core tests require Linux"; exit 1; }
    cargo test -p consortium-ipc --target {{ a32_target }} -F std
    cargo test -p consortium-ipc-transport-memory --target {{ a32_target }} -F std
    cargo test -p consortium-ipc-doorbell-hsem --target {{ a32_target }} -F std
    cargo test -p consortium-ipc-doorbell-ipcc --target {{ a32_target }} -F std
    cargo test -p consortium-ipc-doorbell-mu --target {{ a32_target }} -F std

# ── HMI (Cog/WPE / Linux) ──────────────────────────────────────────────────

# Run host-side HMI tests, including both adapters and the Cog wrapper layers.
test-hmi-host:
    cargo test -p consortium-hmi
    cargo test -p consortium-hmi-pocket
    cargo test -p consortium-hmi-webkit
    cargo test -p cogcore-sys
    cargo test -p cogcore

# Run/compile HMI tests for aarch64 Linux.
test-hmi-a64:
    @[ "$(uname)" = "Linux" ] || { echo "Error: A-core HMI tests require Linux"; exit 1; }
    cargo test -p cogcore-sys --target {{ a64_target }}
    cargo test -p cogcore --target {{ a64_target }}
    cargo test -p consortium-hmi --target {{ a64_target }}
    cargo test -p consortium-hmi-webkit --target {{ a64_target }}
    cargo test -p consortium-hmi-pocket --target {{ a64_target }}

# Run/compile HMI tests for armv7 Linux.
test-hmi-a32:
    @[ "$(uname)" = "Linux" ] || { echo "Error: A-core HMI tests require Linux"; exit 1; }
    cargo test -p cogcore-sys --target {{ a32_target }}
    cargo test -p cogcore --target {{ a32_target }}
    cargo test -p consortium-hmi --target {{ a32_target }}
    cargo test -p consortium-hmi-webkit --target {{ a32_target }}
    cargo test -p consortium-hmi-pocket --target {{ a32_target }}

# ── TEE (Trusted Execution Environment / Linux) ──────────────────────────────

# Check TEE crates
check-tee:
    @[ "$(uname)" = "Linux" ] || { echo "Error: TEE crates require Linux"; exit 1; }
    cargo check -p consortium-tee
    cargo check -p consortium-tee --no-default-features --features ta
    cargo check -p consortium-tee-macros
    cargo check -p consortium-tee-macros-impl

# Check TEE crates with panic=abort (for TA binaries)
check-tee-test:
    @[ "$(uname)" = "Linux" ] || { echo "Error: TEE test crate requires Linux"; exit 1; }
    RUSTFLAGS="-C panic=abort" cargo check -p consortium-tee-test

# Build TEE crates
build-tee:
    @[ "$(uname)" = "Linux" ] || { echo "Error: TEE crates require Linux"; exit 1; }
    cargo build -p consortium-tee --release
    cargo build -p consortium-tee-macros --release
    cargo build -p consortium-tee-macros-impl --release

# Build TEE test crate with panic=abort
build-tee-test:
    @[ "$(uname)" = "Linux" ] || { echo "Error: TEE test crate requires Linux"; exit 1; }
    RUSTFLAGS="-C panic=abort" cargo build -p consortium-tee-test --release

# Run host TEE tests that do not require OP-TEE runtime libraries.
test-tee:
    just test-tee-host

test-tee-host:
    # `ta` is host-agnostic (no optee-teec/Linux deps) and is required for the
    # tee_command trybuild fixtures to exercise the TA codegen path. CI has no
    # optee-* bindings, so the `ca` path stays disabled.
    cargo test -p consortium-tee --no-default-features --features ta
    cargo test -p consortium-tee-macros-impl --test snap

# Run TEE tests for Linux A-core targets.
test-tee-a64:
    @[ "$(uname)" = "Linux" ] || { echo "Error: A-core TEE tests require Linux"; exit 1; }
    cargo test -p consortium-tee --no-default-features --target {{ a64_target }}
    cargo test -p consortium-tee-macros-impl --test snap

test-tee-a32:
    @[ "$(uname)" = "Linux" ] || { echo "Error: A-core TEE tests require Linux"; exit 1; }
    cargo test -p consortium-tee --no-default-features --target {{ a32_target }}
    cargo test -p consortium-tee-macros-impl --test snap

# ── M-core (thumbv8 / bare-metal) ───────────────────────────────────────────

# thumbv8m serves two families: the STM32MP2 CM33 and the i.MX 93 CM33. The MU
# doorbell and the chip-glue transports must name `mimx93` explicitly — their
# default is `mimx95`, whose CM7 is a thumbv7 part, so a defaulted build here
# compiles the wrong chip map for this target.
check-ipc-m8:
    cargo check -p consortium-ipc --target {{ m8_target }}
    cargo check -p consortium-ipc-transport-memory --target {{ m8_target }} --no-default-features
    cargo check -p consortium-ipc-transport-memory --target {{ m8_target }} --no-default-features --features mimx93
    cargo check -p consortium-ipc-transport-fdcan --target {{ m8_target }} --no-default-features
    cargo check -p consortium-ipc-transport-fdcan-embedded --target {{ m8_target }} --features stm32mp25x
    cargo check -p consortium-ipc-transport-fdcan-embedded --target {{ m8_target }} --features mimx93
    cargo check -p consortium-ipc-transport-uart --target {{ m8_target }} --no-default-features
    cargo check -p consortium-ipc-transport-uart-embedded --target {{ m8_target }} --features stm32mp25x
    cargo check -p consortium-ipc-transport-uart-embedded --target {{ m8_target }} --features mimx93
    cargo check -p consortium-runtime-mcu --target {{ m8_target }}
    cargo check -p consortium-runtime-mcu --target {{ m8_target }} --no-default-features --features ipc-shm,mimx93
    cargo check -p consortium-runtime-mcu --target {{ m8_target }} --no-default-features --features ipc-uart
    cargo check -p consortium-runtime-mcu --target {{ m8_target }} --no-default-features --features ipc-fdcan
    cargo check -p consortium-ipc-doorbell-hsem --target {{ m8_target }}
    cargo check -p consortium-ipc-doorbell-ipcc --target {{ m8_target }}
    cargo check -p consortium-ipc-doorbell-mu --target {{ m8_target }} --no-default-features --features mimx93

build-ipc-m8:
    cargo build -p consortium-ipc --release --target {{ m8_target }}
    cargo build -p consortium-ipc-transport-memory --release --target {{ m8_target }} --no-default-features
    cargo build -p consortium-ipc-transport-memory --release --target {{ m8_target }} --no-default-features --features mimx93
    cargo build -p consortium-ipc-transport-fdcan --release --target {{ m8_target }} --no-default-features
    cargo build -p consortium-ipc-transport-fdcan-embedded --release --target {{ m8_target }} --features stm32mp25x
    cargo build -p consortium-ipc-transport-fdcan-embedded --release --target {{ m8_target }} --features mimx93
    cargo build -p consortium-ipc-transport-uart --release --target {{ m8_target }} --no-default-features
    cargo build -p consortium-ipc-transport-uart-embedded --release --target {{ m8_target }} --features stm32mp25x
    cargo build -p consortium-ipc-transport-uart-embedded --release --target {{ m8_target }} --features mimx93
    cargo build -p consortium-runtime-mcu --release --target {{ m8_target }}
    cargo build -p consortium-ipc-doorbell-hsem --release --target {{ m8_target }}
    cargo build -p consortium-ipc-doorbell-ipcc --release --target {{ m8_target }}
    cargo build -p consortium-ipc-doorbell-mu --release --target {{ m8_target }} --no-default-features --features mimx93

# Compile M-core IPC crates for thumbv8. Unit tests are not runnable on the host.
test-ipc-m8: check-ipc-m8

# ── M-core (thumbv7 / bare-metal) ───────────────────────────────────────────

check-ipc-m7:
    cargo check -p consortium-ipc --target {{ m7_target }}
    cargo check -p consortium-ipc-transport-memory --target {{ m7_target }} --no-default-features
    cargo check -p consortium-ipc-transport-fdcan --target {{ m7_target }} --no-default-features
    cargo check -p consortium-ipc-transport-fdcan-embedded --target {{ m7_target }} --features mimx95
    cargo check -p consortium-ipc-transport-uart --target {{ m7_target }} --no-default-features
    cargo check -p consortium-ipc-transport-uart-embedded --target {{ m7_target }} --features mimx95
    cargo check -p consortium-runtime-mcu --target {{ m7_target }}
    cargo check -p consortium-runtime-mcu --target {{ m7_target }} --no-default-features --features ipc-uart
    cargo check -p consortium-runtime-mcu --target {{ m7_target }} --no-default-features --features ipc-fdcan
    cargo check -p consortium-ipc-doorbell-hsem --target {{ m7_target }}
    cargo check -p consortium-ipc-doorbell-ipcc --target {{ m7_target }}
    cargo check -p consortium-ipc-doorbell-mu --target {{ m7_target }}

build-ipc-m7:
    cargo build -p consortium-ipc --release --target {{ m7_target }}
    cargo build -p consortium-ipc-transport-memory --release --target {{ m7_target }} --no-default-features
    cargo build -p consortium-ipc-transport-fdcan --release --target {{ m7_target }} --no-default-features
    cargo build -p consortium-ipc-transport-fdcan-embedded --release --target {{ m7_target }} --features mimx95
    cargo build -p consortium-ipc-transport-uart --release --target {{ m7_target }} --no-default-features
    cargo build -p consortium-ipc-transport-uart-embedded --release --target {{ m7_target }} --features mimx95
    cargo build -p consortium-runtime-mcu --release --target {{ m7_target }}
    cargo build -p consortium-ipc-doorbell-hsem --release --target {{ m7_target }}
    cargo build -p consortium-ipc-doorbell-ipcc --release --target {{ m7_target }}
    cargo build -p consortium-ipc-doorbell-mu --release --target {{ m7_target }}

# Compile M-core IPC crates for thumbv7. Unit tests are not runnable on the host.
test-ipc-m7: check-ipc-m7

# Run host-side IPC tests, including fake-MMIO std and no-std-shaped wait-future tests.
test-ipc-host:
    cargo test -p consortium-ipc --features std
    cargo test -p consortium-ipc-transport-memory
    cargo test -p consortium-ipc-transport-memory --no-default-features --test async_atomic_waker_test
    cargo test -p consortium-ipc-doorbell-hsem
    cargo test -p consortium-ipc-doorbell-hsem --features std
    cargo test -p consortium-ipc-doorbell-ipcc
    cargo test -p consortium-ipc-doorbell-ipcc --features std
    cargo test -p consortium-ipc-doorbell-mu
    cargo test -p consortium-ipc-doorbell-mu --features std
    cargo test -p consortium-ipc-doorbell-mu --no-default-features --features mimx93,std
    cargo test -p consortium-ipc-transport-fdcan
    cargo test -p consortium-ipc-transport-fdcan --no-default-features
    cargo test -p consortium-ipc-transport-uart
    cargo test -p consortium-ipc-transport-uart --no-default-features --test loopback_embedded
    cargo test -p consortium-ipc-transport-uart-unix

# ── Configuration (host-only) ───────────────────────────────────────────────

# Run host-side configuration tests: schema/validation, lowering, and the
# codegen/DTS snapshot fixtures. These crates are host-only (no target builds).
test-cfg-host:
    cargo test -p consortium-cfg-common
    cargo test -p consortium-cfg

# Run host-side debug-logging tests: descriptor/ring behavior and the logger,
# plus the defmt `decode` path (defmt-decoder host reader).
test-dbg-host:
    cargo test -p consortium-dbg
    cargo test -p consortium-dbg --features decode

# Run host-side codec tests across all families, plus the derive-macro
# snapshot/trybuild coverage.
test-codec-host:
    cargo test -p consortium-codec --features postcard,prost,rkyv,alloc
    cargo test -p consortium-codec-macros-impl

# ── RISC-V (rv32imc) ────────────────────────────────────────────────────────

check-rv32imc:
    cargo check -p consortium-ipc --target {{ rv32imc_target }}

build-rv32imc:
    cargo build -p consortium-ipc --release --target {{ rv32imc_target }}

# ── RISC-V (rv32imac) ───────────────────────────────────────────────────────

check-rv32imac:
    cargo check -p consortium-ipc --target {{ rv32imac_target }}

build-rv32imac:
    cargo build -p consortium-ipc --release --target {{ rv32imac_target }}

# ── RISC-V (rv32imafcp) ─────────────────────────────────────────────────────

check-rv32imafcp:
    cargo check -p consortium-ipc --target {{ rv32imafcp_target }}

build-rv32imafcp:
    cargo build -p consortium-ipc --release --target {{ rv32imafcp_target }}

# ── Combined ─────────────────────────────────────────────────────────────────

check-ipc-m: check-ipc-m8 check-ipc-m7

build-ipc-m: build-ipc-m8 build-ipc-m7

check-ipc-a: check-ipc-a64 check-ipc-a32

build-ipc-a: build-ipc-a64 build-ipc-a32

check-rv: check-rv32imc check-rv32imac check-rv32imafcp

build-rv: build-rv32imc build-rv32imac build-rv32imafcp

check-ipc-arm: check-ipc-a check-ipc-m

build-ipc-arm: build-ipc-a build-ipc-m

# ── Linux container (Apple `container` on macOS) ─────────────────────────────
# The `*-a64` / `*-a32` recipes above refuse to run on a non-Linux host. On
# Apple Silicon, Apple `container` boots a *native* arm64 Linux VM, so those
# targets build natively there and `[profile] sysroot = "/"` resolves to the
# image root — no Yocto SDK required. See `Dockerfile.linux`.
#
# `--dns` is not optional on a host whose VPN hands out fake-ip DNS (the
# 198.18.0.0/15 pool): the VM inherits the mapped address, cannot route it, and
# every network call hangs silently instead of failing.

# Named volumes keep Linux artifacts out of the macOS ./target tree; sharing one
# target dir between the two makes every host/container switch a full rebuild.
linux_image     := "consortium-linux"
linux_dns       := env_var_or_default("CONSORTIUM_LINUX_DNS", "1.1.1.1")
linux_cpus      := env_var_or_default("CONSORTIUM_LINUX_CPUS", "6")
linux_memory    := env_var_or_default("CONSORTIUM_LINUX_MEMORY", "8192M")
linux_wpe_ver   := "2.52.4"
linux_wpe_file  := "wpe-kiosk-stack_" + linux_wpe_ver + "_arm64.deb"
linux_wpe_url   := "https://github.com/consortium-rs/wpe-kiosk-stack-prebuilt/releases/download/v" + linux_wpe_ver + "/" + linux_wpe_file
# Defaults into the gitignored `assets/`, which `linux-image-full` fills on
# demand. Point CONSORTIUM_WPE_DEB at an existing copy to skip the download.
linux_wpe_deb   := env_var_or_default("CONSORTIUM_WPE_DEB", justfile_directory() + "/assets/" + linux_wpe_file)
linux_ctx       := justfile_directory() + "/target/container-context"
# Kernel checkout handed to `csti build --linux` inside the container, mounted
# at /root/linux. Cloned (shallow) on first use when the host has none.
linux_kernel     := env_var_or_default("CONSORTIUM_LINUX_KERNEL", home_directory() + "/Developer/linux")
linux_kernel_url := env_var_or_default("CONSORTIUM_LINUX_KERNEL_URL", "https://github.com/torvalds/linux.git")

# Start the Apple `container` services and builder with a routable resolver
linux-setup:
    container system start
    @container builder status >/dev/null 2>&1 \
      && echo "builder already running (restart with 'container builder stop' if DNS changed)" \
      || container builder start --dns {{ linux_dns }} --cpus 4 --memory 4096M
    @container volume create consortium-target >/dev/null 2>&1 || true
    @container volume create consortium-cargo >/dev/null 2>&1 || true

# Build the base Linux image (Rust toolchains + uv + Vite+ + dtc)
linux-image: linux-setup
    @mkdir -p {{ linux_ctx }}
    cp {{ justfile_directory() }}/rust-toolchain.toml {{ linux_ctx }}/rust-toolchain.toml
    container build --target base -t {{ linux_image }}:base \
      -f {{ justfile_directory() }}/Dockerfile.linux {{ linux_ctx }}

# ~45 MB, so this is on-demand rather than part of `just setup`: only the
# `linux-image-full` path needs it, and most hosts never build that image.
#
# Fetch the prebuilt WPE WebKit/Cog .deb into the gitignored `assets/`
setup-wpe-deb:
    @if [ -f "{{ linux_wpe_deb }}" ]; then \
        echo "WPE .deb already present: {{ linux_wpe_deb }}"; \
    else \
        echo "Downloading {{ linux_wpe_file }} to {{ linux_wpe_deb }}"; \
        mkdir -p "$(dirname "{{ linux_wpe_deb }}")"; \
        curl -fL --progress-bar -o "{{ linux_wpe_deb }}.part" "{{ linux_wpe_url }}" \
            && mv "{{ linux_wpe_deb }}.part" "{{ linux_wpe_deb }}" \
            || { rm -f "{{ linux_wpe_deb }}.part"; \
                 echo "Download failed; fetch {{ linux_wpe_url }} by hand or set CONSORTIUM_WPE_DEB."; \
                 exit 1; }; \
    fi

# Build the full Linux image (+ WPE WebKit/Cog and the OP-TEE TA dev kit)
linux-image-full: linux-setup setup-wpe-deb
    @mkdir -p {{ linux_ctx }}
    cp {{ justfile_directory() }}/rust-toolchain.toml {{ linux_ctx }}/rust-toolchain.toml
    cp "{{ linux_wpe_deb }}" {{ linux_ctx }}/$(basename "{{ linux_wpe_deb }}")
    container build --target full -t {{ linux_image }}:full \
      --build-arg WPE_DEB=$(basename "{{ linux_wpe_deb }}") \
      -f {{ justfile_directory() }}/Dockerfile.linux {{ linux_ctx }}

# Run a command in the base Linux image, e.g. `just linux just check-ipc-a64`
linux +cmd: (linux-run "base" cmd)

# Run a command in the full Linux image (WPE + OP-TEE on PATH/env)
linux-full +cmd: (linux-run "full" cmd)

# Shared runner for the `linux` / `linux-full` recipes
#
# The target volume shadows the repo's own ./target inside the container rather
# than setting CARGO_TARGET_DIR: the melt-pot example crates are excluded from
# the workspace so they build into per-crate `target/` dirs, and the build
# pipeline looks for their ELFs at those exact paths. A global CARGO_TARGET_DIR
# redirects them and the pipeline fails to find its own output.
[private]
linux-run tag +cmd: linux-kernel
    container run --rm --dns {{ linux_dns }} \
      --cpus {{ linux_cpus }} --memory {{ linux_memory }} \
      -v {{ justfile_directory() }}:/workspace \
      -v consortium-target:/workspace/target \
      -v consortium-cargo:/cargo-registry \
      -v {{ linux_kernel }}:/root/linux \
      -e CARGO_HOME=/cargo-registry \
      -w /workspace {{ linux_image }}:{{ tag }} \
      bash -lc '{{ cmd }}'

# Drop into an interactive shell in the full Linux image
linux-shell: linux-kernel
    container run --rm -it --dns {{ linux_dns }} \
      --cpus {{ linux_cpus }} --memory {{ linux_memory }} \
      -v {{ justfile_directory() }}:/workspace \
      -v consortium-target:/workspace/target \
      -v consortium-cargo:/cargo-registry \
      -v {{ linux_kernel }}:/root/linux \
      -e CARGO_HOME=/cargo-registry \
      -w /workspace {{ linux_image }}:full bash

# Ensure the kernel checkout mounted at /root/linux exists (shallow clone if not)
[private]
linux-kernel:
    @[ -d "{{ linux_kernel }}" ] || git clone --depth 1 "{{ linux_kernel_url }}" "{{ linux_kernel }}"

# Remove the cached Linux build volumes
linux-clean:
    -container volume delete consortium-target
    -container volume delete consortium-cargo
    -rm -rf {{ linux_ctx }}

# ── Utilities ────────────────────────────────────────────────────────────────

# Add license headers (requires addlicense)
license:
    addlicense -c "Ethan Wu" -y 2026 -l apache -s -ignore "data/**/*" .

# ── PAC generation (consortium-pac-*) ────────────────────────────────────────
# Tooling for the peripheral access crates. Vendor headers live in `submodules/`,
# local patches in `scripts/pac/patches/`, generated IR under
# `crates/consortium-data/database/` (`chip/`, `peri/`). Requires
# chiptool (see `just setup-chiptool`).

# Initialize the PAC vendor submodules and apply local patches
pac-init:
    git submodule update --init submodules/imx-sm submodules/mcux-devices-imx submodules/STM32CubeMP2
    just pac-patch

# Initialize source submodules used directly by workspace crates
init:
    git submodule update --init submodules/embassy submodules/pocketjs

# Compatibility alias for the previous recipe name
ref-init: init

# Apply all patches to all PAC submodules
pac-patch:
    #!/usr/bin/env bash
    set -e
    PATCHES_DIR="$(pwd)/scripts/pac/patches"

    for submodule_patches_dir in "$PATCHES_DIR"/*/; do
        submodule=$(basename "$submodule_patches_dir")
        target="submodules/$submodule"

        if [ ! -d "$target" ]; then
            echo "Skipping $submodule (not cloned yet)"
            continue
        fi

        patches=("$submodule_patches_dir"*.diff)
        if [ ! -e "${patches[0]}" ]; then
            echo "No patches for $submodule"
            continue
        fi

        echo "🔧 Patching $submodule..."
        for patch in "${patches[@]}"; do
            if git -C "$target" apply --check --reverse --ignore-whitespace "$patch"; then
                echo "$(basename "$patch") already applied"
                continue
            fi
            git -C "$target" apply --check --ignore-whitespace "$patch"
            git -C "$target" apply --ignore-whitespace "$patch"
        done
        echo "$submodule patched"
    done

# Apply patches for a specific PAC submodule
pac-patch-one submodule:
    #!/usr/bin/env bash
    set -e
    patches=(scripts/pac/patches/{{submodule}}/*.diff)
    if [ ! -e "${patches[0]}" ]; then
        echo "No patches found for {{submodule}}"
        exit 0
    fi
    for patch in "${patches[@]}"; do
        if git -C submodules/{{submodule}} apply --check --reverse --ignore-whitespace "$patch"; then
            echo "$(basename "$patch") already applied"
            continue
        fi
        git -C submodules/{{submodule}} apply --check --ignore-whitespace "$patch"
        git -C submodules/{{submodule}} apply --ignore-whitespace "$patch"
    done

# Reset all PAC submodules to upstream HEAD and re-apply patches
pac-reset:
    #!/usr/bin/env bash
    set -e
    echo "Resetting all PAC submodules to upstream HEAD..."
    git submodule foreach --recursive 'git am --abort 2>/dev/null || true'
    git submodule foreach --recursive 'git rebase --abort 2>/dev/null || true'
    git submodule update --init --force submodules/imx-sm submodules/mcux-devices-imx submodules/STM32CubeMP2
    just pac-patch

# Reset a specific PAC submodule and re-apply its patches
pac-reset-one submodule:
    git -C submodules/{{submodule}} reset --hard
    git -C submodules/{{submodule}} clean -fd
    just pac-patch-one {{submodule}}

# Fetch a patch from a GitHub PR and save it under scripts/pac/patches
pac-fetch-patch submodule owner repo pr:
    mkdir -p scripts/pac/patches/{{submodule}}
    curl -L https://github.com/{{owner}}/{{repo}}/pull/{{pr}}.diff \
        -o scripts/pac/patches/{{submodule}}/pr{{pr}}.diff
    echo "Saved to scripts/pac/patches/{{submodule}}/pr{{pr}}.diff"

# Check that all PAC patches apply cleanly (dry run)
pac-check:
    #!/usr/bin/env bash
    set -e
    PATCHES_DIR="$(pwd)/scripts/pac/patches"

    for submodule_patches_dir in "$PATCHES_DIR"/*/; do
        submodule=$(basename "$submodule_patches_dir")
        target="submodules/$submodule"
        [ ! -d "$target" ] && continue

        patches=("$submodule_patches_dir"*.diff)
        [ ! -e "${patches[0]}" ] && continue

        echo "🔍 Checking $submodule..."
        git -C "$target" apply --check --ignore-whitespace "${patches[@]}" \
            && echo "$submodule OK" \
            || echo "$submodule FAILED"
    done

# Show PAC submodule status
pac-status:
    git submodule status submodules/imx-sm submodules/mcux-devices-imx submodules/STM32CubeMP2

# Stage vendor CMSIS headers into ./data for the parsers
pac-prepare:
    bash scripts/pac/prepare.sh

# Parse STM32MP CMSIS headers into chiptool IR
pac-parse-stm32:
    python scripts/pac/parse_stm32.py --all

# Parse NXP MCUX CMSIS headers into chiptool IR
pac-parse-nxp:
    python scripts/pac/parse_nxp.py --all

# Generate svd2rust-style interrupt tables from the chip databases
pac-gen-interrupts:
    python scripts/pac/gen_interrupts.py

# Generate the chip HALs' peripheral singletons from the chip databases
pac-gen-peripherals:
    python scripts/pac/gen_peripherals.py

# Regenerate the IWDG PAC after parsing the STM32 headers.
pac-gen-iwdg:
    chiptool gen-block --input crates/consortium-data/database/peri/stm32mp/iwdg_v1.yaml \
      --output crates/consortium-pac-stm32mp2/src/peripherals/iwdg.rs \
      --common-module crate::common --skip-no-std
    rustfmt --edition 2024 crates/consortium-pac-stm32mp2/src/peripherals/iwdg.rs

# Parse all CMSIS headers (STM32 + NXP)
pac-parse-all: pac-parse-stm32 pac-parse-nxp
    @echo "✓ All parsers completed"

# Run custom PAC output validation tests
pac-test:
    python scripts/pac/test_output.py

# Validate PAC output with chiptool
pac-validate:
    python scripts/pac/validate_with_chiptool.py

# Show PAC validation summary
pac-summary:
    bash scripts/pac/test_summary.sh

# Full PAC pipeline: parse, test, and validate
pac-ci: pac-parse-all pac-test pac-validate
    @echo ""
    @echo "=========================================="
    @echo "PAC Pipeline Complete"
    @echo "=========================================="
    bash scripts/pac/test_summary.sh
