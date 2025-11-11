import ormsgpack
ormsgpack.packb({"a": 1, "ä": 2, "A": 3}, option=ormsgpack.OPT_SORT_KEYS)
