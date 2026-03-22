m8_target := "thumbv8m.main-none-eabihf"
m7_target := "thumbv7em-none-eabihf"

# List available recipes
default:
    @just --list

# ── Host (dev / CI) ──────────────────────────────────────────────────────────

# Check all crates on the host
check:
    cargo check --workspace --features consortium-core/std

# Run all host-side tests
test:
    cargo test --workspace --features consortium-core/std

# Clippy across the workspace (deny warnings)
clippy:
    cargo clippy --workspace --features consortium-core/std -- -D warnings

# Check consortium-uio on the host (cross-compilation is handled by Yocto)
check-uio:
    cargo check -p consortium-uio

# ── M-core (thumbv8) ─────────────────────────────────────────────────────────

check-m8:
    cargo check -p consortium-core --target {{ m8_target }}
    cargo check -p consortium-mem --target {{ m8_target }}

build-m8:
    cargo build -p consortium-core --release --target {{ m8_target }}
    cargo build -p consortium-mem --release --target {{ m8_target }}

# ── M-core (thumbv7) ─────────────────────────────────────────────────────────

check-m7:
    cargo check -p consortium-core --target {{ m7_target }}
    cargo check -p consortium-mem --target {{ m7_target }}

build-m7:
    cargo build -p consortium-core --release --target {{ m7_target }}
    cargo build -p consortium-mem --release --target {{ m7_target }}

# ── Combined ─────────────────────────────────────────────────────────────────

check-m: check-m8 check-m7

build-m: build-m8 build-m7

check-all: check check-uio check-m
