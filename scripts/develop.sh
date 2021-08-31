#!/bin/sh -e

rm -f target/wheels/*

export RUSTFLAGS="-C target-cpu=k8"

maturin build --no-sdist --manylinux off -i python3 --release "$@"

pip install --force $(find target/wheels -name "*cp3*")
