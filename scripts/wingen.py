#!/usr/bin/env python3
import sys
import os
import subprocess
from pathlib import Path


def add_line_to_file(file_path: str, line_number: int, line_to_add: str) -> bool:
    path = Path(file_path)
    if not path.exists():
        print(f"Error: File not found: {file_path}", file=sys.stderr)
        return False

    with open(path, 'r', encoding='utf-8') as f:
        lines = f.read().splitlines()

    if line_number <= 0 or line_number > len(lines):
        print(f"Error: Line number {line_number} is out of range (1..{len(lines)})", file=sys.stderr)
        return False

    lines[line_number - 1] += line_to_add

    with open(path, 'w', encoding='utf-8', newline='') as f:
        f.write('\n'.join(lines))
        f.write('\n')

    return True


MACRO_STRING = r'''
macro_rules! link {
    ($library:literal $abi:literal $($link_name:literal)? $(#[$doc:meta])? fn $($function:tt)*) => (
        #[link(name = $library)]
        unsafe extern $abi {
            $(#[link_name=$link_name])?
            pub fn $($function)*;
        }
    )
}
'''


def main():
    args = sys.argv[1:]

    with open("winlinks.txt", "a") as f:
        for arg in args:
            f.write(f"{arg}\n")

    original_dir = os.getcwd()
    os.chdir("wingen")
    try:
        subprocess.run(["cargo", "run"], check=True)
    finally:
        os.chdir(original_dir)

    add_line_to_file("src/os/windows.rs", 10, "#![allow(clippy::unreadable_literal)]")
    add_line_to_file("src/os/windows.rs", 10, MACRO_STRING)

    file_path = Path("src/os/windows.rs")
    with open(file_path, 'r', encoding='utf-8') as f:
        content = f.read()

    content = content.replace("windows_link::", "")
    content = content.replace(".dll", "")
    content = content.replace("unsafe { core::mem::zeroed() }", "// SAFETY: All types are guaranteed to be zeroable\n\t\tunsafe { core::mem::zeroed() }")
    content = content.replace("#[derive(Clone, Copy)]\npub struct GUID {", "#[derive(Clone, Copy, Default)]\npub struct GUID {")

    with open(file_path, 'w', encoding='utf-8', newline='') as f:
        f.write(content)


if __name__ == "__main__":
    main()