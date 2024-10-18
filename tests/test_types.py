# SPDX-License-Identifier: (Apache-2.0 OR MIT)

import datetime
import enum
import uuid
from dataclasses import dataclass, make_dataclass

import msgpack
import pytest
from pydantic import BaseModel, create_model

import ormsgpack


@dataclass
class Dataclass:
    a: str


class Enum(enum.Enum):
    A = "a"


class Model(BaseModel):
    a: str


class StrSubclass(str):
    pass


def default(obj: object) -> object:
    if isinstance(obj, set):
        return str(obj)
    raise TypeError


TYPE_PARAMS = (
    pytest.param(
        None,
        None,
        0,
        id="None",
    ),
    pytest.param(
        True,
        True,
        0,
        id="bool",
    ),
    pytest.param(
        1.1,
        1.1,
        0,
        id="float",
    ),
    pytest.param(
        1,
        1,
        0,
        id="int",
    ),
    pytest.param(
        b"a",
        b"a",
        0,
        id="bytes",
    ),
    pytest.param(
        "a",
        "a",
        0,
        id="str",
    ),
    pytest.param(
        StrSubclass("a"),
        "a",
        0,
        id="str subclass",
    ),
    pytest.param(
        {"a": "b"},
        {"a": "b"},
        0,
        id="dict",
    ),
    pytest.param(
        [1, 2],
        [1, 2],
        0,
        id="list",
    ),
    pytest.param(
        (1, 2),
        [1, 2],
        0,
        id="tuple",
    ),
    pytest.param(
        Dataclass("a"),
        {"a": "a"},
        0,
        id="dataclass",
    ),
    pytest.param(
        datetime.datetime(2000, 1, 1, 2, 3, 4, 123),
        "2000-01-01T02:03:04.000123",
        0,
        id="datetime",
    ),
    pytest.param(
        datetime.datetime(2000, 1, 1, 2, 3, 4, 123),
        "2000-01-01T02:03:04",
        ormsgpack.OPT_OMIT_MICROSECONDS,
        id="datetime options",
    ),
    pytest.param(
        datetime.date(2000, 1, 1),
        "2000-01-01",
        0,
        id="date",
    ),
    pytest.param(
        datetime.time(2, 3, 4, 123),
        "02:03:04.000123",
        0,
        id="time",
    ),
    pytest.param(
        Enum.A,
        "a",
        0,
        id="enum",
    ),
    pytest.param(
        Model(a="a"),
        {"a": "a"},
        ormsgpack.OPT_SERIALIZE_PYDANTIC,
        id="pydantic",
    ),
    pytest.param(
        uuid.UUID("00000000-0000-0000-0000-000000000000"),
        "00000000-0000-0000-0000-000000000000",
        0,
        id="uuid",
    ),
    pytest.param(
        {1, 2},
        "{1, 2}",
        0,
        id="unknown",
    ),
)


@pytest.mark.parametrize(("value", "converted_value", "option"), TYPE_PARAMS)
def test_dataclass(value: object, converted_value: object, option: int) -> None:
    dataclass_type = make_dataclass("TestDataclass", [("a", type(value))])
    obj = dataclass_type(value)
    converted_obj = {"a": converted_value}

    packed = ormsgpack.packb(obj, default=default, option=option)
    assert packed == msgpack.packb(converted_obj)
    assert ormsgpack.unpackb(packed) == converted_obj


@pytest.mark.parametrize(("value", "converted_value", "option"), TYPE_PARAMS)
def test_dict(value: object, converted_value: object, option: int) -> None:
    obj = {"a": value}
    converted_obj = {"a": converted_value}

    packed = ormsgpack.packb(obj, default=default, option=option)
    assert packed == msgpack.packb(converted_obj)
    assert ormsgpack.unpackb(packed) == converted_obj


@pytest.mark.parametrize(("value", "converted_value", "option"), TYPE_PARAMS)
def test_enum(value: object, converted_value: object, option: int) -> None:
    class TestEnum(enum.Enum):
        A = value

    obj = TestEnum.A
    converted_obj = converted_value

    packed = ormsgpack.packb(obj, default=default, option=option)
    assert packed == msgpack.packb(converted_obj)
    assert ormsgpack.unpackb(packed) == converted_obj


@pytest.mark.parametrize(("value", "converted_value", "option"), TYPE_PARAMS)
def test_list(value: object, converted_value: object, option: int) -> None:
    obj = [value]
    converted_obj = [converted_value]

    packed = ormsgpack.packb(obj, default=default, option=option)
    assert packed == msgpack.packb(converted_obj)
    assert ormsgpack.unpackb(packed) == converted_obj


@pytest.mark.parametrize(("value", "converted_value", "option"), TYPE_PARAMS)
def test_tuple(value: object, converted_value: object, option: int) -> None:
    obj = (value,)
    converted_obj = [converted_value]

    packed = ormsgpack.packb(obj, default=default, option=option)
    assert packed == msgpack.packb(converted_obj)
    assert ormsgpack.unpackb(packed) == converted_obj


@pytest.mark.parametrize(
    ("value", "converted_value", "option"),
    (
        param
        for param in TYPE_PARAMS
        if param.id
        not in {
            "str subclass",
            "unknown",
        }
    ),
)
def test_pydantic(value: object, converted_value: object, option: int) -> None:
    model_type = create_model("TestModel", a=(type(value), ...))
    obj = model_type(a=value)
    converted_obj = {"a": converted_value}

    packed = ormsgpack.packb(
        obj,
        default=default,
        option=option | ormsgpack.OPT_SERIALIZE_PYDANTIC,
    )
    assert packed == msgpack.packb(converted_obj)
    assert ormsgpack.unpackb(packed) == converted_obj


@pytest.mark.parametrize(
    ("value", "converted_value", "option"),
    (param for param in TYPE_PARAMS if param.id != "unknown"),
)
def test_default(value: object, converted_value: object, option: int) -> None:
    packed = ormsgpack.packb(object(), default=lambda _: value, option=option)
    assert packed == msgpack.packb(converted_value)


@pytest.mark.parametrize(
    ("value", "converted_value", "option"),
    (
        param
        for param in TYPE_PARAMS
        if param.id
        not in {
            "dict",
            "list",
            "dataclass",
            "pydantic",
            "unknown",
        }
    ),
)
def test_dict_key(value: object, converted_value: object, option: int) -> None:
    obj = {value: True}
    if isinstance(value, tuple):
        converted_value = value
    converted_obj = {converted_value: True}

    if type(value) is not str:
        with pytest.raises(ormsgpack.MsgpackEncodeError):
            ormsgpack.packb(obj)
    packed = ormsgpack.packb(obj, option=option | ormsgpack.OPT_NON_STR_KEYS)
    assert packed == msgpack.packb(converted_obj)

    if not isinstance(converted_value, (str, bytes)):
        with pytest.raises(ormsgpack.MsgpackDecodeError):
            ormsgpack.unpackb(packed)
    assert ormsgpack.unpackb(packed, option=ormsgpack.OPT_NON_STR_KEYS) == converted_obj


def test_object() -> None:
    with pytest.raises(ormsgpack.MsgpackEncodeError):
        ormsgpack.packb(object())
