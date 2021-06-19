#!/usr/bin/env bash

set -eou pipefail

autoflake --in-place --recursive --remove-all-unused-imports --ignore-init-module-imports .
isort ./ormsgpack.pyi ./tests/*.py ./benchmarks/*.py ./scripts/*.py
black ./ormsgpack.pyi ./tests/*.py ./benchmarks/*.py ./scripts/*.py
mypy --ignore-missing-imports ./ormsgpack.pyi ./tests/*.py ./benchmarks/*.py ./scripts/*.py
cargo fmt
