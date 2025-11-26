RUSTFMT = cargo +nightly fmt

.PHONY: fmt fmt-check

release:
	cargo build --release

fmt:
	$(RUSTFMT)

fmt-check:
	$(RUSTFMT) -- --check

fix:
	cargo fix --allow-dirty