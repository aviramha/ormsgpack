from typing import Optional

from pydantic import BaseModel

import ormsgpack


class Model1(BaseModel):
    hi: str
    number: int
    sub: Optional[int]


def test_basemodel() -> None:
    """
    packb() pydantic basemodel
    """
    obj = Model1(hi="a", number=1, sub=None)
    packed = ormsgpack.packb(obj, option=ormsgpack.OPT_SERIALIZE_PYDANTIC)
    assert ormsgpack.unpackb(packed) == {"hi": "a", "number": 1, "sub": None}
