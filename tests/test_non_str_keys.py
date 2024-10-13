# SPDX-License-Identifier: (Apache-2.0 OR MIT)

import dataclasses
import datetime

import msgpack
import pytest
import pytz

import ormsgpack


class SubStr(str):
    pass


def test_dict_keys_substr_passthrough() -> None:
    """
    OPT_PASSTHROUGH_SUBCLASS does not affect OPT_NON_STR_KEYS
    """
    assert ormsgpack.packb(
        {SubStr("aaa"): True},
        option=ormsgpack.OPT_NON_STR_KEYS | ormsgpack.OPT_PASSTHROUGH_SUBCLASS,
    ) == msgpack.packb({"aaa": True})


def test_dict_keys_datetime_passthrough() -> None:
    """
    OPT_PASSTHROUGH_DATETIME does not affect OPT_NON_STR_KEYS
    """
    assert ormsgpack.packb(
        {datetime.datetime(2000, 1, 1, 2, 3, 4, 123): True},
        option=ormsgpack.OPT_NON_STR_KEYS | ormsgpack.OPT_PASSTHROUGH_DATETIME,
    ) == msgpack.packb({"2000-01-01T02:03:04.000123": True})


def test_dict_keys_tuple_passthrough() -> None:
    """
    OPT_PASSTHROUGH_TUPLE does not affect OPT_NON_STR_KEYS
    """
    obj = {(1, 2): True}
    assert ormsgpack.packb(
        obj, option=ormsgpack.OPT_NON_STR_KEYS | ormsgpack.OPT_PASSTHROUGH_TUPLE
    ) == msgpack.packb(obj)


def test_dict_non_str_and_sort_keys() -> None:
    with pytest.raises(ormsgpack.MsgpackEncodeError):
        ormsgpack.packb(
            {
                datetime.date(1970, 1, 3): 3,
                datetime.date(1970, 1, 5): 2,
                "other": 1,
            },
            option=ormsgpack.OPT_NON_STR_KEYS | ormsgpack.OPT_SORT_KEYS,
        )


def test_dict_keys_time_err() -> None:
    """
    OPT_NON_STR_KEYS propagates errors in types
    """
    val = datetime.time(12, 15, 59, 111, tzinfo=pytz.timezone("Asia/Shanghai"))
    with pytest.raises(ormsgpack.MsgpackEncodeError):
        ormsgpack.packb({val: True}, option=ormsgpack.OPT_NON_STR_KEYS)


def test_dict_keys_dataclass() -> None:
    @dataclasses.dataclass(frozen=True)
    class Dataclass:
        a: str

    obj = {Dataclass("a"): True}
    with pytest.raises(ormsgpack.MsgpackEncodeError):
        ormsgpack.packb(obj, option=ormsgpack.OPT_NON_STR_KEYS)


def test_dict_keys_unknown() -> None:
    with pytest.raises(ormsgpack.MsgpackEncodeError):
        ormsgpack.packb({frozenset(): True}, option=ormsgpack.OPT_NON_STR_KEYS)
