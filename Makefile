.PHONY: install
install:
	cargo install cargo-tarpaulin

.PHONY: lint
lint:
	cargo +nightly fmt --check
	cargo check --all-features --tests
	cargo clippy --tests --all-features  -- -D warnings

.PHONY: tarpaulin
tarpaulin:
	cargo tarpaulin -o html --engine llvm --output-dir ./coverage
	open coverage/tarpaulin-report.html