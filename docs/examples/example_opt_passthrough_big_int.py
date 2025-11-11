import ormsgpack
def default(obj):
    if isinstance(obj, int):
        return ormsgpack.Ext(0, obj.to_bytes(9))
    raise TypeError

ormsgpack.packb(2**65)
ormsgpack.packb(
    2**65,
    option=ormsgpack.OPT_PASSTHROUGH_BIG_INT,
    default=default,
)
