# SPDX-License-Identifier: (Apache-2.0 OR MIT)

import datetime
import enum

import msgpack
import pytest

import ormsgpack


class StrEnum(str, enum.Enum):
    AAA = "aaa"


class IntEnum(int, enum.Enum):
    ONE = 1


class IntEnumEnum(enum.IntEnum):
    ONE = 1


class IntFlagEnum(enum.IntFlag):
    ONE = 1


class FlagEnum(enum.Flag):
    ONE = 1


class AutoEnum(enum.auto):
    A = "a"


class FloatEnum(float, enum.Enum):
    ONE = 1.1


class Custom:
    def __init__(self, val):
        self.val = val


def default(obj):
    if isinstance(obj, Custom):
        return obj.val
    raise TypeError


class UnspecifiedEnum(enum.Enum):
    A = "a"
    B = 1
    C = FloatEnum.ONE
    D = {"d": IntEnum.ONE}  # noqa: RUF012
    E = Custom("c")
    F = datetime.datetime(1970, 1, 1)


def test_cannot_subclass():
    """
    enum.Enum cannot be subclassed

    obj->ob_type->ob_base will always be enum.EnumMeta
    """
    with pytest.raises(TypeError):

        class Subclass(StrEnum):  # type: ignore
            B = "b"


def test_arbitrary_enum():
    assert ormsgpack.packb(UnspecifiedEnum.A) == msgpack.packb("a")
    assert ormsgpack.packb(UnspecifiedEnum.B) == msgpack.packb(1)
    assert ormsgpack.packb(UnspecifiedEnum.C) == msgpack.packb(1.1)
    assert ormsgpack.packb(UnspecifiedEnum.D) == msgpack.packb({"d": 1})


def test_custom_enum():
    assert ormsgpack.packb(UnspecifiedEnum.E, default=default) == msgpack.packb("c")


def test_enum_options():
    assert ormsgpack.packb(
        UnspecifiedEnum.F, option=ormsgpack.OPT_NAIVE_UTC
    ) == msgpack.packb("1970-01-01T00:00:00+00:00")


def test_int_enum():
    assert ormsgpack.packb(IntEnum.ONE) == msgpack.packb(1)


def test_intenum_enum():
    assert ormsgpack.packb(IntEnumEnum.ONE) == msgpack.packb(1)


def test_intflag_enum():
    assert ormsgpack.packb(IntFlagEnum.ONE) == msgpack.packb(1)


def test_flag_enum():
    assert ormsgpack.packb(FlagEnum.ONE) == msgpack.packb(1)


def test_auto_enum():
    assert ormsgpack.packb(AutoEnum.A) == msgpack.packb("a")


def test_float_enum():
    assert ormsgpack.packb(FloatEnum.ONE) == msgpack.packb(1.1)


def test_str_enum():
    assert ormsgpack.packb(StrEnum.AAA) == msgpack.packb("aaa")


def test_bool_enum():
    with pytest.raises(TypeError):

        class BoolEnum(bool, enum.Enum):  # type: ignore
            TRUE = True


def test_non_str_keys_enum():
    assert ormsgpack.packb(
        {StrEnum.AAA: 1}, option=ormsgpack.OPT_NON_STR_KEYS
    ) == msgpack.packb({"aaa": 1})
    assert ormsgpack.packb(
        {IntEnum.ONE: 1}, option=ormsgpack.OPT_NON_STR_KEYS
    ) == msgpack.packb({1: 1})
