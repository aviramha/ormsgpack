# SPDX-License-Identifier: (Apache-2.0 OR MIT)

import ormsgpack


def test_sort_keys():
    obj = {"b": 1, "c": 2, "a": 3, "ä": 4, "A": 5}
    packed = ormsgpack.packb(obj, option=ormsgpack.OPT_SORT_KEYS)
    unpacked = ormsgpack.unpackb(packed)
    assert list(unpacked.keys()) == sorted(obj.keys())
