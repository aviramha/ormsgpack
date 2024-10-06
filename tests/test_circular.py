# SPDX-License-Identifier: (Apache-2.0 OR MIT)

from typing import Dict, List

import pytest

import ormsgpack


def test_circular_dict() -> None:
    """
    packb() circular reference dict
    """
    obj: Dict[str, object] = {}
    obj["obj"] = obj
    with pytest.raises(ormsgpack.MsgpackEncodeError):
        ormsgpack.packb(obj)


def test_circular_list() -> None:
    """
    packb() circular reference list
    """
    obj: List[object] = []
    obj.append(obj)
    with pytest.raises(ormsgpack.MsgpackEncodeError):
        ormsgpack.packb(obj)


def test_circular_nested() -> None:
    """
    packb() circular reference nested dict, list
    """
    obj: Dict[str, object] = {}
    obj["list"] = [{"obj": obj}]
    with pytest.raises(ormsgpack.MsgpackEncodeError):
        ormsgpack.packb(obj)
