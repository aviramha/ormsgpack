from typing import Any, Callable, Optional, Union

__version__: str

def packb(
    obj: Any,
    default: Optional[Callable[[Any], Any]] = ...,
    option: Optional[int] = ...,
) -> bytes: ...
def unpackb(__obj: Union[bytes, bytearray, memoryview]) -> Any: ...

class MsgpackDecodeError(ValueError): ...
class MsgpackEncodeError(TypeError): ...

OPT_NAIVE_UTC: int
OPT_OMIT_MICROSECONDS: int
OPT_PASSTHROUGH_DATACLASS: int
OPT_PASSTHROUGH_DATETIME: int
OPT_PASSTHROUGH_SUBCLASS: int
OPT_SERIALIZE_NUMPY: int
OPT_SERIALIZE_UUID: int
OPT_SERIALIZE_PYDANTIC: int
OPT_UTC_Z: int
