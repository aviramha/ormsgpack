# SPDX-License-Identifier: (Apache-2.0 OR MIT)

import msgpack
import pytest

import ormsgpack


@pytest.mark.parametrize(
    "value",
    (
        pytest.param(b"a" * 32, id="bin 8"),
        pytest.param(b"a" * 256, id="bin 16"),
        pytest.param(b"a" * 65536, id="bin 32"),
    ),
)
def test_bytes(value: bytes) -> None:
    packed = ormsgpack.packb(value)
    assert packed == msgpack.packb(value)
    assert ormsgpack.unpackb(packed) == value
