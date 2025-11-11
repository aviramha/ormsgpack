import ormsgpack, numpy
ormsgpack.packb(
    numpy.array([[1, 2, 3], [4, 5, 6]]),
    option=ormsgpack.OPT_SERIALIZE_NUMPY,
)
ormsgpack.unpackb(_)
