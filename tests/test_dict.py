# SPDX-License-Identifier: (Apache-2.0 OR MIT)

import msgpack
import pytest

import ormsgpack


@pytest.mark.parametrize(
    "value",
    (
        pytest.param({"0": 0}, id="fixmap"),
        pytest.param({str(i): i for i in range(16)}, id="map 16"),
        pytest.param({str(i): i for i in range(65536)}, id="map 32"),
    ),
)
def test_dict(value: dict[str, int]) -> None:
    packed = ormsgpack.packb(value)
    assert packed == msgpack.packb(value)
    assert ormsgpack.unpackb(packed) == value


def test_dict_large() -> None:
    cache_size = 512
    obj = {str(i): i for i in range(cache_size + 1)}
    assert ormsgpack.unpackb(ormsgpack.packb(obj)) == obj


def test_dict_large_key() -> None:
    max_cached_key_len = 64
    obj = {"k" * max_cached_key_len * 2: "value"}
    assert ormsgpack.unpackb(ormsgpack.packb(obj)) == obj


def test_dict_unicode_key() -> None:
    obj = {"🚀": "value"}
    assert ormsgpack.unpackb(ormsgpack.packb(obj)) == obj


def test_dict_similar_keys() -> None:
    """
    unpackb() similar keys

    This was a regression in 3.4.2 caused by using
    the implementation in wy instead of wyhash.
    """
    obj = {"cf_status_firefox67": "---", "cf_status_firefox57": "verified"}
    assert ormsgpack.unpackb(ormsgpack.packb(obj)) == obj
