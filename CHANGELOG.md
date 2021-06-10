# Changelog
## Next Version

### Fixed
- `orjson.packb` with `option` argument as `ormsgpack.OPT_NON_STR_KEYS` serializes bytes key into tuple of integers
    instead of using bin type. This also resulted in assymetrical packb/unpackb.
## 0.1.0 - 9/6/2021

First version, changed orjson to ormsgpack.
