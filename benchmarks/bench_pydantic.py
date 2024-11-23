import msgpack
from pydantic import BaseModel

import ormsgpack


class Member(BaseModel):
    id: int
    active: bool


class Object(BaseModel):
    id: int
    name: str
    members: list[Member]


objects_as_pydantic = [
    Object(
        id=i, name=str(i) * 3, members=[Member(id=j, active=True) for j in range(0, 10)]
    )
    for i in range(100000, 102000)
]


def default(__obj):
    if isinstance(__obj, BaseModel):
        return __obj.dict()


def test_pydantic_msgpack(benchmark):
    benchmark.group = "pydantic"
    benchmark(msgpack.packb, objects_as_pydantic, default=default)


def test_pydantic_ormsgpack(benchmark):
    benchmark.group = "pydantic"
    benchmark(
        ormsgpack.packb, objects_as_pydantic, option=ormsgpack.OPT_SERIALIZE_PYDANTIC
    )
