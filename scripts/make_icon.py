#!/usr/bin/env python3
"""Genera el icono de Cristal: una gota de vidrio sobre azul profundo."""

import struct
import zlib
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1] / "src-tauri" / "icons"
SIZE = 1024


def pixel(x, y):
    nx = (x + 0.5) / SIZE
    ny = (y + 0.5) / SIZE
    cx, cy = 0.50, 0.46
    dx, dy = nx - cx, (ny - cy) * 0.92
    dist = (dx * dx + dy * dy) ** 0.5
    radius = 0.34
    # Superellipse-ish rounded square so it reads as a macOS icon.
    ax = abs(nx - 0.5) / 0.42
    ay = abs(ny - 0.5) / 0.42
    box = (ax ** 4 + ay ** 4) ** 0.25
    if box > 1:
        return (0, 0, 0, 0)

    # Deep translucent blue, lighter toward the top.
    light = max(0.0, 1 - ny)
    r = int(36 + 70 * light)
    g = int(110 + 90 * light)
    b = int(196 + 40 * light)
    a = 255

    # Droplet cutout in the center, slightly glassy.
    drop_y = ny - 0.08
    in_drop = (nx - 0.5) ** 2 / 0.045 + (drop_y - 0.48) ** 2 / 0.07 < 1
    tip = (nx - 0.5) ** 2 * 18 + (0.34 - ny) * 1.15
    if ny < 0.48 and tip < 0.08:
        in_drop = True
    if in_drop:
        r = min(255, r + 40)
        g = min(255, g + 28)
        b = 255
        # Inner highlight.
        if nx < 0.47 and ny > 0.42:
            r, g, b = 230, 244, 255

    # Specular streak.
    streak = abs((nx * 0.7 + ny) - 0.72)
    if streak < 0.035 and box < 0.92:
        blend = 1 - streak / 0.035
        r = int(r + (255 - r) * blend * 0.55)
        g = int(g + (255 - g) * blend * 0.55)
        b = int(b + (255 - b) * blend * 0.45)

    # Soft edge.
    edge = max(0.0, min(1.0, (1 - box) / 0.025))
    a = int(255 * edge)
    return (r, g, b, a)


def write_png(path, size, source=SIZE):
    raw = bytearray()
    step = source / size
    for y in range(size):
        raw.append(0)
        sy = min(source - 1, int((y + 0.5) * step))
        for x in range(size):
            sx = min(source - 1, int((x + 0.5) * step))
            raw.extend(pixel(sx, sy) if source == SIZE else sample(sx, sy))
    chunk = lambda tag, data: struct.pack(">I", len(data)) + tag + data + struct.pack(">I", zlib.crc32(tag + data) & 0xFFFFFFFF)
    ihdr = struct.pack(">IIBBBBB", size, size, 8, 6, 0, 0, 0)
    png = b"\x89PNG\r\n\x1a\n" + chunk(b"IHDR", ihdr) + chunk(b"IDAT", zlib.compress(bytes(raw), 9)) + chunk(b"IEND", b"")
    path.write_bytes(png)


def sample(x, y):
    return pixel(x, y)


def write_ico(path, png_path):
    png = png_path.read_bytes()
    header = struct.pack("<HHH", 0, 1, 1)
    entry = struct.pack("<BBBBHHII", 0, 0, 0, 0, 1, 32, len(png), 22)
    path.write_bytes(header + entry + png)


def main():
    ROOT.mkdir(parents=True, exist_ok=True)
    master = ROOT / "icon.png"
    write_png(master, 1024)
    write_png(ROOT / "128x128.png", 128)
    write_png(ROOT / "128x128@2x.png", 256)
    write_png(ROOT / "32x32.png", 32)
    write_ico(ROOT / "icon.ico", ROOT / "32x32.png")
    print(master)


if __name__ == "__main__":
    main()
