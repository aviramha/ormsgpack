# ormsgpack

ormsgpack is a fast msgpack library for Python. It is a fork/reboot of [orjson](https://github.com/ijl/orjson)
It serializes faster than [msgpack-python](https://github.com/msgpack/msgpack-python) and deserializes a bit slower (right now).
It supports serialization of:
[dataclass](https://github.com/aviramha/ormsgpack#dataclass),
[datetime](https://github.com/aviramha/ormsgpack#datetime),
[numpy](https://github.com/aviramha/ormsgpack#numpy), and
[UUID](https://github.com/aviramha/ormsgpack#uuid) instances natively.

Its features and drawbacks compared to other Python msgpack libraries:

* serializes `dataclass` instances natively.
* serializes `datetime`, `date`, and `time` instances to RFC 3339 format,
e.g., "1970-01-01T00:00:00+00:00"
* serializes `numpy.ndarray` instances natively and faster.
* serializes arbitrary types using a `default` hook

ormsgpack supports CPython 3.6, 3.7, 3.8, 3.9, and 3.10. ormsgpack does not support PyPy. Releases follow semantic
versioning and serializing a new object type without an opt-in flag is
considered a breaking change.

ormsgpack is licensed under both the Apache 2.0 and MIT licenses. The
repository and issue tracker is
[github.com/aviramha/ormsgpack](https://github.com/aviramha/ormsgpack), and patches may be
submitted there. There is a
[CHANGELOG](https://github.com/aviramha/ormsgpack/blob/master/CHANGELOG.md)
available in the repository.

1. [Usage](https://github.com/aviramha/ormsgpack#usage)
    1. [Install](https://github.com/aviramha/ormsgpack#install)
    2. [Quickstart](https://github.com/aviramha/ormsgpack#quickstart)
    4. [Serialize](https://github.com/aviramha/ormsgpack#serialize)
        1. [default](https://github.com/aviramha/ormsgpack#default)
        2. [option](https://github.com/aviramha/ormsgpack#option)
    5. [Deserialize](https://github.com/aviramha/ormsgpack#deserialize)
2. [Types](https://github.com/aviramha/ormsgpack#types)
    1. [dataclass](https://github.com/aviramha/ormsgpack#dataclass)
    2. [datetime](https://github.com/aviramha/ormsgpack#datetime)
    3. [enum](https://github.com/aviramha/ormsgpack#enum)
    4. [float](https://github.com/aviramha/ormsgpack#float)
    5. [int](https://github.com/aviramha/ormsgpack#int)
    6. [numpy](https://github.com/aviramha/ormsgpack#numpy)
    7. [uuid](https://github.com/aviramha/ormsgpack#uuid)
3. [Questions](https://github.com/aviramha/ormsgpack#questions)
4. [Packaging](https://github.com/aviramha/ormsgpack#packaging)
5. [License](https://github.com/aviramha/ormsgpack#license)

## Usage

### Install

To install a wheel from PyPI:

```sh
pip install --upgrade "pip>=19.3" # manylinux2014 support
pip install --upgrade ormsgpack
```

Notice that Linux environments with a `pip` version shipped in 2018 or earlier
must first upgrade `pip` to support `manylinux2014` wheels.

To build a wheel, see [packaging](https://github.com/aviramha/ormsgpack#packaging).

### Quickstart

This is an example of serializing, with options specified, and deserializing:

```python
>>> import ormsgpack, datetime, numpy
>>> data = {
    "type": "job",
    "created_at": datetime.datetime(1970, 1, 1),
    "status": "🆗",
    "payload": numpy.array([[1, 2], [3, 4]]),
}
>>> ormsgpack.packb(data, option=ormsgpack.OPT_NAIVE_UTC | ormsgpack.OPT_SERIALIZE_NUMPY)
b'\x84\xa4type\xa3job\xaacreated_at\xb91970-01-01T00:00:00+00:00\xa6status\xa4\xf0\x9f\x86\x97\xa7payload\x92\x92\x01\x02\x92\x03\x04'
>>> ormsgpack.unpackb(_)
{'type': 'job', 'created_at': '1970-01-01T00:00:00+00:00', 'status': '🆗', 'payload': [[1, 2], [3, 4]]}
```

### Serialize

```python
def packb(
    __obj: Any,
    default: Optional[Callable[[Any], Any]] = ...,
    option: Optional[int] = ...,
) -> bytes: ...
```

`packb()` serializes Python objects to msgpack.

It natively serializes
`bytes`, `str`, `dict`, `list`, `tuple`, `int`, `float`, `bool`,
`dataclasses.dataclass`, `typing.TypedDict`, `datetime.datetime`,
`datetime.date`, `datetime.time`, `uuid.UUID`, `numpy.ndarray`, and
`None` instances. It supports arbitrary types through `default`. It
serializes subclasses of `str`, `int`, `dict`, `list`,
`dataclasses.dataclass`, and `enum.Enum`. It does not serialize subclasses
of `tuple` to avoid serializing `namedtuple` objects as arrays. To avoid
serializing subclasses, specify the option `ormsgpack.OPT_PASSTHROUGH_SUBCLASS`.

The output is a `bytes` object containing UTF-8.

The global interpreter lock (GIL) is held for the duration of the call.

It raises `MsgpackEncodeError` on an unsupported type. This exception message
describes the invalid object with the error message
`Type is not JSON serializable: ...`. To fix this, specify
[default](https://github.com/aviramha/ormsgpack#default).

It raises `MsgpackEncodeError` on a `str` that contains invalid UTF-8.

It raises `MsgpackEncodeError` if a `dict` has a key of a type other than `str` or `bytes`,
unless `OPT_NON_STR_KEYS` is specified.

It raises `MsgpackEncodeError` if the output of `default` recurses to handling by
`default` more than 254 levels deep.

It raises `MsgpackEncodeError` on circular references.

It raises `MsgpackEncodeError`  if a `tzinfo` on a datetime object is
unsupported.

`MsgpackEncodeError` is a subclass of `TypeError`. This is for compatibility
with the standard library.

#### default

To serialize a subclass or arbitrary types, specify `default` as a
callable that returns a supported type. `default` may be a function,
lambda, or callable class instance. To specify that a type was not
handled by `default`, raise an exception such as `TypeError`.

```python
>>> import ormsgpack, decimal
>>>
def default(obj):
    if isinstance(obj, decimal.Decimal):
        return str(obj)
    raise TypeError

>>> ormsgpack.packb(decimal.Decimal("0.0842389659712649442845"))
MsgpackEncodeError: Type is not JSON serializable: decimal.Decimal
>>> ormsgpack.packb(decimal.Decimal("0.0842389659712649442845"), default=default)
b'\xb80.0842389659712649442845'
>>> ormsgpack.packb({1, 2}, default=default)
ormsgpack.MsgpackEncodeError: Type is not msgpack serializable: set
```

The `default` callable may return an object that itself
must be handled by `default` up to 254 times before an exception
is raised.

It is important that `default` raise an exception if a type cannot be handled.
Python otherwise implicitly returns `None`, which appears to the caller
like a legitimate value and is serialized:

```python
>>> import ormsgpack, json, rapidjson
>>>
def default(obj):
    if isinstance(obj, decimal.Decimal):
        return str(obj)

>>> ormsgpack.unpackb(ormsgpack.packb({"set":{1, 2}}, default=default))
{'set': None}
```

#### option

To modify how data is serialized, specify `option`. Each `option` is an integer
constant in `ormspgack`. To specify multiple options, mask them together, e.g.,
`option=ormspgack.OPT_NON_STR_KEYS | ormspgack.OPT_NAIVE_UTC`.

##### OPT_NAIVE_UTC

Serialize `datetime.datetime` objects without a `tzinfo` as UTC. This
has no effect on `datetime.datetime` objects that have `tzinfo` set.

```python
>>> import ormsgpack, datetime
>>> ormsgpack.unpackb(ormsgpack.packb(
        datetime.datetime(1970, 1, 1, 0, 0, 0),
    ))
"1970-01-01T00:00:00"
>>> ormsgpack.unpackb(ormsgpack.packb(
        datetime.datetime(1970, 1, 1, 0, 0, 0),
        option=ormsgpack.OPT_NAIVE_UTC,
    ))
"1970-01-01T00:00:00+00:00"
```

##### OPT_NON_STR_KEYS

Serialize `dict` keys of type other than `str`. This allows `dict` keys
to be one of `str`, `int`, `float`, `bool`, `None`, `datetime.datetime`,
`datetime.date`, `datetime.time`, `enum.Enum`, and `uuid.UUID`. For comparison,
the standard library serializes `str`, `int`, `float`, `bool` or `None` by
default.

```python
>>> import ormsgpack, datetime, uuid
>>> ormsgpack.packb(
        {uuid.UUID("7202d115-7ff3-4c81-a7c1-2a1f067b1ece"): [1, 2, 3]},
        option=ormsgpack.OPT_NON_STR_KEYS,
    )
>>> ormsgpack.packb(
        {datetime.datetime(1970, 1, 1, 0, 0, 0): [1, 2, 3]},
        option=ormsgpack.OPT_NON_STR_KEYS | ormsgpack.OPT_NAIVE_UTC,
    )
```

These types are generally serialized how they would be as
values, e.g., `datetime.datetime` is still an RFC 3339 string and respects
options affecting it.

This option has the risk of creating duplicate keys. This is because non-`str`
objects may serialize to the same `str` as an existing key, e.g.,
`{"1970-01-01T00:00:00+00:00": true, datetime.datetime(1970, 1, 1, 0, 0, 0): false}`.
The last key to be inserted to the `dict` will be serialized last and a msgpack deserializer will presumably take the last
occurrence of a key (in the above, `false`). The first value will be lost.

##### OPT_OMIT_MICROSECONDS

Do not serialize the `microsecond` field on `datetime.datetime` and
`datetime.time` instances.

```python
>>> import ormsgpack, datetime
>>> ormsgpack.packb(
        datetime.datetime(1970, 1, 1, 0, 0, 0, 1),
    )
>>> ormsgpack.packb(
        datetime.datetime(1970, 1, 1, 0, 0, 0, 1),
        option=ormsgpack.OPT_OMIT_MICROSECONDS,
    )
```

##### OPT_PASSTHROUGH_DATACLASS

Passthrough `dataclasses.dataclass` instances to `default`. This allows
customizing their output but is much slower.


```python
>>> import ormsgpack, dataclasses
>>>
@dataclasses.dataclass
class User:
    id: str
    name: str
    password: str

def default(obj):
    if isinstance(obj, User):
        return {"id": obj.id, "name": obj.name}
    raise TypeError

>>> ormsgpack.packb(User("3b1", "asd", "zxc"))
b'\x83\xa2id\xa33b1\xa4name\xa3asd\xa8password\xa3zxc'
>>> ormsgpack.packb(User("3b1", "asd", "zxc"), option=ormsgpack.OPT_PASSTHROUGH_DATACLASS)
TypeError: Type is not msgpack serializable: User
>>> ormsgpack.packb(
        User("3b1", "asd", "zxc"),
        option=ormsgpack.OPT_PASSTHROUGH_DATACLASS,
        default=default,
    )
b'\x82\xa2id\xa33b1\xa4name\xa3asd'
```

##### OPT_PASSTHROUGH_DATETIME

Passthrough `datetime.datetime`, `datetime.date`, and `datetime.time` instances
to `default`. This allows serializing datetimes to a custom format, e.g.,
HTTP dates:

```python
>>> import ormsgpack, datetime
>>>
def default(obj):
    if isinstance(obj, datetime.datetime):
        return obj.strftime("%a, %d %b %Y %H:%M:%S GMT")
    raise TypeError

>>> ormsgpack.packb({"created_at": datetime.datetime(1970, 1, 1)})
b'\x81\xaacreated_at\xb31970-01-01T00:00:00'
>>> ormsgpack.packb({"created_at": datetime.datetime(1970, 1, 1)}, option=ormsgpack.OPT_PASSTHROUGH_DATETIME)
TypeError: Type is not msgpack serializable: datetime.datetime
>>> ormsgpack.packb(
        {"created_at": datetime.datetime(1970, 1, 1)},
        option=ormsgpack.OPT_PASSTHROUGH_DATETIME,
        default=default,
    )
b'\x81\xaacreated_at\xbdThu, 01 Jan 1970 00:00:00 GMT'
```

This does not affect datetimes in `dict` keys if using OPT_NON_STR_KEYS.

##### OPT_PASSTHROUGH_SUBCLASS

Passthrough subclasses of builtin types to `default`.

```python
>>> import ormsgpack
>>>
class Secret(str):
    pass

def default(obj):
    if isinstance(obj, Secret):
        return "******"
    raise TypeError

>>> ormsgpack.packb(Secret("zxc"))
b'\xa3zxc'
>>> ormsgpack.packb(Secret("zxc"), option=ormsgpack.OPT_PASSTHROUGH_SUBCLASS)
TypeError: Type is not msgpack serializable: Secret
>>> ormsgpack.packb(Secret("zxc"), option=ormsgpack.OPT_PASSTHROUGH_SUBCLASS, default=default)
b'\xa6******'
```

This does not affect serializing subclasses as `dict` keys if using
OPT_NON_STR_KEYS.

##### OPT_SERIALIZE_NUMPY

Serialize `numpy.ndarray` instances. For more, see
[numpy](https://github.com/aviramha/ormsgpack#numpy).

##### OPT_UTC_Z

Serialize a UTC timezone on `datetime.datetime` instances as `Z` instead
of `+00:00`.

```python
>>> import ormsgpack, datetime
>>> ormsgpack.packb(
        datetime.datetime(1970, 1, 1, 0, 0, 0, tzinfo=datetime.timezone.utc),
    )
b'"1970-01-01T00:00:00+00:00"'
>>> ormsgpack.packb(
        datetime.datetime(1970, 1, 1, 0, 0, 0, tzinfo=datetime.timezone.utc),
        option=ormsgpack.OPT_UTC_Z
    )
b'"1970-01-01T00:00:00Z"'
```

### Deserialize
**WARNING: Currently there's no recursion limit, meaning this can cause stack overflow and crash your app!
Pending fix here: https://github.com/3Hren/msgpack-rust/issues/276**
```python
def unpackb(__obj: Union[bytes, bytearray, memoryview]) -> Any: ...
```

`unpackb()` deserializes msgpack to Python objects. It deserializes to `dict`,
`list`, `int`, `float`, `str`, `bool`, `bytes` and `None` objects.

`bytes`, `bytearray`, `memoryview` input are accepted.

ormsgpack maintains a cache of map keys for the duration of the process. This
causes a net reduction in memory usage by avoiding duplicate strings. The
keys must be at most 64 bytes to be cached and 512 entries are stored.

The global interpreter lock (GIL) is held for the duration of the call.

It raises `MsgpackDecodeError` if given an invalid type or invalid
msgpack.

`MsgpackDecodeError` is a subclass of `ValueError`.

## Types

### dataclass

ormsgpack serializes instances of `dataclasses.dataclass` natively. It serializes
instances 40-50x as fast as other libraries and avoids a severe slowdown seen
in other libraries compared to serializing `dict`.

It is supported to pass all variants of dataclasses, including dataclasses
using `__slots__`, frozen dataclasses, those with optional or default
attributes, and subclasses. There is a performance benefit to not
using `__slots__`.

Dataclasses are serialized as maps, with every attribute serialized and in
the order given on class definition:

```python
>>> import dataclasses, ormsgpack, typing

@dataclasses.dataclass
class Member:
    id: int
    active: bool = dataclasses.field(default=False)

@dataclasses.dataclass
class Object:
    id: int
    name: str
    members: typing.List[Member]

>>> ormsgpack.packb(Object(1, "a", [Member(1, True), Member(2)]))
b'\x83\xa2id\x01\xa4name\xa1a\xa7members\x92\x82\xa2id\x01\xa6active\xc3\x82\xa2id\x02\xa6active\xc2'
```
Users may wish to control how dataclass instances are serialized, e.g.,
to not serialize an attribute or to change the name of an
attribute when serialized. ormsgpack may implement support using the
metadata mapping on `field` attributes,
e.g., `field(metadata={"json_serialize": False})`, if use cases are clear.

### datetime

ormsgpack serializes `datetime.datetime` objects to
[RFC 3339](https://tools.ietf.org/html/rfc3339) format,
e.g., "1970-01-01T00:00:00+00:00". This is a subset of ISO 8601 and
compatible with `isoformat()` in the standard library.

```python
>>> import ormsgpack, datetime, zoneinfo
>>> ormsgpack.packb(
    datetime.datetime(2018, 12, 1, 2, 3, 4, 9, tzinfo=zoneinfo.ZoneInfo('Australia/Adelaide'))
)
>>> ormsgpack.unpackb(_)
"2018-12-01T02:03:04.000009+10:30"
>>> ormsgpack.packb(
    datetime.datetime.fromtimestamp(4123518902).replace(tzinfo=datetime.timezone.utc)
)
>>> ormsgpack.unpackb(_)
"2100-09-01T21:55:02+00:00"
>>> ormsgpack.packb(
    datetime.datetime.fromtimestamp(4123518902)
)
>>> ormsgpack.unpackb(_)
"2100-09-01T21:55:02"
```

`datetime.datetime` supports instances with a `tzinfo` that is `None`,
`datetime.timezone.utc`, a timezone instance from the python3.9+ `zoneinfo`
module, or a timezone instance from the third-party `pendulum`, `pytz`, or
`dateutil`/`arrow` libraries.

`datetime.time` objects must not have a `tzinfo`.

```python
>>> import ormsgpack, datetime
>>> ormsgpack.packb(datetime.time(12, 0, 15, 290))
>>> ormsgpack.unpackb(_)
"12:00:15.000290"
```

`datetime.date` objects will always serialize.

```python
>>> import ormsgpack, datetime
>>> ormsgpack.packb(datetime.date(1900, 1, 2))
>>> ormsgpack.unpackb(_)
"1900-01-02"
```

Errors with `tzinfo` result in `MsgpackEncodeError` being raised.

It is faster to have ormsgpack serialize datetime objects than to do so
before calling `packb()`. If using an unsupported type such as
`pendulum.datetime`, use `default`.

To disable serialization of `datetime` objects specify the option
`ormsgpack.OPT_PASSTHROUGH_DATETIME`.

To use "Z" suffix instead of "+00:00" to indicate UTC ("Zulu") time, use the option
`ormsgpack.OPT_UTC_Z`.

To assume datetimes without timezone are UTC, se the option `ormsgpack.OPT_NAIVE_UTC`.

### enum

ormsgpack serializes enums natively. Options apply to their values.

```python
>>> import enum, datetime, ormsgpack
>>>
class DatetimeEnum(enum.Enum):
    EPOCH = datetime.datetime(1970, 1, 1, 0, 0, 0)
>>> ormsgpack.packb(DatetimeEnum.EPOCH)
>>> ormsgpack.unpackb(_)
"1970-01-01T00:00:00"
>>> ormsgpack.packb(DatetimeEnum.EPOCH, option=ormsgpack.OPT_NAIVE_UTC)
>>> ormsgpack.unpackb(_)
"1970-01-01T00:00:00+00:00"
```

Enums with members that are not supported types can be serialized using
`default`:

```python
>>> import enum, ormsgpack
>>>
class Custom:
    def __init__(self, val):
        self.val = val

def default(obj):
    if isinstance(obj, Custom):
        return obj.val
    raise TypeError

class CustomEnum(enum.Enum):
    ONE = Custom(1)

>>> ormsgpack.packb(CustomEnum.ONE, default=default)
>>> ormsgpack.unpackb(_)
1
```

### float

ormsgpack serializes and deserializes double precision floats with no loss of
precision and consistent rounding.

### int

ormsgpack serializes and deserializes 64-bit integers by default. The range
supported is a signed 64-bit integer's minimum (-9223372036854775807) to
an unsigned 64-bit integer's maximum (18446744073709551615).

### numpy

ormsgpack natively serializes `numpy.ndarray` and individual `numpy.float64`,
`numpy.float32`, `numpy.int64`, `numpy.int32`, `numpy.int8`, `numpy.uint64`,
`numpy.uint32`, and `numpy.uint8` instances. Arrays may have a
`dtype` of `numpy.bool`, `numpy.float32`, `numpy.float64`, `numpy.int32`,
`numpy.int64`, `numpy.uint32`, `numpy.uint64`, `numpy.uintp`, or `numpy.intp`.
ormsgpack is faster than all compared libraries at serializing
numpy instances. Serializing numpy data requires specifying
`option=ormsgpack.OPT_SERIALIZE_NUMPY`.

```python
>>> import ormsgpack, numpy
>>> ormsgpack.packb(
        numpy.array([[1, 2, 3], [4, 5, 6]]),
        option=ormsgpack.OPT_SERIALIZE_NUMPY,
)
>>> ormsgpack.unpackb(_)
[[1,2,3],[4,5,6]]
```

The array must be a contiguous C array (`C_CONTIGUOUS`) and one of the
supported datatypes.

If an array is not a contiguous C array or contains an supported datatype,
ormsgpack falls through to `default`. In `default`, `obj.tolist()` can be
specified. If an array is malformed, which is not expected,
`ormsgpack.MsgpackEncodeError` is raised.


### uuid

ormsgpack serializes `uuid.UUID` instances to
[RFC 4122](https://tools.ietf.org/html/rfc4122) format, e.g.,
"f81d4fae-7dec-11d0-a765-00a0c91e6bf6".

``` python
>>> import ormsgpack, uuid
>>> ormsgpack.packb(uuid.UUID('f81d4fae-7dec-11d0-a765-00a0c91e6bf6'))
>>> ormsgpack.unpackb(_)
"f81d4fae-7dec-11d0-a765-00a0c91e6bf6"
>>> ormsgpack.packb(uuid.uuid5(uuid.NAMESPACE_DNS, "python.org"))
>>> ormsgpack.unpackb(_)
"886313e1-3b8a-5372-9b90-0c9aee199e5d"
```

## Questions

### Why can't I install it from PyPI?

Probably `pip` needs to be upgraded. `pip` added support for `manylinux2014`
in 2019.

### Will it deserialize to dataclasses, UUIDs, decimals, etc or support object_hook?

No. This requires a schema specifying what types are expected and how to
handle errors etc. This is addressed by data validation libraries a
level above this.

### Will it serialize to `str`?

No. `bytes` is the correct type for a serialized blob.

### Will it support PyPy?

If someone implements it well.

## Packaging

To package ormsgpack requires [Rust](https://www.rust-lang.org/) on the
 nightly channel and the [maturin](https://github.com/PyO3/maturin)
build tool. maturin can be installed from PyPI or packaged as
well. This is the simplest and recommended way of installing
from source, assuming `rustup` is available from a
package manager:

```sh
rustup default nightly
pip wheel --no-binary=ormsgpack ormsgpack
```

This is an example of building a wheel using the repository as source,
`rustup` installed from upstream, and a pinned version of Rust:

```sh
pip install maturin
curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain nightly-2021-05-25 --profile minimal -y
maturin build --no-sdist --release --strip --manylinux off
ls -1 target/wheels
```

Problems with the Rust nightly channel may require pinning a version.
`nightly-2021-05-25` is known to be ok.

ormsgpack is tested for amd64 and aarch64 on Linux, macOS, and Windows. It
may not work on 32-bit targets. It has recommended `RUSTFLAGS`
specified in `.cargo/config` so it is recommended to either not set
`RUSTFLAGS` or include these options.

There are no runtime dependencies other than libc.

## License

orjson was written by ijl <<ijl@mailbox.org>>, copyright 2018 - 2021, licensed
under both the Apache 2 and MIT licenses.

ormsgpack was forked from orjson and is maintained by Aviram Hassan <<aviramyhassan@gmail.com>>, licensed
same as orjson.