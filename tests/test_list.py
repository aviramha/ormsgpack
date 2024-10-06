# SPDX-License-Identifier: (Apache-2.0 OR MIT)

from typing import List

import msgpack
import pytest

import ormsgpack


@pytest.mark.parametrize(
    "value",
    (
        pytest.param([0], id="fixarray"),
        pytest.param([i for i in range(16)], id="array 16"),
        pytest.param([i for i in range(65536)], id="array 32"),
    ),
)
def test_list(value: List[int]) -> None:
    packed = ormsgpack.packb(value)
    assert packed == msgpack.packb(value)
    assert ormsgpack.unpackb(packed) == value
