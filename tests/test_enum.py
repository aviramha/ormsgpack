# SPDX-License-Identifier: (Apache-2.0 OR MIT)

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


class FloatEnum(float, enum.Enum):
    ONE = 1.1


def test_cannot_subclass() -> None:
    """
    enum.Enum cannot be subclassed

    obj->ob_type->ob_base will always be enum.EnumMeta
    """
    with pytest.raises(TypeError):

        class Subclass(StrEnum):  # type: ignore
            B = "b"


def test_int_enum() -> None:
    assert ormsgpack.packb(IntEnum.ONE) == msgpack.packb(1)


def test_intenum_enum() -> None:
    assert ormsgpack.packb(IntEnumEnum.ONE) == msgpack.packb(1)


def test_intflag_enum() -> None:
    assert ormsgpack.packb(IntFlagEnum.ONE) == msgpack.packb(1)


def test_flag_enum() -> None:
    assert ormsgpack.packb(FlagEnum.ONE) == msgpack.packb(1)


def test_float_enum() -> None:
    assert ormsgpack.packb(FloatEnum.ONE) == msgpack.packb(1.1)


def test_str_enum() -> None:
    assert ormsgpack.packb(StrEnum.AAA) == msgpack.packb("aaa")


def test_bool_enum() -> None:
    with pytest.raises(TypeError):

        class BoolEnum(bool, enum.Enum):  # type: ignore
            TRUE = True


@pytest.mark.parametrize(
    "value",
    (
        FlagEnum.ONE,
        FloatEnum.ONE,
        IntEnum.ONE,
        IntEnumEnum.ONE,
        IntFlagEnum.ONE,
        StrEnum.AAA,
    ),
)
def test_enum_passthrough(value: enum.Enum) -> None:
    with pytest.raises(ormsgpack.MsgpackEncodeError):
        assert ormsgpack.packb(value, option=ormsgpack.OPT_PASSTHROUGH_ENUM)


@pytest.mark.parametrize(
    "value",
    (
        FlagEnum.ONE,
        FloatEnum.ONE,
        IntEnum.ONE,
        IntEnumEnum.ONE,
        IntFlagEnum.ONE,
        StrEnum.AAA,
    ),
)
def test_enum_passthrough_default(value: enum.Enum) -> None:
    assert ormsgpack.packb(
        value, option=ormsgpack.OPT_PASSTHROUGH_ENUM, default=str
    ) == msgpack.packb(str(value))
