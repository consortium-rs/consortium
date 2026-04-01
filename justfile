m8_target := "thumbv8m.main-none-eabihf"
m7_target := "thumbv7em-none-eabihf"
a8_target := "aarch64-unknown-linux-gnu"
a7_target := "armv7-unknown-linux-gnueabihf"
rvimc_target := "riscv32imc-unknown-none-elf"
rvimac_target := "riscv32imac-unknown-none-elf"
rvimafcp_target := "riscv32imafc-unknown-none-elf"

# List available recipes
default:
    @just --list

# ── Host (dev / CI) ──────────────────────────────────────────────────────────

# Check all crates on the host
check:
    cargo check --workspace --features consortium-ipc/std

# Run all host-side tests
test:
    cargo test --workspace --features consortium-ipc/std

# Clippy across the workspace (deny warnings)
clippy:
    cargo clippy --workspace --features consortium-ipc/std -- -D warnings

# Check consortium-uio on the host (cross-compilation is handled by Yocto)
check-uio:
    cargo check -p consortium-uio

# ── A-core (aarch64) ─────────────────────────────────────────────────────────

check-a8:
    cargo check -p consortium-ipc --target {{ a8_target }}
    cargo check -p consortium-uio --target {{ a8_target }}

build-a8:
    cargo build -p consortium-ipc --release --target {{ a8_target }}
    cargo build -p consortium-uio --release --target {{ a8_target }}

# ── A-core (armv7) ───────────────────────────────────────────────────────────

check-a7:
    cargo check -p consortium-ipc --target {{ a7_target }}
    cargo check -p consortium-uio --target {{ a7_target }}

build-a7:
    cargo build -p consortium-ipc --release --target {{ a7_target }}
    cargo build -p consortium-uio --release --target {{ a7_target }}

# ── M-core (thumbv8) ─────────────────────────────────────────────────────────

check-m8:
    cargo check -p consortium-ipc --target {{ m8_target }}
    cargo check -p consortium-mem --target {{ m8_target }}

build-m8:
    cargo build -p consortium-ipc --release --target {{ m8_target }}
    cargo build -p consortium-mem --release --target {{ m8_target }}

# ── M-core (thumbv7) ─────────────────────────────────────────────────────────

check-m7:
    cargo check -p consortium-ipc --target {{ m7_target }}
    cargo check -p consortium-mem --target {{ m7_target }}

build-m7:
    cargo build -p consortium-ipc --release --target {{ m7_target }}
    cargo build -p consortium-mem --release --target {{ m7_target }}

# ── RISC-V (rv32imc) ─────────────────────────────────────────────────────────

check-rv32imc:
    cargo check -p consortium-ipc --target {{ rvimc_target }}

# ── RISC-V (rv32imac) ────────────────────────────────────────────────────────

check-rv32imac:
    cargo check -p consortium-ipc --target {{ rvimac_target }}

# ── RISC-V (rv32imafcp) ───────────────────────────────────────────────────────

check-rv32imafcp:
    cargo check -p consortium-ipc --target {{ rvimafcp_target }}

# ── Combined ─────────────────────────────────────────────────────────────────

check-m: check-m8 check-m7

build-m: build-m8 build-m7

check-a: check-a8 check-a7

build-a: build-a8 build-a7

check-arm: check check-a check-m

check-rv: check-rv32imc check-rv32imac check-rv32imafcp