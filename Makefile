all: check test

check:
	cargo fmt -- --check && cargo clippy -- -D warnings

test:
	cargo test

doc:
	cargo doc --open