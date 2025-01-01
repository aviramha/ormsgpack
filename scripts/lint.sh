#!/usr/bin/env bash

set -eou pipefail

uv sync --frozen
uv run --no-sync ruff format --check .
uv run --no-sync ruff check .
uv run --no-sync mypy .
cargo fmt --check
cargo clippy -- -D warnings
