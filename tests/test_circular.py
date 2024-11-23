# SPDX-License-Identifier: (Apache-2.0 OR MIT)

import pytest

import ormsgpack


def test_circular_dict() -> None:
    """
    packb() circular reference dict
    """
    obj: dict[str, object] = {}
    obj["obj"] = obj
    with pytest.raises(ormsgpack.MsgpackEncodeError):
        ormsgpack.packb(obj)


def test_circular_list() -> None:
    """
    packb() circular reference list
    """
    obj: list[object] = []
    obj.append(obj)
    with pytest.raises(ormsgpack.MsgpackEncodeError):
        ormsgpack.packb(obj)


def test_circular_nested() -> None:
    """
    packb() circular reference nested dict, list
    """
    obj: dict[str, object] = {}
    obj["list"] = [{"obj": obj}]
    with pytest.raises(ormsgpack.MsgpackEncodeError):
        ormsgpack.packb(obj)
