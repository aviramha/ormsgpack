# SPDX-License-Identifier: (Apache-2.0 OR MIT)

import ctypes
import datetime
import inspect
import re

import msgpack
import pytest

import ormsgpack

SIMPLE_TYPES = (1, 1.0, -1, None, "str", True, False)


def test_simple_msgpack() -> None:
    """
    packb() equivalent to msgpack on simple types
    """
    for obj in SIMPLE_TYPES:
        assert ormsgpack.packb(obj, option=None) == msgpack.packb(obj)


def test_simple_round_trip() -> None:
    """
    packb(), unpackb() round trip on simple types
    """
    for obj in SIMPLE_TYPES:
        assert ormsgpack.unpackb(ormsgpack.packb(obj, option=None), option=None) == obj


def test_unpackb_invalid_type() -> None:
    """
    unpackb() invalid type
    """
    val: object
    for val in (1, 3.14, [], {}, None):
        with pytest.raises(ormsgpack.MsgpackDecodeError):
            ormsgpack.unpackb(val)  # type: ignore[arg-type]


def test_unpackb_bytes() -> None:
    assert ormsgpack.unpackb(b"\x90") == []


def test_unpackb_bytearray() -> None:
    assert ormsgpack.unpackb(bytearray(b"\x90")) == []


def test_unpackb_memoryview() -> None:
    assert ormsgpack.unpackb(memoryview(b"\x90")) == []


def test_unpackb_invalid_data() -> None:
    for val in (b"\xd9\x97#DL_", b"\xc1", b"\x91\xc1"):
        with pytest.raises(ormsgpack.MsgpackDecodeError):
            ormsgpack.unpackb(val)


def test_unpackb_recursion() -> None:
    """
    unpackb() recursion limit
    """
    with pytest.raises(ormsgpack.MsgpackDecodeError):
        ormsgpack.unpackb(b"\x91" * (1024 * 1024))


def test_version() -> None:
    """
    __version__
    """
    assert re.match(r"^\d+\.\d+(\.\d+)?$", ormsgpack.__version__)


def test_valueerror() -> None:
    """
    ormsgpack.MsgpackDecodeError is a subclass of ValueError
    """
    with pytest.raises(ormsgpack.MsgpackDecodeError):
        ormsgpack.unpackb(b"\x91")
    with pytest.raises(ValueError):
        ormsgpack.unpackb(b"\x91")


@pytest.mark.parametrize(
    "option",
    (
        1 << 13,
        True,
        -1,
        9223372036854775809,
    ),
)
def test_packb_invalid_option(option: int) -> None:
    with pytest.raises(ormsgpack.MsgpackEncodeError):
        ormsgpack.packb(True, option=option)


@pytest.mark.parametrize(
    "option",
    (
        ormsgpack.OPT_NAIVE_UTC,
        ormsgpack.OPT_OMIT_MICROSECONDS,
        ormsgpack.OPT_PASSTHROUGH_BIG_INT,
        ormsgpack.OPT_PASSTHROUGH_DATACLASS,
        ormsgpack.OPT_PASSTHROUGH_DATETIME,
        ormsgpack.OPT_PASSTHROUGH_SUBCLASS,
        ormsgpack.OPT_PASSTHROUGH_TUPLE,
        ormsgpack.OPT_SERIALIZE_NUMPY,
        ormsgpack.OPT_SERIALIZE_PYDANTIC,
        ormsgpack.OPT_SORT_KEYS,
        ormsgpack.OPT_UTC_Z,
        True,
        -1,
        9223372036854775809,
    ),
)
def test_unpackb_invalid_option(option: int) -> None:
    with pytest.raises(ormsgpack.MsgpackDecodeError):
        ormsgpack.unpackb(b"\x00", option=option)


def test_opts_multiple() -> None:
    """
    packb() multiple option
    """
    assert ormsgpack.packb(
        [1, datetime.datetime(2000, 1, 1, 2, 3, 4)],
        option=ormsgpack.OPT_SERIALIZE_NUMPY | ormsgpack.OPT_NAIVE_UTC,
    ) == msgpack.packb([1, "2000-01-01T02:03:04+00:00"])


def test_default_positional() -> None:
    """
    packb() positional arg
    """
    with pytest.raises(TypeError):
        ormsgpack.packb(__obj={})  # type: ignore[call-arg]
    with pytest.raises(TypeError):
        ormsgpack.packb(zxc={})  # type: ignore[call-arg]


def test_default_unknown_kwarg() -> None:
    """
    packb() unknown kwarg
    """
    with pytest.raises(TypeError):
        ormsgpack.packb({}, zxc={})  # type: ignore[call-arg]
    with pytest.raises(ValueError):
        ormsgpack.unpackb(b"\x00", zxc={})  # type: ignore[call-arg]


def test_default_empty_kwarg() -> None:
    """
    unpackb/packb() empty kwarg
    """
    assert ormsgpack.packb(None, **{}) == b"\xc0"
    assert ormsgpack.unpackb(b"\xc0", **{}) is None


def test_default_twice() -> None:
    """
    packb() default twice
    """
    with pytest.raises(TypeError):
        ormsgpack.packb({}, None, default=None)  # type: ignore[misc]


def test_option_twice() -> None:
    """
    packb() option twice
    """
    with pytest.raises(TypeError):
        ormsgpack.packb(
            {},
            None,
            ormsgpack.OPT_NAIVE_UTC,
            option=ormsgpack.OPT_NAIVE_UTC,  # type: ignore[misc]
        )


def test_option_mixed() -> None:
    """
    packb() option one arg, one kwarg
    """

    assert ormsgpack.packb(
        [{1, 2}, datetime.datetime(2000, 1, 1, 2, 3, 4)],
        default=lambda x: str(x),
        option=ormsgpack.OPT_NAIVE_UTC,
    ) == msgpack.packb(["{1, 2}", "2000-01-01T02:03:04+00:00"])


def test_packb_signature() -> None:
    """
    packb() valid __text_signature__
    """
    assert (
        str(inspect.signature(ormsgpack.packb)) == "(obj, /, default=None, option=None)"
    )
    inspect.signature(ormsgpack.packb).bind("str")
    inspect.signature(ormsgpack.packb).bind("str", default=None, option=1)


def test_unpackb_signature() -> None:
    """
    unpackb() valid __text_signature__
    """
    assert (
        str(inspect.signature(ormsgpack.unpackb))
        == "(obj, /, ext_hook=None, option=None)"
    )
    inspect.signature(ormsgpack.unpackb).bind("[]")


def test_packb_module_str() -> None:
    """
    ormsgpack.packb.__module__ is a str
    """
    assert ormsgpack.packb.__module__ == "ormsgpack.ormsgpack"


def test_unpackb_module_str() -> None:
    """
    ormsgpack.unpackb.__module__ is a str
    """
    assert ormsgpack.unpackb.__module__ == "ormsgpack.ormsgpack"


def test_bytes_buffer() -> None:
    """
    packb() trigger buffer growing where length is greater than growth
    """
    a = "a" * 900
    b = "b" * 4096
    c = "c" * 4096 * 4096
    assert ormsgpack.packb([a, b, c]) == msgpack.packb([a, b, c])


def test_function_flags() -> None:
    """
    Make sure we use fastcall
    """
    FASTCALL = 0x0080
    KEYWORDS = 0x0002
    ctypes.pythonapi.PyCFunction_GetFlags.argtypes = [ctypes.py_object]
    packb_flags = ctypes.pythonapi.PyCFunction_GetFlags(ormsgpack.packb)
    unpackb_flags = ctypes.pythonapi.PyCFunction_GetFlags(ormsgpack.unpackb)
    flags = FASTCALL | KEYWORDS
    assert packb_flags & flags == flags
    assert unpackb_flags & flags == flags
