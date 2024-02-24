#!/usr/bin/env bash

set -eou pipefail

ruff format --check .
ruff check .
mypy --ignore-missing-imports .
cargo fmt --check
cargo clippy -- -D warnings
