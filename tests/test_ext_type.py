import msgpack
import pytest

import ormsgpack


def test_ext_type():
    tag = 1
    data = b"test"
    packed = ormsgpack.packb(ormsgpack.Ext(tag, data))
    assert packed == msgpack.packb(msgpack.ExtType(tag, data))

    unpacked = ormsgpack.unpackb(
        packed,
        ext_hook=lambda x, y: (x, y),
    )
    assert unpacked == (tag, data)

    unpacked = ormsgpack.unpackb(
        packed,
        ext_hook=lambda x, y: (x, y),
        option=ormsgpack.OPT_NON_STR_KEYS,
    )
    assert unpacked == (tag, data)

    with pytest.raises(ormsgpack.MsgpackDecodeError):
        ormsgpack.unpackb(packed)
