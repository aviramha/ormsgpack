# SPDX-License-Identifier: (Apache-2.0 OR MIT)

import datetime
import sys

import msgpack
import pendulum
import pytest
import pytz
from dateutil import tz

import ormsgpack

if sys.version_info >= (3, 9):
    import zoneinfo

    ZoneInfo = zoneinfo.ZoneInfo
    ZoneInfoUTC = zoneinfo.ZoneInfo("UTC")
else:
    ZoneInfo = None
    ZoneInfoUTC = None


TIMEZONE_PARAMS = (
    pytest.param(pendulum.timezone, id="pendulum"),
    pytest.param(pytz.timezone, id="pytz"),
    pytest.param(tz.gettz, id="dateutil"),
    pytest.param(
        ZoneInfo,
        id="zoneinfo",
        marks=pytest.mark.skipif(ZoneInfo is None, reason="zoneinfo not available"),
    ),
)


def test_datetime_naive():
    """
    datetime.datetime naive prints without offset
    """
    assert ormsgpack.packb(
        [datetime.datetime(2000, 1, 1, 2, 3, 4, 123)]
    ) == msgpack.packb(["2000-01-01T02:03:04.000123"])


def test_datetime_naive_utc():
    """
    datetime.datetime naive with opt assumes UTC
    """
    assert ormsgpack.packb(
        [datetime.datetime(2000, 1, 1, 2, 3, 4, 123)],
        option=ormsgpack.OPT_NAIVE_UTC,
    ) == msgpack.packb(["2000-01-01T02:03:04.000123+00:00"])


def test_datetime_min():
    """
    datetime.datetime min range
    """
    assert ormsgpack.packb(
        [datetime.datetime(datetime.MINYEAR, 1, 1, 0, 0, 0, 0)],
        option=ormsgpack.OPT_NAIVE_UTC,
    ) == msgpack.packb(["0001-01-01T00:00:00+00:00"])


def test_datetime_max():
    """
    datetime.datetime max range
    """
    assert ormsgpack.packb(
        [datetime.datetime(datetime.MAXYEAR, 12, 31, 23, 59, 50, 999999)],
        option=ormsgpack.OPT_NAIVE_UTC,
    ) == msgpack.packb(["9999-12-31T23:59:50.999999+00:00"])


def test_datetime_three_digits():
    """
    datetime.datetime three digit year
    """
    assert ormsgpack.packb(
        [datetime.datetime(312, 1, 1)],
        option=ormsgpack.OPT_NAIVE_UTC,
    ) == msgpack.packb(["0312-01-01T00:00:00+00:00"])


def test_datetime_two_digits():
    """
    datetime.datetime two digit year
    """
    assert ormsgpack.packb(
        [datetime.datetime(46, 1, 1)],
        option=ormsgpack.OPT_NAIVE_UTC,
    ) == msgpack.packb(["0046-01-01T00:00:00+00:00"])


def test_datetime_tz_assume():
    """
    datetime.datetime tz with assume UTC uses tz
    """
    assert ormsgpack.packb(
        [datetime.datetime(2018, 1, 1, 2, 3, 4, 0, tzinfo=tz.gettz("Asia/Shanghai"))],
        option=ormsgpack.OPT_NAIVE_UTC,
    ) == msgpack.packb(["2018-01-01T02:03:04+08:00"])


@pytest.mark.parametrize(
    "timezone",
    (
        pytest.param(datetime.timezone.utc, id="datetime"),
        pytest.param(pendulum.UTC, id="pendulum"),
        pytest.param(pytz.UTC, id="pytz"),
        pytest.param(tz.UTC, id="dateutil"),
        pytest.param(
            ZoneInfoUTC,
            id="zoneinfo",
            marks=pytest.mark.skipif(ZoneInfo is None, reason="zoneinfo not available"),
        ),
    ),
)
def test_datetime_utc(timezone):
    """
    datetime.datetime UTC
    """
    assert ormsgpack.packb(
        [datetime.datetime(2018, 6, 1, 2, 3, 4, 0, tzinfo=timezone)]
    ) == msgpack.packb(["2018-06-01T02:03:04+00:00"])


@pytest.mark.parametrize("timezone", TIMEZONE_PARAMS)
def test_datetime_positive(timezone):
    """
    datetime.datetime positive UTC
    """
    assert ormsgpack.packb(
        [
            datetime.datetime(
                2018,
                1,
                1,
                2,
                3,
                4,
                0,
                tzinfo=timezone("Asia/Shanghai"),
            )
        ]
    ) == msgpack.packb(["2018-01-01T02:03:04+08:00"])


@pytest.mark.parametrize("timezone", TIMEZONE_PARAMS)
def test_datetime_negative_dst(timezone):
    """
    datetime.datetime negative UTC DST
    """
    assert ormsgpack.packb(
        [
            datetime.datetime(
                2018,
                6,
                1,
                2,
                3,
                4,
                0,
                tzinfo=timezone("America/New_York"),
            )
        ]
    ) == msgpack.packb(["2018-06-01T02:03:04-04:00"])


@pytest.mark.parametrize("timezone", TIMEZONE_PARAMS)
def test_datetime_negative_non_dst(timezone):
    """
    datetime.datetime negative UTC non-DST
    """
    assert ormsgpack.packb(
        [
            datetime.datetime(
                2018,
                12,
                1,
                2,
                3,
                4,
                0,
                tzinfo=timezone("America/New_York"),
            )
        ]
    ) == msgpack.packb(["2018-12-01T02:03:04-05:00"])


@pytest.mark.parametrize("timezone", TIMEZONE_PARAMS)
def test_datetime_partial_hour(timezone):
    """
    datetime.datetime UTC offset partial hour
    """
    assert ormsgpack.packb(
        [
            datetime.datetime(
                2018,
                12,
                1,
                2,
                3,
                4,
                0,
                tzinfo=timezone("Australia/Adelaide"),
            )
        ]
    ) == msgpack.packb(["2018-12-01T02:03:04+10:30"])


@pytest.mark.parametrize("timezone", TIMEZONE_PARAMS)
def test_datetime_partial_second(timezone):
    """
    datetime.datetime UTC offset round seconds

    https://tools.ietf.org/html/rfc3339#section-5.8
    """
    assert ormsgpack.packb(
        [
            datetime.datetime(
                1937,
                1,
                1,
                12,
                0,
                27,
                87,
                tzinfo=timezone("Europe/Amsterdam"),
            )
        ]
    ) in {
        msgpack.packb(["1937-01-01T12:00:27.000087+00:00"]),
        msgpack.packb(["1937-01-01T12:00:27.000087+00:20"]),
    }


def test_datetime_microsecond_max():
    """
    datetime.datetime microsecond max
    """
    assert ormsgpack.packb(
        datetime.datetime(2000, 1, 1, 0, 0, 0, 999999)
    ) == msgpack.packb("2000-01-01T00:00:00.999999")


def test_datetime_microsecond_min():
    """
    datetime.datetime microsecond min
    """
    assert ormsgpack.packb(datetime.datetime(2000, 1, 1, 0, 0, 0, 1)) == msgpack.packb(
        "2000-01-01T00:00:00.000001"
    )


def test_datetime_omit_microseconds():
    """
    datetime.datetime OPT_OMIT_MICROSECONDS
    """
    assert ormsgpack.packb(
        [datetime.datetime(2000, 1, 1, 2, 3, 4, 123)],
        option=ormsgpack.OPT_OMIT_MICROSECONDS,
    ) == msgpack.packb(["2000-01-01T02:03:04"])


def test_datetime_omit_microseconds_naive():
    """
    datetime.datetime naive OPT_OMIT_MICROSECONDS
    """
    assert ormsgpack.packb(
        [datetime.datetime(2000, 1, 1, 2, 3, 4, 123)],
        option=ormsgpack.OPT_NAIVE_UTC | ormsgpack.OPT_OMIT_MICROSECONDS,
    ) == msgpack.packb(["2000-01-01T02:03:04+00:00"])


def test_time_omit_microseconds():
    """
    datetime.time OPT_OMIT_MICROSECONDS
    """
    assert ormsgpack.packb(
        [datetime.time(2, 3, 4, 123)], option=ormsgpack.OPT_OMIT_MICROSECONDS
    ) == msgpack.packb(["02:03:04"])


def test_datetime_utc_z_naive_omit():
    """
    datetime.datetime naive OPT_UTC_Z
    """
    assert ormsgpack.packb(
        [datetime.datetime(2000, 1, 1, 2, 3, 4, 123)],
        option=ormsgpack.OPT_NAIVE_UTC
        | ormsgpack.OPT_UTC_Z
        | ormsgpack.OPT_OMIT_MICROSECONDS,
    ) == msgpack.packb(["2000-01-01T02:03:04Z"])


def test_datetime_utc_z_naive():
    """
    datetime.datetime naive OPT_UTC_Z
    """
    assert ormsgpack.packb(
        [datetime.datetime(2000, 1, 1, 2, 3, 4, 123)],
        option=ormsgpack.OPT_NAIVE_UTC | ormsgpack.OPT_UTC_Z,
    ) == msgpack.packb(["2000-01-01T02:03:04.000123Z"])


def test_datetime_utc_z_without_tz():
    """
    datetime.datetime naive OPT_UTC_Z
    """
    assert ormsgpack.packb(
        [datetime.datetime(2000, 1, 1, 2, 3, 4, 123)], option=ormsgpack.OPT_UTC_Z
    ) == msgpack.packb(["2000-01-01T02:03:04.000123"])


def test_datetime_utc_z_with_tz():
    """
    datetime.datetime naive OPT_UTC_Z
    """
    assert ormsgpack.packb(
        [datetime.datetime(2000, 1, 1, 0, 0, 0, 1, tzinfo=datetime.timezone.utc)],
        option=ormsgpack.OPT_UTC_Z,
    ) == msgpack.packb(["2000-01-01T00:00:00.000001Z"])

    assert ormsgpack.packb(
        [
            datetime.datetime(
                1937, 1, 1, 12, 0, 27, 87, tzinfo=tz.gettz("Europe/Amsterdam")
            )
        ],
        option=ormsgpack.OPT_UTC_Z,
    ) in {
        msgpack.packb(["1937-01-01T12:00:27.000087Z"]),
        msgpack.packb(["1937-01-01T12:00:27.000087+00:20"]),
    }


def test_datetime_roundtrip():
    """
    datetime.datetime parsed by pendulum
    """
    obj = datetime.datetime(2000, 1, 1, 0, 0, 0, 1, tzinfo=datetime.timezone.utc)
    deserialized = ormsgpack.unpackb(ormsgpack.packb(obj))
    parsed = pendulum.parse(deserialized)
    for attr in ("year", "month", "day", "hour", "minute", "second", "microsecond"):
        assert getattr(obj, attr) == getattr(parsed, attr)


def test_date():
    """
    datetime.date
    """
    assert ormsgpack.packb([datetime.date(2000, 1, 13)]) == msgpack.packb(
        ["2000-01-13"]
    )


def test_date_min():
    """
    datetime.date MINYEAR
    """
    assert ormsgpack.packb([datetime.date(datetime.MINYEAR, 1, 1)]) == msgpack.packb(
        ["0001-01-01"]
    )


def test_date_max():
    """
    datetime.date MAXYEAR
    """
    assert ormsgpack.packb([datetime.date(datetime.MAXYEAR, 12, 31)]) == msgpack.packb(
        ["9999-12-31"]
    )


def test_date_three_digits():
    """
    datetime.date three digit year
    """
    assert ormsgpack.packb(
        [datetime.date(312, 1, 1)],
    ) == msgpack.packb(["0312-01-01"])


def test_date_two_digits():
    """
    datetime.date two digit year
    """
    assert ormsgpack.packb(
        [datetime.date(46, 1, 1)],
    ) == msgpack.packb(["0046-01-01"])


def test_time():
    """
    datetime.time
    """
    assert ormsgpack.packb([datetime.time(12, 15, 59, 111)]) == msgpack.packb(
        ["12:15:59.000111"]
    )
    assert ormsgpack.packb([datetime.time(12, 15, 59)]) == msgpack.packb(["12:15:59"])


def test_time_tz():
    """
    datetime.time with tzinfo error
    """
    with pytest.raises(ormsgpack.MsgpackEncodeError):
        ormsgpack.packb(
            [datetime.time(12, 15, 59, 111, tzinfo=tz.gettz("Asia/Shanghai"))]
        )


def test_time_microsecond_max():
    """
    datetime.time microsecond max
    """
    assert ormsgpack.packb(datetime.time(0, 0, 0, 999999)) == msgpack.packb(
        "00:00:00.999999"
    )


def test_time_microsecond_min():
    """
    datetime.time microsecond min
    """
    assert ormsgpack.packb(datetime.time(0, 0, 0, 1)) == msgpack.packb(
        "00:00:00.000001"
    )


def test_passthrough_datetime():
    with pytest.raises(ormsgpack.MsgpackEncodeError):
        ormsgpack.packb(
            datetime.datetime(1970, 1, 1), option=ormsgpack.OPT_PASSTHROUGH_DATETIME
        )


def test_passthrough_date():
    with pytest.raises(ormsgpack.MsgpackEncodeError):
        ormsgpack.packb(
            datetime.date(1970, 1, 1), option=ormsgpack.OPT_PASSTHROUGH_DATETIME
        )


def test_passthrough_time():
    with pytest.raises(ormsgpack.MsgpackEncodeError):
        ormsgpack.packb(
            datetime.time(12, 0, 0), option=ormsgpack.OPT_PASSTHROUGH_DATETIME
        )


def test_passthrough_datetime_default():
    def default(obj):
        return obj.strftime("%a, %d %b %Y %H:%M:%S GMT")

    assert ormsgpack.packb(
        datetime.datetime(1970, 1, 1),
        option=ormsgpack.OPT_PASSTHROUGH_DATETIME,
        default=default,
    ) == msgpack.packb("Thu, 01 Jan 1970 00:00:00 GMT")
