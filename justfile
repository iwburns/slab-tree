
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
    cargo +nightly clippy

@format:
    cargo +nightly fmt

@cover:
    cargo +nightly tarpaulin --verbose

@install-dev-deps:
    rustup install nightly
    rustup update nightly
    rustup component add rustfmt-preview --toolchain nightly
    rustup component add clippy-preview --toolchain=nightly
    RUSTFLAGS="--cfg procmacro2_semver_exempt" cargo +nightly install cargo-tarpaulin
