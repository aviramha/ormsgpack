import ormsgpack, decimal
def ext_hook(tag, data):
    if tag == 0:
        return decimal.Decimal(data.decode())
    raise TypeError

ormsgpack.packb(ormsgpack.Ext(0, str(decimal.Decimal("3.14")).encode()))
ormsgpack.unpackb(_, ext_hook=ext_hook)
