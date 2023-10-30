# SPDX-License-Identifier: (Apache-2.0 OR MIT)

import msgpack
import pytest

import ormsgpack

try:
    import numpy
except ImportError:
    numpy = None  # type: ignore
    pytestmark = pytest.mark.skip


def numpy_default(obj):
    return obj.tolist()


def test_numpy_array_d1_uintp():
    assert ormsgpack.packb(
        numpy.array([0, 18446744073709551615], numpy.uintp),
        option=ormsgpack.OPT_SERIALIZE_NUMPY,
    ) == msgpack.packb([0, 18446744073709551615])


def test_numpy_array_d1_intp():
    assert ormsgpack.packb(
        numpy.array([-9223372036854775807, 9223372036854775807], numpy.intp),
        option=ormsgpack.OPT_SERIALIZE_NUMPY,
    ) == msgpack.packb([-9223372036854775807, 9223372036854775807])


def test_numpy_array_d1_i64():
    assert ormsgpack.packb(
        numpy.array([-9223372036854775807, 9223372036854775807], numpy.int64),
        option=ormsgpack.OPT_SERIALIZE_NUMPY,
    ) == msgpack.packb([-9223372036854775807, 9223372036854775807])


def test_numpy_array_d1_u64():
    assert ormsgpack.packb(
        numpy.array([0, 18446744073709551615], numpy.uint64),
        option=ormsgpack.OPT_SERIALIZE_NUMPY,
    ) == msgpack.packb([0, 18446744073709551615])


def test_numpy_array_d1_i8():
    assert ormsgpack.packb(
        numpy.array([-128, 127], numpy.int8),
        option=ormsgpack.OPT_SERIALIZE_NUMPY,
    ) == msgpack.packb([-128, 127])


def test_numpy_array_d1_u8():
    assert ormsgpack.packb(
        numpy.array([0, 255], numpy.uint8),
        option=ormsgpack.OPT_SERIALIZE_NUMPY,
    ) == msgpack.packb([0, 255])


def test_numpy_array_d1_i16():
    assert ormsgpack.packb(
        numpy.array([-32768, 32767], numpy.int16),
        option=ormsgpack.OPT_SERIALIZE_NUMPY,
    ) == msgpack.packb([-32768, 32767])


def test_numpy_array_d1_u16():
    assert ormsgpack.packb(
        numpy.array([0, 65535], numpy.uint16),
        option=ormsgpack.OPT_SERIALIZE_NUMPY,
    ) == msgpack.packb([0, 65535])


def test_numpy_array_d1_i32():
    assert ormsgpack.packb(
        numpy.array([-2147483647, 2147483647], numpy.int32),
        option=ormsgpack.OPT_SERIALIZE_NUMPY,
    ) == msgpack.packb([-2147483647, 2147483647])


def test_numpy_array_d1_u32():
    assert ormsgpack.packb(
        numpy.array([0, 4294967295], numpy.uint32),
        option=ormsgpack.OPT_SERIALIZE_NUMPY,
    ) == msgpack.packb([0, 4294967295])


def test_numpy_array_d1_f32():
    array = numpy.array([1.0, 3.4028235e38], numpy.float32)
    py_array = [float(x) for x in array]
    ormsgpacked = ormsgpack.packb(
        array,
        option=ormsgpack.OPT_SERIALIZE_NUMPY,
    )
    original_msgpacked = msgpack.packb(py_array)
    assert ormsgpack.unpackb(ormsgpacked) == msgpack.unpackb(original_msgpacked)


def test_numpy_array_d1_f64():
    assert ormsgpack.packb(
        numpy.array([1.0, 1.7976931348623157e308], numpy.float64),
        option=ormsgpack.OPT_SERIALIZE_NUMPY,
    ) == msgpack.packb([1.0, 1.7976931348623157e308])


def test_numpy_array_d1_bool():
    assert ormsgpack.packb(
        numpy.array([True, False, False, True]),
        option=ormsgpack.OPT_SERIALIZE_NUMPY,
    ) == msgpack.packb([True, False, False, True])


def test_numpy_array_d2_i64():
    assert ormsgpack.packb(
        numpy.array([[1, 2, 3], [4, 5, 6]], numpy.int64),
        option=ormsgpack.OPT_SERIALIZE_NUMPY,
    ) == msgpack.packb([[1, 2, 3], [4, 5, 6]])


def test_numpy_array_d2_f64():
    assert ormsgpack.packb(
        numpy.array([[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]], numpy.float64),
        option=ormsgpack.OPT_SERIALIZE_NUMPY,
    ) == msgpack.packb([[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]])


def test_numpy_array_d3_i8():
    assert ormsgpack.packb(
        numpy.array([[[1, 2], [3, 4]], [[5, 6], [7, 8]]], numpy.int8),
        option=ormsgpack.OPT_SERIALIZE_NUMPY,
    ) == msgpack.packb([[[1, 2], [3, 4]], [[5, 6], [7, 8]]])


def test_numpy_array_d3_u8():
    assert ormsgpack.packb(
        numpy.array([[[1, 2], [3, 4]], [[5, 6], [7, 8]]], numpy.uint8),
        option=ormsgpack.OPT_SERIALIZE_NUMPY,
    ) == msgpack.packb([[[1, 2], [3, 4]], [[5, 6], [7, 8]]])


def test_numpy_array_d3_i32():
    assert ormsgpack.packb(
        numpy.array([[[1, 2], [3, 4]], [[5, 6], [7, 8]]], numpy.int32),
        option=ormsgpack.OPT_SERIALIZE_NUMPY,
    ) == msgpack.packb([[[1, 2], [3, 4]], [[5, 6], [7, 8]]])


def test_numpy_array_d3_i64():
    assert ormsgpack.packb(
        numpy.array([[[1, 2], [3, 4], [5, 6], [7, 8]]], numpy.int64),
        option=ormsgpack.OPT_SERIALIZE_NUMPY,
    ) == msgpack.packb([[[1, 2], [3, 4], [5, 6], [7, 8]]])


def test_numpy_array_d3_f64():
    assert ormsgpack.packb(
        numpy.array(
            [[[1.0, 2.0], [3.0, 4.0]], [[5.0, 6.0], [7.0, 8.0]]], numpy.float64
        ),
        option=ormsgpack.OPT_SERIALIZE_NUMPY,
    ) == msgpack.packb([[[1.0, 2.0], [3.0, 4.0]], [[5.0, 6.0], [7.0, 8.0]]])


def test_numpy_array_fortran():
    array = numpy.array([[1, 2], [3, 4]], order="F")
    assert array.flags["F_CONTIGUOUS"] == True
    with pytest.raises(ormsgpack.MsgpackEncodeError):
        ormsgpack.packb(array, option=ormsgpack.OPT_SERIALIZE_NUMPY)
    assert ormsgpack.packb(
        array, default=numpy_default, option=ormsgpack.OPT_SERIALIZE_NUMPY
    ) == ormsgpack.packb(array.tolist())


def test_numpy_array_non_contiguous_message():
    array = numpy.array([[1, 2], [3, 4]], order="F")
    assert array.flags["F_CONTIGUOUS"] == True
    try:
        ormsgpack.packb(array, option=ormsgpack.OPT_SERIALIZE_NUMPY)
        assert False
    except TypeError as exc:
        assert (
            str(exc)
            == "numpy array is not C contiguous; use ndarray.tolist() in default"
        )


def test_numpy_array_unsupported_dtype():
    array = numpy.array([[1, 2], [3, 4]], numpy.float16)
    with pytest.raises(ormsgpack.MsgpackEncodeError):
        ormsgpack.packb(array, option=ormsgpack.OPT_SERIALIZE_NUMPY)
    assert ormsgpack.packb(
        array, default=numpy_default, option=ormsgpack.OPT_SERIALIZE_NUMPY
    ) == ormsgpack.packb(array.tolist())


def test_numpy_array_d1():
    array = numpy.array([1])
    assert (
        ormsgpack.unpackb(
            ormsgpack.packb(
                array,
                option=ormsgpack.OPT_SERIALIZE_NUMPY,
            )
        )
        == array.tolist()
    )


def test_numpy_array_d2():
    array = numpy.array([[1]])
    assert (
        ormsgpack.unpackb(
            ormsgpack.packb(
                array,
                option=ormsgpack.OPT_SERIALIZE_NUMPY,
            )
        )
        == array.tolist()
    )


def test_numpy_array_d3():
    array = numpy.array([[[1]]])
    assert (
        ormsgpack.unpackb(
            ormsgpack.packb(
                array,
                option=ormsgpack.OPT_SERIALIZE_NUMPY,
            )
        )
        == array.tolist()
    )


def test_numpy_array_d4():
    array = numpy.array([[[[1]]]])
    assert (
        ormsgpack.unpackb(
            ormsgpack.packb(
                array,
                option=ormsgpack.OPT_SERIALIZE_NUMPY,
            )
        )
        == array.tolist()
    )


def test_numpy_array_4_stride():
    array = numpy.random.rand(4, 4, 4, 4)
    assert (
        ormsgpack.unpackb(
            ormsgpack.packb(
                array,
                option=ormsgpack.OPT_SERIALIZE_NUMPY,
            )
        )
        == array.tolist()
    )


def test_numpy_array_dimension_zero():
    array = numpy.array(0)
    assert array.ndim == 0
    with pytest.raises(ormsgpack.MsgpackEncodeError):
        ormsgpack.packb(array, option=ormsgpack.OPT_SERIALIZE_NUMPY)

    array = numpy.empty((0, 4, 2))
    assert (
        ormsgpack.unpackb(
            ormsgpack.packb(
                array,
                option=ormsgpack.OPT_SERIALIZE_NUMPY,
            )
        )
        == array.tolist()
    )

    array = numpy.empty((4, 0, 2))
    assert (
        ormsgpack.unpackb(
            ormsgpack.packb(
                array,
                option=ormsgpack.OPT_SERIALIZE_NUMPY,
            )
        )
        == array.tolist()
    )

    array = numpy.empty((2, 4, 0))
    assert (
        ormsgpack.unpackb(
            ormsgpack.packb(
                array,
                option=ormsgpack.OPT_SERIALIZE_NUMPY,
            )
        )
        == array.tolist()
    )


def test_numpy_array_dimension_max():
    array = numpy.random.rand(
        1,
        1,
        1,
        1,
        1,
        1,
        1,
        1,
        1,
        1,
        1,
        1,
        1,
        1,
        1,
        1,
        1,
        1,
        1,
        1,
        1,
        1,
        1,
        1,
        1,
        1,
        1,
        1,
        1,
        1,
        1,
        1,
    )
    assert array.ndim == 32
    assert (
        ormsgpack.unpackb(
            ormsgpack.packb(
                array,
                option=ormsgpack.OPT_SERIALIZE_NUMPY,
            )
        )
        == array.tolist()
    )


def test_numpy_scalar_int8():
    assert ormsgpack.packb(
        numpy.int8(0), option=ormsgpack.OPT_SERIALIZE_NUMPY
    ) == msgpack.packb(0)

    assert ormsgpack.packb(
        numpy.int8(127), option=ormsgpack.OPT_SERIALIZE_NUMPY
    ) == msgpack.packb(127)

    assert ormsgpack.packb(
        numpy.int8(-128), option=ormsgpack.OPT_SERIALIZE_NUMPY
    ) == msgpack.packb(-128)


def test_numpy_scalar_int16():
    assert ormsgpack.packb(
        numpy.int16(0), option=ormsgpack.OPT_SERIALIZE_NUMPY
    ) == msgpack.packb(0)

    assert ormsgpack.packb(
        numpy.int16(32767), option=ormsgpack.OPT_SERIALIZE_NUMPY
    ) == msgpack.packb(32767)

    assert ormsgpack.packb(
        numpy.int16(-32768), option=ormsgpack.OPT_SERIALIZE_NUMPY
    ) == msgpack.packb(-32768)


def test_numpy_scalar_int32():
    assert ormsgpack.packb(
        numpy.int32(1), option=ormsgpack.OPT_SERIALIZE_NUMPY
    ) == msgpack.packb(1)

    assert ormsgpack.packb(
        numpy.int32(2147483647), option=ormsgpack.OPT_SERIALIZE_NUMPY
    ) == msgpack.packb(2147483647)

    assert ormsgpack.packb(
        numpy.int32(-2147483648), option=ormsgpack.OPT_SERIALIZE_NUMPY
    ) == msgpack.packb(-2147483648)


def test_numpy_scalar_int64():
    assert ormsgpack.packb(
        numpy.int64(-9223372036854775808), option=ormsgpack.OPT_SERIALIZE_NUMPY
    ) == msgpack.packb(-9223372036854775808)

    assert ormsgpack.packb(
        numpy.int64(9223372036854775807), option=ormsgpack.OPT_SERIALIZE_NUMPY
    ) == msgpack.packb(9223372036854775807)


def test_numpy_scalar_uint8():
    assert ormsgpack.packb(
        numpy.uint8(0), option=ormsgpack.OPT_SERIALIZE_NUMPY
    ) == msgpack.packb(0)
    assert ormsgpack.packb(
        numpy.uint8(255), option=ormsgpack.OPT_SERIALIZE_NUMPY
    ) == msgpack.packb(255)


def test_numpy_scalar_uint16():
    assert ormsgpack.packb(
        numpy.uint16(0), option=ormsgpack.OPT_SERIALIZE_NUMPY
    ) == msgpack.packb(0)
    assert ormsgpack.packb(
        numpy.uint16(65535), option=ormsgpack.OPT_SERIALIZE_NUMPY
    ) == msgpack.packb(65535)


def test_numpy_scalar_uint32():
    assert ormsgpack.packb(
        numpy.uint32(0), option=ormsgpack.OPT_SERIALIZE_NUMPY
    ) == msgpack.packb(0)
    assert ormsgpack.packb(
        numpy.uint32(4294967295), option=ormsgpack.OPT_SERIALIZE_NUMPY
    ) == msgpack.packb(4294967295)


def test_numpy_scalar_uint64():
    assert ormsgpack.packb(
        numpy.uint64(0), option=ormsgpack.OPT_SERIALIZE_NUMPY
    ) == msgpack.packb(0)
    assert ormsgpack.packb(
        numpy.uint64(18446744073709551615), option=ormsgpack.OPT_SERIALIZE_NUMPY
    ) == msgpack.packb(18446744073709551615)


def test_numpy_scalar_float32():
    assert (
        ormsgpack.unpackb(
            ormsgpack.packb(numpy.float32(1.0), option=ormsgpack.OPT_SERIALIZE_NUMPY)
        )
        == 1.0
    )


def test_numpy_scalar_float64():
    assert ormsgpack.packb(
        numpy.float64(123.123), option=ormsgpack.OPT_SERIALIZE_NUMPY
    ) == msgpack.packb(123.123)


def test_numpy_bool():
    data = {"a": numpy.bool_(True), "b": numpy.bool_(False)}
    assert ormsgpack.packb(
        data,
        option=ormsgpack.OPT_SERIALIZE_NUMPY,
    ) == msgpack.packb({"a": True, "b": False})
