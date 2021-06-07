# SPDX-License-Identifier: (Apache-2.0 OR MIT)

import dataclasses
import datetime
import uuid

import msgpack
import pytest
import pytz

import ormsgpack


class SubStr(str):
    pass


def test_dict_keys_substr():
    assert ormsgpack.packb(
        {SubStr("aaa"): True}, option=ormsgpack.OPT_NON_STR_KEYS
    ) == msgpack.packb({"aaa": True})


def test_dict_keys_substr_passthrough():
    """
    OPT_PASSTHROUGH_SUBCLASS does not affect OPT_NON_STR_KEYS
    """
    assert (
        ormsgpack.packb(
            {SubStr("aaa"): True},
            option=ormsgpack.OPT_NON_STR_KEYS | ormsgpack.OPT_PASSTHROUGH_SUBCLASS,
        )
        == msgpack.packb({"aaa": True})
    )


def test_dict_keys_int_range_valid_i64():
    """
    OPT_NON_STR_KEYS has a i64 range for int, valid
    """
    assert (
        ormsgpack.packb(
            {9223372036854775807: True},
            option=ormsgpack.OPT_NON_STR_KEYS,
        )
        == msgpack.packb({9223372036854775807: True})
    )
    assert (
        ormsgpack.packb(
            {-9223372036854775807: True},
            option=ormsgpack.OPT_NON_STR_KEYS,
        )
        == msgpack.packb({-9223372036854775807: True})
    )
    assert (
        ormsgpack.packb(
            {9223372036854775809: True},
            option=ormsgpack.OPT_NON_STR_KEYS,
        )
        == msgpack.packb({9223372036854775809: True})
    )


def test_dict_keys_int_range_valid_u64():
    """
    OPT_NON_STR_KEYS has a u64 range for int, valid
    """
    assert (
        ormsgpack.packb(
            {0: True},
            option=ormsgpack.OPT_NON_STR_KEYS,
        )
        == msgpack.packb({0: True})
    )

    assert (
        ormsgpack.packb(
            {18446744073709551615: True},
            option=ormsgpack.OPT_NON_STR_KEYS,
        )
        == msgpack.packb({18446744073709551615: True})
    )


def test_dict_keys_int_range_invalid():
    """
    OPT_NON_STR_KEYS has a range of i64::MIN to u64::MAX
    """
    with pytest.raises(ormsgpack.MsgpackEncodeError):
        ormsgpack.packb({-9223372036854775809: True}, option=ormsgpack.OPT_NON_STR_KEYS)
    with pytest.raises(ormsgpack.MsgpackEncodeError):
        ormsgpack.packb({18446744073709551616: True}, option=ormsgpack.OPT_NON_STR_KEYS)


def test_dict_keys_float():
    assert ormsgpack.packb(
        {1.1: True, 2.2: False}, option=ormsgpack.OPT_NON_STR_KEYS
    ) == msgpack.packb({1.1: True, 2.2: False})


def test_dict_keys_inf():
    assert ormsgpack.packb(
        {float("Infinity"): True}, option=ormsgpack.OPT_NON_STR_KEYS
    ) == msgpack.packb({float("Infinity"): True})
    assert ormsgpack.packb(
        {float("-Infinity"): True}, option=ormsgpack.OPT_NON_STR_KEYS
    ) == msgpack.packb({float("-Infinity"): True})


def test_dict_keys_nan():
    assert ormsgpack.packb(
        {float("NaN"): True}, option=ormsgpack.OPT_NON_STR_KEYS
    ) == msgpack.packb({float("NaN"): True})


def test_dict_keys_bool():
    assert ormsgpack.packb(
        {True: True, False: False}, option=ormsgpack.OPT_NON_STR_KEYS
    ) == msgpack.packb({True: True, False: False})


def test_dict_keys_datetime():
    assert (
        ormsgpack.packb(
            {datetime.datetime(2000, 1, 1, 2, 3, 4, 123): True},
            option=ormsgpack.OPT_NON_STR_KEYS,
        )
        == msgpack.packb({"2000-01-01T02:03:04.000123": True})
    )


def test_dict_keys_datetime_opt():
    assert (
        ormsgpack.packb(
            {datetime.datetime(2000, 1, 1, 2, 3, 4, 123): True},
            option=ormsgpack.OPT_NON_STR_KEYS
            | ormsgpack.OPT_OMIT_MICROSECONDS
            | ormsgpack.OPT_NAIVE_UTC
            | ormsgpack.OPT_UTC_Z,
        )
        == msgpack.packb({"2000-01-01T02:03:04Z": True})
    )


def test_dict_keys_datetime_passthrough():
    """
    OPT_PASSTHROUGH_DATETIME does not affect OPT_NON_STR_KEYS
    """
    assert (
        ormsgpack.packb(
            {datetime.datetime(2000, 1, 1, 2, 3, 4, 123): True},
            option=ormsgpack.OPT_NON_STR_KEYS | ormsgpack.OPT_PASSTHROUGH_DATETIME,
        )
        == msgpack.packb({"2000-01-01T02:03:04.000123": True})
    )


def test_dict_keys_uuid():
    """
    OPT_NON_STR_KEYS always serializes UUID as keys
    """
    assert (
        ormsgpack.packb(
            {uuid.UUID("7202d115-7ff3-4c81-a7c1-2a1f067b1ece"): True},
            option=ormsgpack.OPT_NON_STR_KEYS,
        )
        == msgpack.packb({"7202d115-7ff3-4c81-a7c1-2a1f067b1ece": True})
    )


def test_dict_keys_date():
    assert ormsgpack.packb(
        {datetime.date(1970, 1, 1): True}, option=ormsgpack.OPT_NON_STR_KEYS
    ) == msgpack.packb({"1970-01-01": True})


def test_dict_keys_time():
    assert (
        ormsgpack.packb(
            {datetime.time(12, 15, 59, 111): True},
            option=ormsgpack.OPT_NON_STR_KEYS,
        )
        == msgpack.packb({"12:15:59.000111": True})
    )


def test_dict_non_str_and_sort_keys():
    assert (
        ormsgpack.packb(
            {
                datetime.date(1970, 1, 3): 3,
                datetime.date(1970, 1, 5): 2,
                "other": 1,
            },
            option=ormsgpack.OPT_NON_STR_KEYS,
        )
        == msgpack.packb({"1970-01-03": 3, "1970-01-05": 2, "other": 1})
    )


def test_dict_keys_time_err():
    """
    OPT_NON_STR_KEYS propagates errors in types
    """
    val = datetime.time(12, 15, 59, 111, tzinfo=pytz.timezone("Asia/Shanghai"))
    with pytest.raises(ormsgpack.MsgpackEncodeError):
        ormsgpack.packb({val: True}, option=ormsgpack.OPT_NON_STR_KEYS)


def test_dict_keys_str():
    assert ormsgpack.packb(
        {"1": True}, option=ormsgpack.OPT_NON_STR_KEYS
    ) == msgpack.packb({"1": True})


def test_dict_keys_type():
    class Obj:
        a: str

    val = Obj()
    with pytest.raises(ormsgpack.MsgpackEncodeError):
        ormsgpack.packb({val: True}, option=ormsgpack.OPT_NON_STR_KEYS)


def test_dict_keys_dataclass_hash():
    @dataclasses.dataclass
    class Dataclass:
        a: str

        def __hash__(self):
            return 1

    obj = {Dataclass("a"): True}
    with pytest.raises(ormsgpack.MsgpackEncodeError):
        ormsgpack.packb(obj, option=ormsgpack.OPT_NON_STR_KEYS)


def test_dict_keys_tuple():
    obj = {(): True}
    with pytest.raises(ormsgpack.MsgpackEncodeError):
        ormsgpack.packb(obj, option=ormsgpack.OPT_NON_STR_KEYS)


def test_dict_keys_unknown():
    obj = {frozenset(): True}
    with pytest.raises(ormsgpack.MsgpackEncodeError):
        ormsgpack.packb({frozenset(): True}, option=ormsgpack.OPT_NON_STR_KEYS)


def test_dict_keys_no_str_call():
    class Obj:
        a: str

        def __str__(self):
            return "Obj"

    val = Obj()
    with pytest.raises(ormsgpack.MsgpackEncodeError):
        ormsgpack.packb({val: True}, option=ormsgpack.OPT_NON_STR_KEYS)
