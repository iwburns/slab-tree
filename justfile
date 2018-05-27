
@watch:
    cargo watch -x check -x "test --lib"

@lint:
    cargo +nightly clippy

@install-dev-deps:
    rustup install nightly
    rustup update nightly
    cargo +nightly install --force clippy
