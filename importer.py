#!/usr/bin/env python3

from pathlib import Path
import re
import sys


def pascal_case(name: str) -> str:
    parts = re.split(r"[^a-zA-Z0-9]+", name)
    return "".join(p.capitalize() for p in parts if p)


def snake_case(name: str) -> str:
    name = name.strip().lower()
    name = re.sub(r"[^a-z0-9]+", "_", name)
    return name.strip("_")


def rgba_to_hex(r: int, g: int, b: int, a: int) -> str:
    return f"#{r:02x}{g:02x}{b:02x}{a:02x}"


def parse_gpl(path: Path):
    colors = []

    with path.open("r", encoding="utf-8", errors="ignore") as f:
        for line in f:
            line = line.strip()

            if not line:
                continue
            if line.startswith("#"):
                continue
            if ":" in line:
                # Name:, Author:, etc.
                continue
            if line.lower() == "gimp palette":
                continue

            # Split on whitespace; name may contain spaces
            parts = line.split()
            if len(parts) < 4:
                continue

            try:
                r = int(parts[0])
                g = int(parts[1])
                b = int(parts[2])
            except ValueError:
                continue

            a = 255
            name_part = 4
            try:
                a = int(parts[3])
            except ValueError:
                name_part = 3
            if a < 0 or a > 255:
                a = 255
                name_part = 3

            name = " ".join(parts[name_part:])
            name = snake_case(name)
            hex = rgba_to_hex(r, g, b, a)
            if name == "untitled":
                name = hex[1:]
            if name == hex[1:] or name == hex[1:7]:
                name = "color_" + name
            color = (name, hex)
            if color not in colors:
                colors.append(color)

    return colors


def main(root: Path, output: Path):
    palettes = []

    for gpl in sorted(root.rglob("*.gpl")):
        palette_name = pascal_case(gpl.stem)
        colors = parse_gpl(gpl)

        if not colors:
            continue

        palettes.append((palette_name, colors))

    with output.open("w", encoding="utf-8") as out:
        out.write("use macros::palette;\n")

        for palette_name, colors in palettes:
            out.write(f"\npalette!({palette_name} {{\n")
            for name, hexcode in colors:
                out.write(f'\t"{name}": "{hexcode}",\n')
            out.write("});\n")


if __name__ == "__main__":
    if len(sys.argv) != 3:
        print("Usage: gpl_to_palettes.py <input_dir> <output.rs>")
        sys.exit(1)

    main(Path(sys.argv[1]), Path(sys.argv[2]))
