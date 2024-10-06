# SPDX-License-Identifier: (Apache-2.0 OR MIT)

from typing import TypedDict

import ormsgpack


def test_typeddict() -> None:
    """
    packb() TypedDict
    """

    class TypedDict1(TypedDict):
        a: str
        b: int

    obj = TypedDict1(a="a", b=1)

    assert ormsgpack.unpackb(ormsgpack.packb(obj)) == {"a": "a", "b": 1}
