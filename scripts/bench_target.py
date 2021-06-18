#!/usr/bin/env python3
# SPDX-License-Identifier: (Apache-2.0 OR MIT)
"""
This allows gathering profiling data from running packb or unpackb
in a loop on fixtures from files containing messagepack data. Example usage:

./scripts/bench_target.py benchmarks/samples/citm_catalog.mpack packb
./scripts/bench_target.py benchmarks/samples/citm_catalog.mpack packb 5000
"""

import os
import sys

os.sched_setaffinity(os.getpid(), {0, 1})

from ormsgpack import packb, unpackb

filename = sys.argv[1]
n = int(sys.argv[3]) if len(sys.argv) >= 4 else 5000

with open(filename, "rb") as fileh:
    file_bytes = fileh.read()

if sys.argv[2] == "packb":
    file_obj = unpackb(file_bytes)
    for _ in range(n):
        packb(file_obj)
elif sys.argv[2] == "unpackb":
    for _ in range(n):
        unpackb(file_bytes)
