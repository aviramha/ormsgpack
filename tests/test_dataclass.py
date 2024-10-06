# SPDX-License-Identifier: (Apache-2.0 OR MIT)
from dataclasses import InitVar, asdict, dataclass, field
from typing import ClassVar, Optional

import msgpack
import pytest

import ormsgpack


def test_dataclass() -> None:
    @dataclass
    class Dataclass:
        a: str = field()
        b: int
        c: InitVar[str]
        d: ClassVar[str] = "cls"

    obj = Dataclass("a", 1, "")
    assert ormsgpack.packb(obj) == msgpack.packb(
        {
            "a": "a",
            "b": 1,
        }
    )


def test_dataclass_with_slots() -> None:
    @dataclass
    class Dataclass:
        a: str
        b: int
        c: InitVar[str]
        d: ClassVar[str] = "cls"

        __slots__ = (
            "a",
            "b",
            "c",
        )

    obj = Dataclass("a", 1, "")
    assert not hasattr(obj, "__dict__")
    assert ormsgpack.packb(obj) == msgpack.packb(
        {
            "a": "a",
            "b": 1,
        }
    )


def test_dataclass_subclass() -> None:
    @dataclass
    class Base:
        a: str

    @dataclass
    class Dataclass(Base):
        b: int

    obj = Dataclass("a", 1)
    assert ormsgpack.packb(obj) == msgpack.packb(
        {
            "a": "a",
            "b": 1,
        }
    )


def test_dataclass_with_dict_and_slots() -> None:
    @dataclass
    class Base:
        a: str

    @dataclass
    class Dataclass(Base):
        b: int

        __slots__ = (
            "a",
            "b",
        )

    obj = Dataclass("a", 1)
    assert hasattr(obj, "__dict__")
    assert ormsgpack.packb(obj) == msgpack.packb(
        {
            "a": "a",
            "b": 1,
        }
    )


def test_dataclass_empty() -> None:
    @dataclass
    class Dataclass:
        pass

    assert ormsgpack.packb(Dataclass()) == msgpack.packb({})


def test_dataclass_empty_with_slots() -> None:
    @dataclass
    class Dataclass:
        __slots__ = ()

    assert ormsgpack.packb(Dataclass()) == msgpack.packb({})


def test_dataclass_with_private_field() -> None:
    @dataclass
    class Dataclass:
        a: str
        b: int
        _c: str

    obj = Dataclass("a", 1, "")
    assert ormsgpack.packb(obj) == msgpack.packb(
        {
            "a": "a",
            "b": 1,
        }
    )


def test_dataclass_with_private_field_and_slots() -> None:
    @dataclass
    class Dataclass:
        a: str
        b: int
        _c: str

        __slots__ = (
            "a",
            "b",
            "_c",
        )

    obj = Dataclass("a", 1, "")
    assert ormsgpack.packb(obj) == msgpack.packb(
        {
            "a": "a",
            "b": 1,
        }
    )


def test_dataclass_circular() -> None:
    @dataclass
    class Dataclass:
        a: str
        b: int
        c: Optional["Dataclass"]

    obj1 = Dataclass("a", 1, None)
    obj2 = Dataclass("b", 2, obj1)
    obj1.c = obj2
    with pytest.raises(ormsgpack.MsgpackEncodeError):
        ormsgpack.packb(obj1)


def test_dataclass_passthrough() -> None:
    @dataclass
    class Dataclass:
        a: str
        b: int

    obj = Dataclass("a", 1)
    with pytest.raises(ormsgpack.MsgpackEncodeError):
        ormsgpack.packb(obj, option=ormsgpack.OPT_PASSTHROUGH_DATACLASS)


def test_dataclass_passthrough_default() -> None:
    @dataclass
    class Dataclass:
        a: str
        b: int

    obj = Dataclass("a", 1)
    assert ormsgpack.packb(
        obj, option=ormsgpack.OPT_PASSTHROUGH_DATACLASS, default=asdict
    ) == msgpack.packb(
        {
            "a": "a",
            "b": 1,
        }
    )
