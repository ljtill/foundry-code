build *args="":
    cargo build {{args}}

run:
    cargo run

test *args="":
    cargo test {{args}}

check:
    cargo check

format *args="":
    cargo fmt {{args}}

lint:
    cargo clippy
