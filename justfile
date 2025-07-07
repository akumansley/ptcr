# See https://github.com/casey/just

# run tests

test:
    cargo test

# run clippy lints
clippy:
    cargo clippy -- -D warnings

# build the project
build:
    cargo build

# check formatting
lint:
    cargo fmt --all -- --check

# format source code
format:
    cargo fmt --all

# run all checks
verify: format lint clippy build test
