# Run all CI checks (same as GitHub Actions!)
# This is what developers should run before pushing
ci: fmt-check lint test build
    @echo "Safe to push to GitHub - CI will pass."

# Format all code
fmt:
    cargo fmt --all

# Check formatting without modifying files
fmt-check:
    cargo fmt --all -- --check

# Lint with clippy (warnings are errors)
lint:
    cargo clippy --locked --workspace --all-targets -- -D warnings

# Run the test suite
test:
    cargo test --locked --workspace --all-targets

# Build in release mode
build:
    cargo build --locked --release

# Install the seqlings binary locally (~/.cargo/bin)
install:
    cargo install --locked --path .

# Remove build artifacts (cargo target/ and mdbook book/)
clean:
    cargo clean
    rm -rf book

# Regenerate docs/README.md from source
gen-docs:
    ./scripts/generate-docs.sh

# Build the mdbook site into ./book/
build-docs: gen-docs
    mdbook build

# Serve mdbook locally with live reload
docs:
    just gen-docs
    mdbook serve --open
