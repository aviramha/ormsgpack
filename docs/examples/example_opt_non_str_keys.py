import ormsgpack

ormsgpack.packb(
    {0: [1, 2], 1.0: [3, 4]},
)
ormsgpack.packb(
    {0: [1, 2], 1.0: [3, 4]},
    option=ormsgpack.OPT_NON_STR_KEYS,
)
ormsgpack.unpackb(_, option=ormsgpack.OPT_NON_STR_KEYS)
