# SPDX-License-Identifier: (Apache-2.0 OR MIT)

import msgpack
import pytest

import ormsgpack


def test_tuple_passthrough() -> None:
    obj = (1, 2)
    with pytest.raises(ormsgpack.MsgpackEncodeError):
        ormsgpack.packb(obj, option=ormsgpack.OPT_PASSTHROUGH_TUPLE)


def test_tuple_passthrough_default() -> None:
    obj = (1, 2)
    assert ormsgpack.packb(
        obj, option=ormsgpack.OPT_PASSTHROUGH_TUPLE, default=list
    ) == msgpack.packb(obj)
