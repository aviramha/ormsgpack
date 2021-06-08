# SPDX-License-Identifier: (Apache-2.0 OR MIT)

import datetime
import inspect
import re

import msgpack
import pytest

import ormsgpack

SIMPLE_TYPES = (1, 1.0, -1, None, "str", True, False)


def default(obj):
    return str(obj)


def test_simple_msgpack():
    """
    packb() equivalent to msgpack on simple types
    """
    for obj in SIMPLE_TYPES:
        assert ormsgpack.packb(obj) == msgpack.packb(obj)


def test_simple_round_trip():
    """
    packb(), unpackb() round trip on simple types
    """
    for obj in SIMPLE_TYPES:
        assert ormsgpack.unpackb(ormsgpack.packb(obj)) == obj


def test_unpackb_type():
    """
    unpackb() invalid type
    """
    for val in (1, 3.14, [], {}, None):
        with pytest.raises(ormsgpack.MsgpackDecodeError):
            ormsgpack.unpackb(val)


@pytest.mark.skip(reason="https://github.com/3Hren/msgpack-rust/issues/276")
def test_loads_recursion():
    """
    unpackb() recursion limit
    """
    with pytest.raises(ormsgpack.MsgpackDecodeError):
        ormsgpack.unpackb(b"\x91" * (1024 * 1024))


def test_version():
    """
    __version__
    """
    assert re.match(r"^\d+\.\d+(\.\d+)?$", ormsgpack.__version__)


def test_valueerror():
    """
    ormsgpack.MsgpackDecodeError is a subclass of ValueError
    """
    with pytest.raises(ormsgpack.MsgpackDecodeError):
        ormsgpack.unpackb(b"\x91")
    with pytest.raises(ValueError):
        ormsgpack.unpackb(b"\x91")


def test_option_not_int():
    """
    packb() option not int or None
    """
    with pytest.raises(ormsgpack.MsgpackEncodeError):
        ormsgpack.packb(True, option=True)


def test_option_invalid_int():
    """
    packb() option invalid 64-bit number
    """
    with pytest.raises(ormsgpack.MsgpackEncodeError):
        ormsgpack.packb(True, option=9223372036854775809)


def test_option_range_low():
    """
    packb() option out of range low
    """
    with pytest.raises(ormsgpack.MsgpackEncodeError):
        ormsgpack.packb(True, option=-1)


def test_option_range_high():
    """
    packb() option out of range high
    """
    with pytest.raises(ormsgpack.MsgpackEncodeError):
        ormsgpack.packb(True, option=1 << 12)


def test_opts_multiple():
    """
    packb() multiple option
    """
    assert (
        ormsgpack.packb(
            [1, datetime.datetime(2000, 1, 1, 2, 3, 4)],
            option=ormsgpack.OPT_SERIALIZE_NUMPY | ormsgpack.OPT_NAIVE_UTC,
        )
        == msgpack.packb([1, "2000-01-01T02:03:04+00:00"])
    )


def test_default_positional():
    """
    packb() positional arg
    """
    with pytest.raises(TypeError):
        ormsgpack.packb(__obj={})
    with pytest.raises(TypeError):
        ormsgpack.packb(zxc={})


def test_default_unknown_kwarg():
    """
    packb() unknown kwarg
    """
    with pytest.raises(TypeError):
        ormsgpack.packb({}, zxc=default)


def test_default_empty_kwarg():
    """
    packb() empty kwarg
    """
    assert ormsgpack.packb(None, **{}) == b"\xc0"


def test_default_twice():
    """
    packb() default twice
    """
    with pytest.raises(TypeError):
        ormsgpack.packb({}, default, default=default)


def test_option_twice():
    """
    packb() option twice
    """
    with pytest.raises(TypeError):
        ormsgpack.packb(
            {}, None, ormsgpack.OPT_NAIVE_UTC, option=ormsgpack.OPT_NAIVE_UTC
        )


def test_option_mixed():
    """
    packb() option one arg, one kwarg
    """

    class Custom:
        def __str__(self):
            return "zxc"

    assert (
        ormsgpack.packb(
            [Custom(), datetime.datetime(2000, 1, 1, 2, 3, 4)],
            default=default,
            option=ormsgpack.OPT_NAIVE_UTC,
        )
        == msgpack.packb(["zxc", "2000-01-01T02:03:04+00:00"])
    )


def test_packb_signature():
    """
    packb() valid __text_signature__
    """
    assert (
        str(inspect.signature(ormsgpack.packb)) == "(obj, /, default=None, option=None)"
    )
    inspect.signature(ormsgpack.packb).bind("str")
    inspect.signature(ormsgpack.packb).bind("str", default=default, option=1)


def test_unpackb_signature():
    """
    unpackb() valid __text_signature__
    """
    assert str(inspect.signature(ormsgpack.unpackb)) == "(obj, /)"
    inspect.signature(ormsgpack.unpackb).bind("[]")


def test_packb_module_str():
    """
    ormsgpack.packb.__module__ is a str
    """
    assert ormsgpack.packb.__module__ == "ormsgpack"


def test_unpackb_module_str():
    """
    ormsgpack.unpackb.__module__ is a str
    """
    assert ormsgpack.unpackb.__module__ == "ormsgpack"


def test_bytes_buffer():
    """
    packb() trigger buffer growing where length is greater than growth
    """
    a = "a" * 900
    b = "b" * 4096
    c = "c" * 4096 * 4096
    assert ormsgpack.packb([a, b, c]) == msgpack.packb([a, b, c])