import ormsgpack, datetime
ormsgpack.packb(
    datetime.datetime(1970, 1, 1, 0, 0, 0, tzinfo=datetime.timezone.utc),
)
ormsgpack.packb(
    datetime.datetime(1970, 1, 1, 0, 0, 0, tzinfo=datetime.timezone.utc),
    option=ormsgpack.OPT_UTC_Z,
)
