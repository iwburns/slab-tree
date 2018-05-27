
@watch:
    cargo watch -x check -x "test --lib"

@check:
    cargo check

@test-lib:
    cargo test --lib

@test-all:
    cargo test

@lint:
    cargo +nightly clippy

@format:
    cargo +nightly fmt

@install-dev-deps:
    rustup install nightly
    rustup update nightly
    rustup component add rustfmt-preview --toolchain nightly
    cargo +nightly install --force clippy
