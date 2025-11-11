import ormsgpack, datetime
ormsgpack.packb(
    datetime.datetime(1970, 1, 1, 0, 0, 0, 1),
)
ormsgpack.unpackb(_)
ormsgpack.packb(
    datetime.datetime(1970, 1, 1, 0, 0, 0, 1),
    option=ormsgpack.OPT_OMIT_MICROSECONDS,
)
ormsgpack.unpackb(_)
