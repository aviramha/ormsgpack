import ormsgpack, uuid
ormsgpack.packb(uuid.uuid5(uuid.NAMESPACE_DNS, "python.org"))
ormsgpack.unpackb(_)
