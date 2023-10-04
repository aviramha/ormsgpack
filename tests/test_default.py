# SPDX-License-Identifier: (Apache-2.0 OR MIT)

import uuid

import msgpack
import pytest

import ormsgpack


class Custom:
    def __init__(self):
        self.name = uuid.uuid4().hex

    def __str__(self):
        return f"{self.__class__.__name__}({self.name})"


class Recursive:
    def __init__(self, cur):
        self.cur = cur


def default(obj):
    if obj.cur != 0:
        obj.cur -= 1
        return obj
    return obj.cur


def default_raises(obj):
    raise TypeError


def test_default_not_callable():
    """
    packb() default not callable
    """
    with pytest.raises(ormsgpack.MsgpackEncodeError):
        ormsgpack.packb(Custom(), default=NotImplementedError)

    ran = False
    try:
        ormsgpack.packb(Custom(), default=NotImplementedError)
    except Exception as err:
        assert isinstance(err, ormsgpack.MsgpackEncodeError)
        assert str(err) == "default serializer exceeds recursion limit"
        ran = True
    assert ran


def test_default_func():
    """
    packb() default function
    """
    ref = Custom()

    def default(obj):
        return str(obj)

    assert ormsgpack.packb(ref, default=default) == msgpack.packb(str(ref))


def test_default_func_none():
    """
    packb() default function None ok
    """
    assert ormsgpack.packb(Custom(), default=lambda x: None) == ormsgpack.packb(
        Custom(), default=lambda x: None
    )


def test_default_func_empty():
    """
    packb() default function no explicit return
    """
    ref = Custom()

    def default(obj):
        if isinstance(obj, set):
            return list(obj)

    assert ormsgpack.packb(ref, default=default) == msgpack.packb(None)
    assert ormsgpack.packb({ref}, default=default) == msgpack.packb([None])


def test_default_func_exc():
    """
    packb() default function raises exception
    """

    def default(obj):
        raise NotImplementedError

    with pytest.raises(ormsgpack.MsgpackEncodeError):
        ormsgpack.packb(Custom(), default=default)

    ran = False
    try:
        ormsgpack.packb(Custom(), default=default)
    except Exception as err:
        assert isinstance(err, ormsgpack.MsgpackEncodeError)
        assert str(err) == "Type is not msgpack serializable: Custom"
        ran = True
    assert ran


def test_default_exception_type():
    """
    packb() TypeError in default() raises ormsgpack.MsgpackEncodeError
    """
    ref = Custom()

    with pytest.raises(ormsgpack.MsgpackEncodeError):
        ormsgpack.packb(ref, default=default_raises)


def test_default_func_nested_str():
    """
    packb() default function nested str
    """
    ref = Custom()

    def default(obj):
        return str(obj)

    assert ormsgpack.packb({"a": ref}, default=default) == msgpack.packb(
        {"a": str(ref)}
    )


def test_default_func_list():
    """
    packb() default function nested list
    """
    ref = Custom()

    def default(obj):
        if isinstance(obj, Custom):
            return [str(obj)]

    assert ormsgpack.packb({"a": ref}, default=default) == msgpack.packb(
        {"a": ref}, default=default
    )


def test_default_func_nested_list():
    """
    packb() default function list
    """
    ref = Custom()

    def default(obj):
        return str(obj)

    assert ormsgpack.packb([ref] * 100, default=default) == msgpack.packb(
        [ref] * 100, default=default
    )


def test_default_func_bytes():
    """
    packb() default function errors on non-str
    """
    ref = Custom()

    def default(obj):
        return bytes(obj)

    with pytest.raises(ormsgpack.MsgpackEncodeError):
        ormsgpack.packb(ref, default=default)

    ran = False
    try:
        ormsgpack.packb(ref, default=default)
    except Exception as err:
        assert isinstance(err, ormsgpack.MsgpackEncodeError)
        assert str(err) == "Type is not msgpack serializable: Custom"
        ran = True
    assert ran


def test_default_func_invalid_str():
    """
    packb() default function errors on invalid str
    """
    ref = Custom()

    def default(obj):
        return "\ud800"

    with pytest.raises(ormsgpack.MsgpackEncodeError):
        ormsgpack.packb(ref, default=default)


def test_default_lambda_ok():
    """
    packb() default lambda
    """
    ref = Custom()
    assert ormsgpack.packb(ref, default=lambda x: str(x)) == msgpack.packb(
        ref, default=lambda x: str(x)
    )


def test_default_callable_ok():
    """
    packb() default callable
    """

    class CustomSerializer:
        def __init__(self):
            self._cache = {}

        def __call__(self, obj):
            if obj not in self._cache:
                self._cache[obj] = str(obj)
            return self._cache[obj]

    ref_obj = Custom()
    ref_bytes = str(ref_obj)
    for obj in [ref_obj] * 100:
        assert ormsgpack.packb(obj, default=CustomSerializer()) == msgpack.packb(
            ref_bytes
        )


def test_default_recursion():
    """
    packb() default recursion limit
    """
    assert ormsgpack.packb(Recursive(254), default=default) == msgpack.packb(0)


def test_default_recursion_reset():
    """
    packb() default recursion limit reset
    """
    assert ormsgpack.packb(
        [Recursive(254), {"a": "b"}, Recursive(254), Recursive(254)],
        default=default,
    ) == msgpack.packb([0, {"a": "b"}, 0, 0])


def test_default_recursion_infinite():
    """
    packb() default infinite recursion
    """
    ref = Custom()

    def default(obj):
        return obj

    with pytest.raises(ormsgpack.MsgpackEncodeError):
        ormsgpack.packb(ref, default=default)
