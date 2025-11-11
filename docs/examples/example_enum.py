import ormsgpack, enum
class Action(enum.Enum):
    ALLOW = 1
    DENY = 2

ormsgpack.packb(Action.ALLOW)
ormsgpack.unpackb(_)
