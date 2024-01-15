# SPDX-License-Identifier: (Apache-2.0 OR MIT)

import ctypes
import datetime
import inspect
import re
import sys

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
        assert ormsgpack.packb(obj, option=None) == msgpack.packb(obj)


def test_simple_round_trip():
    """
    packb(), unpackb() round trip on simple types
    """
    for obj in SIMPLE_TYPES:
        assert ormsgpack.unpackb(ormsgpack.packb(obj, option=None), option=None) == obj


def test_unpackb_type():
    """
    unpackb() invalid type
    """
    for val in (1, 3.14, [], {}, None):
        with pytest.raises(ormsgpack.MsgpackDecodeError):
            ormsgpack.unpackb(val)


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
    packb/unpackb() option not int or None
    """
    with pytest.raises(ormsgpack.MsgpackEncodeError):
        ormsgpack.packb(True, option=True)
    with pytest.raises(ormsgpack.MsgpackDecodeError):
        ormsgpack.unpackb(b"\x00", option=True)


def test_option_invalid_int():
    """
    packb/unpackb() option invalid 64-bit number
    """
    with pytest.raises(ormsgpack.MsgpackEncodeError):
        ormsgpack.packb(True, option=9223372036854775809)
    with pytest.raises(ormsgpack.MsgpackDecodeError):
        ormsgpack.unpackb(b"\x00", option=9223372036854775809)


def test_option_range_low():
    """
    packb/unpackb() option out of range low
    """
    with pytest.raises(ormsgpack.MsgpackEncodeError):
        ormsgpack.packb(True, option=-1)
    with pytest.raises(ormsgpack.MsgpackDecodeError):
        ormsgpack.unpackb("\x00", option=-1)


def test_option_range_high():
    """
    packb/unpackb() option out of range high
    """
    with pytest.raises(ormsgpack.MsgpackEncodeError):
        ormsgpack.packb(True, option=1 << 14)
    with pytest.raises(ormsgpack.MsgpackDecodeError):
        ormsgpack.unpackb("\x00", option=1 << 14)


def test_opts_multiple():
    """
    packb() multiple option
    """
    assert ormsgpack.packb(
        [1, datetime.datetime(2000, 1, 1, 2, 3, 4)],
        option=ormsgpack.OPT_SERIALIZE_NUMPY | ormsgpack.OPT_NAIVE_UTC,
    ) == msgpack.packb([1, "2000-01-01T02:03:04+00:00"])


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
    with pytest.raises(ValueError):
        ormsgpack.unpackb("\x00", zxc=default)


def test_default_empty_kwarg():
    """
    unpackb/packb() empty kwarg
    """
    assert ormsgpack.packb(None, **{}) == b"\xc0"
    assert ormsgpack.unpackb(b"\xc0", **{}) is None


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

    assert ormsgpack.packb(
        [Custom(), datetime.datetime(2000, 1, 1, 2, 3, 4)],
        default=default,
        option=ormsgpack.OPT_NAIVE_UTC,
    ) == msgpack.packb(["zxc", "2000-01-01T02:03:04+00:00"])


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
    assert (
        str(inspect.signature(ormsgpack.unpackb))
        == "(obj, /, ext_hook=None, option=None)"
    )
    inspect.signature(ormsgpack.unpackb).bind("[]")


def test_packb_module_str():
    """
    ormsgpack.packb.__module__ is a str
    """
    assert ormsgpack.packb.__module__ == "ormsgpack.ormsgpack"


def test_unpackb_module_str():
    """
    ormsgpack.unpackb.__module__ is a str
    """
    assert ormsgpack.unpackb.__module__ == "ormsgpack.ormsgpack"


def test_bytes_buffer():
    """
    packb() trigger buffer growing where length is greater than growth
    """
    a = "a" * 900
    b = "b" * 4096
    c = "c" * 4096 * 4096
    assert ormsgpack.packb([a, b, c]) == msgpack.packb([a, b, c])


def test_function_flags():
    """
    Make sure we use fastcall when possible
    """
    FASTCALL = 0x0080
    KEYWORDS = 0x0002
    VARARGS = 0x0001
    ctypes.pythonapi.PyCFunction_GetFlags.argtypes = [ctypes.py_object]
    packb_flags = ctypes.pythonapi.PyCFunction_GetFlags(ormsgpack.packb)
    unpackb_flags = ctypes.pythonapi.PyCFunction_GetFlags(ormsgpack.unpackb)
    if sys.version_info.minor > 7:
        flags = FASTCALL | KEYWORDS
    else:
        flags = KEYWORDS | VARARGS
    assert packb_flags & flags == flags
    assert unpackb_flags & flags == flags
