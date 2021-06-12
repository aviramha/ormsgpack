# Changelog
## Next Version
### Misc
- Grouped benchmarks in a pattern that should make more sense.
- Added pydantic docs to `README.md`
- Added graphs and benchmark results.
## 0.2.1 - 12/6/2021
### Fixed
- Depth limit is now enforced for `ormsgpack.unpackb` - function should be safe for use now.
### Removed
- Removed `OPT_SERIALIZE_UUID` from ormsgpack.pyi as it doesn't exist.
### Misc
- Added `scripts/test.sh` for running tests.
- Added benchmarks, modified scripts to match new layout.
## 0.2.0 - 10/6/2021
### Added
- Add support for serializing pydantic's `BaseModel` instances using `ormsgpack.OPT_SERIALIZE_PYDANTIC`.
### Fixed
- `orjson.packb` with `option` argument as `ormsgpack.OPT_NON_STR_KEYS` serializes bytes key into tuple of integers
    instead of using bin type. This also resulted in assymetrical packb/unpackb.
### Misc
- Added `--no-index` to `pip install ormsgpack` to avoid installing from PyPI on CI.
## 0.1.0 - 9/6/2021

First version, changed orjson to ormsgpack.
