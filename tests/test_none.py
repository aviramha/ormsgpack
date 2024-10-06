# SPDX-License-Identifier: (Apache-2.0 OR MIT)

import msgpack

import ormsgpack


def test_none() -> None:
    value = None
    packed = ormsgpack.packb(value)
    assert packed == msgpack.packb(value)
    assert ormsgpack.unpackb(packed) == value
