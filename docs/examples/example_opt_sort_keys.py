import ormsgpack
ormsgpack.packb({"b": 1, "c": 2, "a": 3})
ormsgpack.packb({"b": 1, "c": 2, "a": 3}, option=ormsgpack.OPT_SORT_KEYS)
