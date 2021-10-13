# SPDX-License-Identifier: (Apache-2.0 OR MIT)
import abc
import uuid
from dataclasses import InitVar, asdict, dataclass, field
from enum import Enum
from typing import ClassVar, Dict, Optional

import msgpack
import pytest

import ormsgpack


class AnEnum(Enum):
    ONE = 1
    TWO = 2


@dataclass
class EmptyDataclass:
    pass


@dataclass
class EmptyDataclassSlots:
    __slots__ = ()


@dataclass
class Dataclass1:
    name: str
    number: int
    sub: Optional["Dataclass1"]


@dataclass
class Dataclass2:
    name: Optional[str] = field(default="?")


@dataclass
class Dataclass3:
    a: str
    b: int
    c: dict
    d: bool
    e: float
    f: list
    g: tuple


@dataclass
class Dataclass4:
    a: str = field()
    b: int = field(metadata={"unrelated": False})
    c: float = 1.1


@dataclass
class Datasubclass(Dataclass1):
    additional: bool


@dataclass
class Slotsdataclass:
    __slots__ = ("a", "b", "_c", "d")
    a: str
    b: int
    _c: str
    d: InitVar[str]
    cls_var: ClassVar[str] = "cls"


@dataclass
class Defaultdataclass:
    a: uuid.UUID
    b: AnEnum


@dataclass
class UnsortedDataclass:
    c: int
    b: int
    a: int
    d: Optional[Dict]


@dataclass
class InitDataclass:
    a: InitVar[str]
    b: InitVar[str]
    cls_var: ClassVar[str] = "cls"
    ab: str = ""

    def __post_init__(self, a: str, b: str):
        self._other = 1
        self.ab = f"{a} {b}"

class AbstractBase(abc.ABC):
    @abc.abstractmethod
    def key(self):
        raise NotImplementedError


@dataclass(frozen=True)
class ConcreteAbc(AbstractBase):

    __slots__ = ("attr",)

    attr: float

    def key(self):
        return "dkjf"


def test_dataclass():
    """
    packb() dataclass
    """
    obj = Dataclass1("a", 1, None)
    assert ormsgpack.packb(obj) == msgpack.packb(
        {"name": "a", "number": 1, "sub": None}
    )


def test_dataclass_recursive():
    """
    packb() dataclass recursive
    """
    obj = Dataclass1("a", 1, Dataclass1("b", 2, None))
    assert ormsgpack.packb(obj) == msgpack.packb(
        {"name": "a", "number": 1, "sub": {"name": "b", "number": 2, "sub": None}}
    )


def test_dataclass_circular():
    """
    packb() dataclass circular
    """
    obj1 = Dataclass1("a", 1, None)
    obj2 = Dataclass1("b", 2, obj1)
    obj1.sub = obj2
    with pytest.raises(ormsgpack.MsgpackEncodeError):
        ormsgpack.packb(obj1)


def test_dataclass_empty():
    """
    packb() no attributes
    """
    assert ormsgpack.packb(EmptyDataclass()) == msgpack.packb({})


def test_dataclass_empty_slots():
    """
    packb() no attributes slots
    """
    assert ormsgpack.packb(EmptyDataclassSlots()) == msgpack.packb({})


def test_dataclass_default_arg():
    """
    packb() dataclass default arg
    """
    obj = Dataclass2()
    assert ormsgpack.packb(obj) == msgpack.packb({"name": "?"})


def test_dataclass_types():
    """
    packb() dataclass types
    """
    obj = Dataclass3("a", 1, {"a": "b"}, True, 1.1, [1, 2], (3, 4))
    assert ormsgpack.packb(obj) == msgpack.packb(
        {
            "a": "a",
            "b": 1,
            "c": {"a": "b"},
            "d": True,
            "e": 1.1,
            "f": [1, 2],
            "g": [3, 4],
        }
    )


def test_dataclass_metadata():
    """
    packb() dataclass metadata
    """
    obj = Dataclass4("a", 1, 2.1)
    assert ormsgpack.packb(obj) == msgpack.packb({"a": "a", "b": 1, "c": 2.1})


def test_dataclass_classvar():
    """
    packb() dataclass class variable
    """
    obj = Dataclass4("a", 1)
    assert ormsgpack.packb(obj) == msgpack.packb({"a": "a", "b": 1, "c": 1.1})


def test_dataclass_subclass():
    """
    packb() dataclass subclass
    """
    obj = Datasubclass("a", 1, None, False)
    assert ormsgpack.packb(obj) == msgpack.packb(
        {"name": "a", "number": 1, "sub": None, "additional": False}
    )


def test_dataclass_slots():
    """
    packb() dataclass with __slots__ does not include under attributes, InitVar, or ClassVar
    """
    obj = Slotsdataclass("a", 1, "c", "d")
    assert "__dict__" not in dir(obj)
    assert ormsgpack.packb(obj) == msgpack.packb({"a": "a", "b": 1})


def test_dataclass_default():
    """
    packb() dataclass with default
    """

    def default(__obj):
        if isinstance(__obj, uuid.UUID):
            return str(__obj)
        elif isinstance(__obj, Enum):
            return __obj.value

    obj = Defaultdataclass(
        uuid.UUID("808989c0-00d5-48a8-b5c4-c804bf9032f2"), AnEnum.ONE
    )
    assert ormsgpack.packb(obj, default=default) == msgpack.packb(
        {"a": "808989c0-00d5-48a8-b5c4-c804bf9032f2", "b": 1}
    )


def test_dataclass_under():
    """
    packb() does not include under attributes, InitVar, or ClassVar
    """
    obj = InitDataclass("zxc", "vbn")
    assert ormsgpack.packb(obj) == msgpack.packb({"ab": "zxc vbn"})


def test_dataclass_passthrough_raise():
    """
    packb() dataclass passes to default with OPT_PASSTHROUGH_DATACLASS
    """
    obj = Dataclass1("a", 1, None)
    with pytest.raises(ormsgpack.MsgpackEncodeError):
        ormsgpack.packb(obj, option=ormsgpack.OPT_PASSTHROUGH_DATACLASS)
    with pytest.raises(ormsgpack.MsgpackEncodeError):
        ormsgpack.packb(
            InitDataclass("zxc", "vbn"), option=ormsgpack.OPT_PASSTHROUGH_DATACLASS
        )


def test_dataclass_passthrough_default():
    """
    packb() dataclass passes to default with OPT_PASSTHROUGH_DATACLASS
    """
    obj = Dataclass1("a", 1, None)
    assert ormsgpack.packb(
        obj, option=ormsgpack.OPT_PASSTHROUGH_DATACLASS, default=asdict
    ) == msgpack.packb({"name": "a", "number": 1, "sub": None})

    def default(obj):
        if isinstance(obj, Dataclass1):
            return {"name": obj.name, "number": obj.number}
        raise TypeError

    assert ormsgpack.packb(
        obj, option=ormsgpack.OPT_PASSTHROUGH_DATACLASS, default=default
    ) == msgpack.packb({"name": "a", "number": 1})


def test_dataclass_abc():
    obj = ConcreteAbc(1.0)
    assert ormsgpack.packb(obj) == msgpack.packb({"attr": 1.0})
