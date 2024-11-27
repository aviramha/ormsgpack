import pydantic
import pydantic.v1

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


def test_pydantic_v1_model() -> None:
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
