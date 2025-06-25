#!/usr/bin/env -S pipx run
# /// script
# requires-python = ">=3.10"
# dependencies = ["rich"]
# ///

from __future__ import annotations
from operator import itemgetter
import sys
from rich.color import ANSI_COLOR_NAMES


def main() -> None:
    print("use phf::{phf_map, Map};")
    print("use unicase::UniCase;")
    print()
    print("#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]")
    print("pub struct AnsiColor(pub u8);")
    print()

    nums_to_names: dict[int, str] = {}
    for name, index in ANSI_COLOR_NAMES.items():
        if index in nums_to_names:
            if "grey" in nums_to_names[index] and "gray" in name:
                nums_to_names[index] = name
            elif "gray" in nums_to_names[index] and "grey" in name:
                pass
            else:
                sys.exit(
                    f"Unexpected: color {index} has names {name!r} and {nums_to_names[index]!r}"
                )
        else:
            nums_to_names[index] = name
    print("impl AnsiColor {")
    print("    pub fn name(self) -> Option<&'static str> {")
    print("        match self.0 {")
    for index in range(256):
        try:
            name = nums_to_names[index]
        except KeyError:
            name = "None"
        else:
            name = f'Some("{name}")'
        print(f"            {index} => {name},")
    print("        }")
    print("    }")
    print("}")
    print()

    print("impl From<u8> for AnsiColor {")
    print("    fn from(value: u8) -> AnsiColor {")
    print("        AnsiColor(value)")
    print("    }")
    print("}")
    print()
    print("impl From<AnsiColor> for u8 {")
    print("    fn from(value: AnsiColor) -> u8 {")
    print("        value.0")
    print("    }")
    print("}")
    print()

    print("static BY_NAME: Map<UniCase<&'static str>, AnsiColor> = phf_map! {")
    for name, index in ANSI_COLOR_NAMES.items():
        print(f'    UniCase::ascii("{name}") => AnsiColor({index}),')
    print("};")


if __name__ == "__main__":
    main()
