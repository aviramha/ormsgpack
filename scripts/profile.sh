#!/bin/sh -e
# Works only on Linux currently.
# usage: ./scripts/profile.sh benchmarks/samples/citm_catalog.mpack packb

perf record -g --delay 250 ./scripts/bench_target.py "$@"
perf report --percent-limit 0.1
rm -f perf.data*
