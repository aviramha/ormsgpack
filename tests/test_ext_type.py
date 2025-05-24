import msgpack
import pytest

import ormsgpack


@pytest.mark.parametrize(
    "data",
    (
        pytest.param(b"a" * 1, id="fixext 1"),
        pytest.param(b"a" * 2, id="fixext 2"),
        pytest.param(b"a" * 4, id="fixext 4"),
        pytest.param(b"a" * 8, id="fixext 8"),
        pytest.param(b"a" * 16, id="fixext 16"),
        pytest.param(b"a" * 32, id="ext 8"),
        pytest.param(b"a" * 256, id="ext 16"),
        pytest.param(b"a" * 65536, id="ext 32"),
    ),
)
def test_ext_type(data: bytes) -> None:
    tag = 1
    value = ormsgpack.Ext(tag, data)
    packed = ormsgpack.packb(value)
    assert packed == msgpack.packb(msgpack.ExtType(tag, data))

    unpacked = ormsgpack.unpackb(
        packed,
        ext_hook=lambda x, y: (x, y),
    )
    assert unpacked == (tag, data)

    with pytest.raises(ormsgpack.MsgpackDecodeError):
        ormsgpack.unpackb(packed)

    with pytest.raises(ormsgpack.MsgpackEncodeError):
        ormsgpack.packb({value: True}, option=ormsgpack.OPT_NON_STR_KEYS)
