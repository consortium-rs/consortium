a_target := "aarch64-unknown-linux-gnu"
m_target  := "thumbv7em-none-eabihf"

# List available recipes
default:
    @just --list

# ── Host (dev / CI) ──────────────────────────────────────────────────────────

# Check all crates on the host (std feature for consortium-core)
check:
    cargo check --workspace --features consortium-core/std

# Run all host-side tests
test:
    cargo test --workspace --features consortium-core/std

# Clippy across the workspace (deny warnings)
clippy:
    cargo clippy --workspace --features consortium-core/std -- -D warnings

# ── A-core (aarch64-unknown-linux-gnu) ───────────────────────────────────────

# Check consortium-uio for the A-core target
check-uio:
    cargo check -p consortium-uio --target {{ a_target }}

# Build consortium-uio for the A-core target
build-uio:
    cargo build -p consortium-uio --release --target {{ a_target }}

# ── M-core (thumbv7em-none-eabihf) ───────────────────────────────────────────

# Check consortium-core for the M-core target (no_std)
check-m:
    cargo check -p consortium-core --target {{ m_target }}

# Build consortium-core for the M-core target
build-m:
    cargo build -p consortium-core --release --target {{ m_target }}

# ── Combined ─────────────────────────────────────────────────────────────────

# Check all targets (requires cross-toolchains in PATH)
check-all: check check-uio check-m

# Build all release artefacts
build-all: build-uio build-m
