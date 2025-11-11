import ormsgpack, datetime, numpy
event = {
    "type": "put",
    "time": datetime.datetime(1970, 1, 1),
    "uid": 1,
    "data": numpy.array([1, 2]),
}
ormsgpack.packb(event, option=ormsgpack.OPT_SERIALIZE_NUMPY)
ormsgpack.unpackb(_)
