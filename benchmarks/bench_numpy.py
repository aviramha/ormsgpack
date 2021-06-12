import pytest
import msgpack
import numpy
import ormsgpack


int32 = numpy.random.randint(((2 ** 31) - 1), size=(100000, 100), dtype=numpy.int32)


float64 = numpy.random.random(size=(50000, 100))


npbool = numpy.random.choice((True, False), size=(100000, 200))


int8 = numpy.random.randint(((2 ** 7) - 1), size=(100000, 100), dtype=numpy.int8)


uint8 = numpy.random.randint(((2 ** 8) - 1), size=(100000, 100), dtype=numpy.uint8)


def default(__obj):
    if isinstance(__obj, numpy.ndarray):
        return __obj.tolist()


def test_int32_msgpack(benchmark):
    benchmark(msgpack.packb, int32, default=default)


def test_int32_ormsgpack(benchmark):
    benchmark(ormsgpack.packb, int32, option=ormsgpack.OPT_SERIALIZE_NUMPY)


def test_float64_msgpack(benchmark):
    benchmark(msgpack.packb, float64, default=default)


def test_float64_ormsgpack(benchmark):
    benchmark(ormsgpack.packb, float64, option=ormsgpack.OPT_SERIALIZE_NUMPY)


def test_npbool_msgpack(benchmark):
    benchmark(msgpack.packb, npbool, default=default)


def test_npbool_ormsgpack(benchmark):
    benchmark(ormsgpack.packb, npbool, option=ormsgpack.OPT_SERIALIZE_NUMPY)


def test_int8_msgpack(benchmark):
    benchmark(msgpack.packb, int8, default=default)


def test_int8_ormsgpack(benchmark):
    benchmark(ormsgpack.packb, int8, option=ormsgpack.OPT_SERIALIZE_NUMPY)


def test_uint8_msgpack(benchmark):
    benchmark(msgpack.packb, uint8, default=default)


def test_uint8_ormsgpack(benchmark):
    benchmark(ormsgpack.packb, uint8, option=ormsgpack.OPT_SERIALIZE_NUMPY)
