import msgpack
import numpy
import pytest

import ormsgpack

DATA_TYPES = {
    "int32": numpy.random.randint(
        ((2**31) - 1), size=(100000, 100), dtype=numpy.int32
    ),
    "float64": numpy.random.random(size=(50000, 100)),
    "npbool": numpy.random.choice((True, False), size=(100000, 200)),
    "int8": numpy.random.randint(((2**7) - 1), size=(100000, 100), dtype=numpy.int8),
    "uint8": numpy.random.randint(
        ((2**8) - 1), size=(100000, 100), dtype=numpy.uint8
    ),
}

PARAMETERS = tuple(DATA_TYPES.keys())


def default(__obj):
    if isinstance(__obj, numpy.ndarray):
        return __obj.tolist()


@pytest.mark.parametrize("data_type", PARAMETERS)
def test_numpy_msgpack(benchmark, data_type):
    benchmark.group = f"numpy {data_type}"
    data = DATA_TYPES[data_type]
    benchmark(msgpack.packb, data, default=default)


@pytest.mark.parametrize("data_type", PARAMETERS)
def test_numpy_ormsgpack(benchmark, data_type):
    benchmark.group = f"numpy {data_type}"
    data = DATA_TYPES[data_type]
    benchmark(ormsgpack.packb, data, option=ormsgpack.OPT_SERIALIZE_NUMPY)
