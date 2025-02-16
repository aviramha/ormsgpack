# SPDX-License-Identifier: (Apache-2.0 OR MIT)

import uuid

import pytest

import ormsgpack


def test_uuid_subclass() -> None:
    """
    UUID subclasses are not serialized
    """

    class AUUID(uuid.UUID):
        pass

    with pytest.raises(ormsgpack.MsgpackEncodeError):
        ormsgpack.packb(AUUID("{12345678-1234-5678-1234-567812345678}"))


def test_nil_uuid() -> None:
    assert (
        ormsgpack.unpackb(
            ormsgpack.packb(uuid.UUID("00000000-0000-0000-0000-000000000000"))
        )
        == "00000000-0000-0000-0000-000000000000"
    )


def test_all_ways_to_create_uuid_behave_equivalently() -> None:
    # Note that according to the docstring for the uuid.UUID class, all the
    # forms below are equivalent -- they end up with the same value for
    # `self.int`, which is all that really matters
    uuids = [
        uuid.UUID("{12345678-1234-5678-1234-567812345678}"),
        uuid.UUID("12345678123456781234567812345678"),
        uuid.UUID("urn:uuid:12345678-1234-5678-1234-567812345678"),
        uuid.UUID(bytes=b"\x12\x34\x56\x78" * 4),
        uuid.UUID(
            bytes_le=b"\x78\x56\x34\x12\x34\x12\x78\x56"
            + b"\x12\x34\x56\x78\x12\x34\x56\x78"
        ),
        uuid.UUID(fields=(0x12345678, 0x1234, 0x5678, 0x12, 0x34, 0x567812345678)),
        uuid.UUID(int=0x12345678123456781234567812345678),
    ]
    result = ormsgpack.unpackb(ormsgpack.packb(uuids))
    packed = [str(u) for u in uuids]
    assert packed == result


def test_serializes_correctly_with_leading_zeroes() -> None:
    instance = uuid.UUID(int=0x00345678123456781234567812345678)
    assert ormsgpack.unpackb(ormsgpack.packb(instance)) == str(instance)


def test_all_uuid_creation_functions_create_serializable_uuids() -> None:
    uuids = (
        uuid.uuid1(),
        uuid.uuid3(uuid.NAMESPACE_DNS, "python.org"),
        uuid.uuid4(),
        uuid.uuid5(uuid.NAMESPACE_DNS, "python.org"),
    )
    for val in uuids:
        assert ormsgpack.unpackb(ormsgpack.packb(val)) == str(val)


def test_uuid_passthrough() -> None:
    obj = uuid.uuid4()
    with pytest.raises(ormsgpack.MsgpackEncodeError):
        ormsgpack.packb(obj, option=ormsgpack.OPT_PASSTHROUGH_UUID)


def test_uuid_passthrough_default() -> None:
    obj = uuid.uuid4()
    assert ormsgpack.packb(
        obj, option=ormsgpack.OPT_PASSTHROUGH_UUID, default=str
    ) == ormsgpack.packb(str(obj))
