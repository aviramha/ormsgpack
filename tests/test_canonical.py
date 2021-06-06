# SPDX-License-Identifier: (Apache-2.0 OR MIT)

import ormsgpack
import msgpack

# AH: I'm not sure if this is needed, changed this into same-behavior comparison.

def test_packb_ctrl_escape():
    """
    packb() ctrl characters
    """
    assert ormsgpack.packb("text\u0003\r\n") == msgpack.packb("text\u0003\r\n")

def test_packb_escape_quote_backslash():
    """
    packb() quote, backslash escape
    """
    assert ormsgpack.packb(r'"\ test') == msgpack.packb(r'"\ test')

def test_packb_escape_line_separator():
    """
    packb() U+2028, U+2029 escape
    """
    assert ormsgpack.packb({"spaces": "\u2028 \u2029"}) == msgpack.packb({"spaces": "\u2028 \u2029"})
