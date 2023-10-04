import datetime
import random
from time import mktime
from typing import Any

import msgpack

import ormsgpack

data = []
for year in range(1920, 2020):
    start = datetime.date(year, 1, 1)
    array: list[tuple[Any, int]] = [
        (int(mktime((start + datetime.timedelta(days=i)).timetuple())), i + 1)
        for i in range(0, 365)
    ]
    array.append(("other", 0))
    random.shuffle(array)
    data.append(dict(array))


def test_msgpack_packb(benchmark):
    benchmark.group = "non_str_keys"
    benchmark(msgpack.packb, data)


def test_ormsgpack_packb(benchmark):
    benchmark.group = "non_str_keys"
    benchmark(ormsgpack.packb, data, option=ormsgpack.OPT_NON_STR_KEYS)
