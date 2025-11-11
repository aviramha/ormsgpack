import ormsgpack, dataclasses
@dataclasses.dataclass
class Group:
    name: str
    uid: int

@dataclasses.dataclass
class User:
    name: str
    uid: int
    groups: list[Group]
    active: bool = True

ormsgpack.packb(
    User(name="a", uid=0, groups=[Group(name="b", uid=1)]),
)
ormsgpack.unpackb(_)
