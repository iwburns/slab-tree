
@watch:
    cargo watch -x check -x "test --lib"

@watch-docs:
    cargo watch -x check -x "test --doc"

@check:
    cargo check

@doc:
    cargo doc

@test-lib:
    cargo test --lib

@test:
    cargo test

@lint:
    cargo clippy

@format:
    cargo fmt

@cover:
    cargo +nightly tarpaulin --verbose

@install-dev-deps:
    rustup install nightly
    rustup update nightly
    RUSTFLAGS="--cfg procmacro2_semver_exempt" cargo +nightly install cargo-tarpaulin
