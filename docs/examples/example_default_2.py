import ormsgpack, decimal
def default(obj):
    if isinstance(obj, decimal.Decimal):
        return ormsgpack.Ext(0, str(obj).encode())

ormsgpack.packb({1, 2}, default=default)
print(ormsgpack.unpackb(_))
