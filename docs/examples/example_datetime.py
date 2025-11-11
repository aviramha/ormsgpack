import ormsgpack, datetime, zoneinfo
tzinfo = zoneinfo.ZoneInfo("Asia/Tokyo")

ormsgpack.packb(datetime.datetime(2018, 12, 1, 2, 3))
ormsgpack.unpackb(_)

ormsgpack.packb(datetime.datetime(2018, 12, 1, 2, 3, tzinfo=tzinfo))
ormsgpack.unpackb(_)

ormsgpack.packb(
    datetime.datetime(2018, 12, 1, 2, 3, tzinfo=tzinfo),
    option=ormsgpack.OPT_DATETIME_AS_TIMESTAMP_EXT,
)
ormsgpack.unpackb(_, option=ormsgpack.OPT_DATETIME_AS_TIMESTAMP_EXT)
