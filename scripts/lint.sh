#!/usr/bin/env bash

set -eou pipefail

autoflake --check --recursive --remove-all-unused-imports --ignore-init-module-imports .
isort --check .
black --check .
mypy --ignore-missing-imports .
cargo fmt --check
cargo clippy -- -D warnings
