# SPDX-License-Identifier: (Apache-2.0 OR MIT)
from dataclasses import InitVar, asdict, dataclass, field
from functools import cached_property
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


def test_dataclass_with_non_init_field() -> None:
    @dataclass
    class Dataclass:
        a: str
        b: int = field(default=1, init=False)

    obj = Dataclass("a")
    assert ormsgpack.packb(obj) == msgpack.packb(
        {
            "a": "a",
            "b": 1,
        }
    )


def test_dataclass_with_descriptor_field() -> None:
    class Descriptor:
        def __init__(self, *, default: int) -> None:
            self._default = default

        def __set_name__(self, owner: object, name: str) -> None:
            self._name = "_" + name

        def __get__(self, instance: object, owner: object) -> int:
            if instance is None:
                return self._default

            return getattr(instance, self._name, self._default)

        def __set__(self, instance: object, value: int) -> None:
            setattr(instance, self._name, value)

    @dataclass
    class Dataclass:
        a: str
        b: Descriptor = Descriptor(default=0)

    obj = Dataclass("a", 1)
    assert ormsgpack.packb(obj) == msgpack.packb(
        {
            "a": "a",
            "b": 1,
        }
    )


def test_dataclass_with_cached_property() -> None:
    @dataclass
    class Dataclass:
        a: str
        b: int

        @cached_property
        def name(self) -> str:
            return "dataclass"

    obj = Dataclass("a", 1)
    obj.name
    assert ormsgpack.packb(obj) == msgpack.packb(
        {
            "a": "a",
            "b": 1,
        }
    )


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
