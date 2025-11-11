import ormsgpack, datetime
ormsgpack.packb(datetime.time(12, 0, 15, 290))
ormsgpack.unpackb(_)
