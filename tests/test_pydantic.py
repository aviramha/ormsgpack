import ormsgpack

from typing import Optional

from pydantic import BaseModel

class Model1(BaseModel):
    hi: str
    number: int
    sub: Optional[int]

class Model2(BaseModel):
    bye: str
    previous: Model1



def test_basemodel():
    """
    packb() pydantic basemodel
    """
    obj = Model1(hi="a", number=1, sub=None)
    packed = ormsgpack.packb(obj, option=ormsgpack.OPT_SERIALIZE_PYDANTIC)
    assert ormsgpack.unpackb(packed)== {"hi":"a","number":1,"sub":None}


def test_recursive_basemodel():
    """
    packb() pydantic basemodel with another basemodel as attribute
    """
    obj = Model1(hi="a", number=1, sub=None)
    obj2 = Model2(previous=obj, bye="lala")
    packed = ormsgpack.packb(obj2, option=ormsgpack.OPT_SERIALIZE_PYDANTIC)
    assert ormsgpack.unpackb(packed) == {"bye":"lala","previous":{"hi":"a","number":1,"sub":None}}