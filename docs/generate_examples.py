#!/usr/bin/env python3
# SPDX-License-Identifier: (Apache-2.0 OR MIT)

import code
import contextlib
import io
import sys
from pathlib import Path
from typing import TextIO


class Console(code.InteractiveConsole):
    def __init__(self, fp: TextIO) -> None:
        super().__init__()
        self.fp = fp

    def raw_input(self, prompt: str = "") -> str:
        try:
            line = next(self.fp).rstrip("\n")
        except StopIteration:
            raise EOFError()

        data = f"{prompt}{line}".rstrip(" ")
        self.write(f"{data}\n")
        return line

    def write(self, data: str) -> None:
        sys.stdout.write(data)


def main() -> None:
    sys.tracebacklimit = 0
    for path in Path("docs/examples").glob("*.py"):
        with (
            io.StringIO() as data,
            contextlib.redirect_stdout(data),
            path.open() as fp,
        ):
            Console(fp).interact(banner="", exitmsg="")
            lines = data.getvalue().split("\n")
            if not lines[-2] and not lines[-1]:
                lines.pop()
            path.with_suffix(".txt").write_text("\n".join(lines))


if __name__ == "__main__":
    main()
