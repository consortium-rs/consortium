m8_target := "thumbv8m.main-none-eabihf"
m7_target := "thumbv7em-none-eabihf"
a64_target := "aarch64-unknown-linux-gnu"
a32_target := "armv7-unknown-linux-gnueabihf"
rv32imc_target := "riscv32imc-unknown-none-elf"
rv32imac_target := "riscv32imac-unknown-none-elf"
rv32imafcp_target := "riscv32imafc-unknown-none-elf"

# List available recipes
default:
    @just --list

# ── Core Commands (from CLAUDE.md) ───────────────────────────────────────────

# Build all crates
build:
    cargo build

# Run all tests
test:
    cargo test

# Run tests for a specific crate (e.g. `just test-crate consortium-ipc`)
test-crate crate:
    cargo test -p {{ crate }}

# Run a single test by name (e.g. `just test-fn consortium-ipc my_test`)
test-fn crate test_name:
    cargo test -p {{ crate }} {{ test_name }}

# Lint with Clippy
clippy:
    cargo clippy

# Format code
fmt:
    cargo fmt

# ── Host (dev / CI) ──────────────────────────────────────────────────────────

# Check all crates on the host
check:
    cargo check --workspace --features consortium-ipc/std

# ── A-core (aarch64 / Linux) ─────────────────────────────────────────────────

check-ipc-a64:
    @[ "$(uname)" = "Linux" ] || { echo "Error: A-core targets require Linux"; exit 1; }
    cargo check -p consortium-ipc --target {{ a64_target }} -F std
    cargo check -p consortium-ipc-transport-uio --target {{ a64_target }}
    cargo check -p consortium-runtime-app --target {{ a64_target }}
    cargo check -p consortium-ipc-doorbell-hsem --target {{ a64_target }} -F std

build-ipc-a64:
    @[ "$(uname)" = "Linux" ] || { echo "Error: A-core targets require Linux"; exit 1; }
    cargo build -p consortium-ipc --release --target {{ a64_target }} -F std
    cargo build -p consortium-ipc-transport-uio --release --target {{ a64_target }}
    cargo build -p consortium-runtime-app --release --target {{ a64_target }}
    cargo build -p consortium-ipc-doorbell-hsem --release --target {{ a64_target }} -F std

# ── A-core (armv7 / Linux) ──────────────────────────────────────────────────

check-ipc-a32:
    @[ "$(uname)" = "Linux" ] || { echo "Error: A-core targets require Linux"; exit 1; }
    cargo check -p consortium-ipc --target {{ a32_target }} -F std
    cargo check -p consortium-ipc-transport-uio --target {{ a32_target }}
    cargo check -p consortium-runtime-app --target {{ a32_target }}
    cargo check -p consortium-ipc-doorbell-hsem --target {{ a32_target }} -F std

build-ipc-a32:
    @[ "$(uname)" = "Linux" ] || { echo "Error: A-core targets require Linux"; exit 1; }
    cargo build -p consortium-ipc --release --target {{ a32_target }} -F std
    cargo build -p consortium-ipc-transport-uio --release --target {{ a32_target }}
    cargo build -p consortium-runtime-app --release --target {{ a32_target }}
    cargo build -p consortium-ipc-doorbell-hsem --release --target {{ a32_target }} -F std

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

# Run TEE tests
test-tee:
    @[ "$(uname)" = "Linux" ] || { echo "Error: TEE tests require Linux"; exit 1; }
    cargo test -p consortium-tee

# ── M-core (thumbv8 / bare-metal) ───────────────────────────────────────────

check-ipc-m8:
    cargo check -p consortium-ipc --target {{ m8_target }}
    cargo check -p consortium-common --target {{ m8_target }}
    cargo check -p consortium-ipc-transport-mem --target {{ m8_target }}
    cargo check -p consortium-runtime-mcu --target {{ m8_target }}
    cargo check -p consortium-ipc-doorbell-hsem --target {{ m8_target }}

build-ipc-m8:
    cargo build -p consortium-common --release --target {{ m8_target }}
    cargo build -p consortium-ipc --release --target {{ m8_target }}
    cargo build -p consortium-ipc-transport-mem --release --target {{ m8_target }}
    cargo build -p consortium-runtime-mcu --release --target {{ m8_target }}
    cargo build -p consortium-ipc-doorbell-hsem --release --target {{ m8_target }}

# ── M-core (thumbv7 / bare-metal) ───────────────────────────────────────────

check-ipc-m7:
    cargo check -p consortium-common --target {{ m7_target }}
    cargo check -p consortium-ipc --target {{ m7_target }}
    cargo check -p consortium-ipc-transport-mem --target {{ m7_target }}
    cargo check -p consortium-runtime-mcu --target {{ m7_target }}
    cargo check -p consortium-ipc-doorbell-hsem --target {{ m7_target }}

build-ipc-m7:
    cargo build -p consortium-common --release --target {{ m7_target }}
    cargo build -p consortium-ipc --release --target {{ m7_target }}
    cargo build -p consortium-ipc-transport-mem --release --target {{ m7_target }}
    cargo build -p consortium-runtime-mcu --release --target {{ m7_target }}
    cargo build -p consortium-ipc-doorbell-hsem --release --target {{ m8_target }}

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

# ── Utilities ────────────────────────────────────────────────────────────────

# Add license headers (requires addlicense)
license:
    addlicense -c "Ethan Wu" -y 2026 -l apache -s -ignore "data/**/*" .
