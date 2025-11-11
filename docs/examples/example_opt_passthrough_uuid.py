import ormsgpack, uuid
def default(obj):
    if isinstance(obj, uuid.UUID):
        return ormsgpack.Ext(0, obj.bytes)
    raise TypeError

ormsgpack.packb(
    uuid.UUID("886313e1-3b8a-5372-9b90-0c9aee199e5d"),
)
ormsgpack.packb(
    uuid.UUID("886313e1-3b8a-5372-9b90-0c9aee199e5d"),
    option=ormsgpack.OPT_PASSTHROUGH_UUID,
)
ormsgpack.packb(
    uuid.UUID("886313e1-3b8a-5372-9b90-0c9aee199e5d"),
    option=ormsgpack.OPT_PASSTHROUGH_UUID,
    default=default,
)
