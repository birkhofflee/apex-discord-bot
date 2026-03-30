default: check release

check:
    cargo check

release:
    cargo build --release
