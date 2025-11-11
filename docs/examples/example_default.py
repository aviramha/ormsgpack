import ormsgpack, decimal
def default(obj):
    if isinstance(obj, decimal.Decimal):
        return ormsgpack.Ext(0, str(obj).encode())
    raise TypeError

ormsgpack.packb(decimal.Decimal("3.14"))
ormsgpack.packb(decimal.Decimal("3.14"), default=default)
ormsgpack.packb({1, 2}, default=default)
