import ormsgpack, pydantic
class Group(pydantic.BaseModel):
    name: str
    uid: int

class User(pydantic.BaseModel):
    name: str
    uid: int
    groups: list[Group]
    active: bool = True

ormsgpack.packb(
    User(name="a", uid=0, groups=[Group(name="b", uid=1)]),
    option=ormsgpack.OPT_SERIALIZE_PYDANTIC,
)
ormsgpack.unpackb(_)
