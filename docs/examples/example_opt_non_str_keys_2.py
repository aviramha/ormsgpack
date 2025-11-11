import ormsgpack, enum
class Action(enum.Enum):
    ALLOW = 1
    DENY = 2

ormsgpack.packb(
    {
        Action.ALLOW: [443, 993],
        1: [80, 143],
    },
    option=ormsgpack.OPT_NON_STR_KEYS,
)
ormsgpack.unpackb(_, option=ormsgpack.OPT_NON_STR_KEYS)
