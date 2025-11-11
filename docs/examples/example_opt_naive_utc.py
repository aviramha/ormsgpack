import ormsgpack, datetime
ormsgpack.packb(
    datetime.datetime(1970, 1, 1, 0, 0, 0),
)
ormsgpack.unpackb(_)
ormsgpack.packb(
    datetime.datetime(1970, 1, 1, 0, 0, 0),
    option=ormsgpack.OPT_NAIVE_UTC,
)
ormsgpack.unpackb(_)
