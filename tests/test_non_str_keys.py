# SPDX-License-Identifier: (Apache-2.0 OR MIT)

import dataclasses
import datetime

import msgpack
import pytest
import pytz

import ormsgpack


class SubStr(str):
    pass


def test_dict_keys_substr_passthrough():
    """
    OPT_PASSTHROUGH_SUBCLASS does not affect OPT_NON_STR_KEYS
    """
    assert ormsgpack.packb(
        {SubStr("aaa"): True},
        option=ormsgpack.OPT_NON_STR_KEYS | ormsgpack.OPT_PASSTHROUGH_SUBCLASS,
    ) == msgpack.packb({"aaa": True})


def test_dict_keys_datetime_passthrough():
    """
    OPT_PASSTHROUGH_DATETIME does not affect OPT_NON_STR_KEYS
    """
    assert ormsgpack.packb(
        {datetime.datetime(2000, 1, 1, 2, 3, 4, 123): True},
        option=ormsgpack.OPT_NON_STR_KEYS | ormsgpack.OPT_PASSTHROUGH_DATETIME,
    ) == msgpack.packb({"2000-01-01T02:03:04.000123": True})


def test_dict_non_str_and_sort_keys():
    with pytest.raises(ormsgpack.MsgpackEncodeError):
        ormsgpack.packb(
            {
                datetime.date(1970, 1, 3): 3,
                datetime.date(1970, 1, 5): 2,
                "other": 1,
            },
            option=ormsgpack.OPT_NON_STR_KEYS | ormsgpack.OPT_SORT_KEYS,
        )


def test_dict_keys_time_err():
    """
    OPT_NON_STR_KEYS propagates errors in types
    """
    val = datetime.time(12, 15, 59, 111, tzinfo=pytz.timezone("Asia/Shanghai"))
    with pytest.raises(ormsgpack.MsgpackEncodeError):
        ormsgpack.packb({val: True}, option=ormsgpack.OPT_NON_STR_KEYS)


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


def test_dict_keys_unknown():
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
