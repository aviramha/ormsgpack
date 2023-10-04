#!/usr/bin/env bash

set -eou pipefail

autoflake --in-place --recursive --remove-all-unused-imports --ignore-init-module-imports .
isort .
black .
mypy --ignore-missing-imports .
cargo fmt
