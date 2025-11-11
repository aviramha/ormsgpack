import ormsgpack, datetime
ormsgpack.packb(datetime.date(1900, 1, 2))
ormsgpack.unpackb(_)
