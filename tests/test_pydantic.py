# SPDX-License-Identifier: (Apache-2.0 OR MIT)
import sys

import pydantic
import pytest

import ormsgpack


def test_pydantic_model() -> None:
    class Point1D(pydantic.BaseModel):
        x: int

    class Point2D(Point1D):
        y: int

    class Model(pydantic.BaseModel):
        a: str
        b: int
        c: Point1D

    obj = Model(a="a", b=1, c=Point2D(x=0, y=0))
    packed = ormsgpack.packb(obj, option=ormsgpack.OPT_SERIALIZE_PYDANTIC)
    assert packed == ormsgpack.packb(obj.model_dump(serialize_as_any=True))
    assert ormsgpack.unpackb(packed) == {
        "a": "a",
        "b": 1,
        "c": {
            "x": 0,
            "y": 0,
        },
    }


@pytest.mark.skipif(
    sys.version_info >= (3, 14),
    reason="pydantic v1 does not support Python 3.14 and greater",
)
def test_pydantic_v1_model() -> None:
    import pydantic.v1

    class Point1D(pydantic.v1.BaseModel):
        x: int

    class Point2D(Point1D):
        y: int

    class Model(pydantic.v1.BaseModel):
        a: str
        b: int
        c: Point1D

    obj = Model(a="a", b=1, c=Point2D(x=0, y=0))
    packed = ormsgpack.packb(obj, option=ormsgpack.OPT_SERIALIZE_PYDANTIC)
    assert packed == ormsgpack.packb(obj.dict())
    assert ormsgpack.unpackb(packed) == {
        "a": "a",
        "b": 1,
        "c": {
            "x": 0,
            "y": 0,
        },
    }


def test_pydantic_model_sort_keys() -> None:
    class Model(pydantic.BaseModel):
        b: int
        c: int
        a: int

    obj = Model(b=1, c=2, a=3)
    packed = ormsgpack.packb(
        obj,
        option=ormsgpack.OPT_SERIALIZE_PYDANTIC | ormsgpack.OPT_SORT_KEYS,
    )
    assert list(obj.__dict__.keys()) != sorted(obj.__dict__.keys())
    assert list(ormsgpack.unpackb(packed).items()) == [
        ("a", 3),
        ("b", 1),
        ("c", 2),
    ]
