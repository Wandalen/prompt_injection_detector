# Makefile for vllm_inferencer
#
# Test levels following workspace standards

.PHONY: ctest1 ctest2 ctest3 doc clean run

# Level 1: Basic tests
ctest1:
	clear && RUSTFLAGS="-D warnings" cargo nextest run --all-features

# Level 2: Tests + doc tests
ctest2:
	clear && RUSTFLAGS="-D warnings" cargo nextest run --all-features && \
	RUSTDOCFLAGS="-D warnings" cargo test --doc --all-features

# Level 3: Tests + docs + clippy (default)
ctest3:
	clear && RUSTFLAGS="-D warnings" cargo nextest run --all-features && \
	RUSTDOCFLAGS="-D warnings" cargo test --doc --all-features && \
	cargo clippy --all-targets --all-features -- -D warnings

# Default test level
test: ctest3

# Generate documentation
doc:
	cargo doc --all-features --no-deps --open

# Clean build artifacts
clean:
	cargo clean

# Run CLI mode (Milestone 1)
run-cli:
	cargo run --release -- --prompt "Once upon a time" --max-tokens 50

# Run HTTP server mode (Milestone 2+)
run-server:
	cargo run --release --features http_server

# Help
help:
	@echo "Available targets:"
	@echo "  ctest1      - Run basic tests"
	@echo "  ctest2      - Run tests + doc tests"
	@echo "  ctest3      - Run tests + docs + clippy (default)"
	@echo "  test        - Alias for ctest3"
	@echo "  doc         - Generate and open documentation"
	@echo "  run-cli     - Run CLI mode example"
	@echo "  run-server  - Run HTTP server mode (M2+)"
	@echo "  clean       - Clean build artifacts"
