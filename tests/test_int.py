# SPDX-License-Identifier: (Apache-2.0 OR MIT)

import msgpack
import pytest

import ormsgpack


@pytest.mark.parametrize(
    "value",
    (
        pytest.param(1, id="positive fixint"),
        pytest.param(128, id="uint 8"),
        pytest.param(256, id="uint 16"),
        pytest.param(65536, id="uint 32"),
        pytest.param(4294967296, id="uint 64"),
        pytest.param(18446744073709551615, id="uint 64 max"),
        pytest.param(-1, id="negative fixint"),
        pytest.param(-128, id="int 8"),
        pytest.param(-256, id="int 16"),
        pytest.param(-65536, id="int 32"),
        pytest.param(-4294967296, id="int 64"),
        pytest.param(-9223372036854775808, id="int 64 min"),
    ),
)
def test_int_64(value: int) -> None:
    packed = ormsgpack.packb(value)
    assert packed == msgpack.packb(value)
    assert ormsgpack.unpackb(packed) == value


@pytest.mark.parametrize(
    "value",
    (
        -9223372036854775809,
        18446744073709551616,
    ),
)
def test_int_128(value: int) -> None:
    with pytest.raises(ormsgpack.MsgpackEncodeError):
        ormsgpack.packb(value)


@pytest.mark.parametrize(
    "value",
    (
        -9223372036854775807,
        9223372036854775807,
        18446744073709551615,
    ),
)
def test_int_64_passthrough(value: int) -> None:
    assert (
        ormsgpack.unpackb(
            ormsgpack.packb(value, option=ormsgpack.OPT_PASSTHROUGH_BIG_INT)
        )
        == value
    )


@pytest.mark.parametrize(
    "value",
    (
        -9223372036854775808,
        18446744073709551616,
    ),
)
def test_int_128_passthrough(value: int) -> None:
    result = ormsgpack.unpackb(
        ormsgpack.packb(
            value,
            option=ormsgpack.OPT_PASSTHROUGH_BIG_INT,
            default=lambda x: {"int": x.to_bytes(16, "little", signed=True)},
        )
    )
    assert list(result.keys()) == ["int"]
    assert int.from_bytes(result["int"], "little", signed=True) == value
