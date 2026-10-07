"""Renders the Local Grind app icon (1024x1024 PNG) without third-party libraries.

Design: a dark rounded square with an amber chevron prompt and a green underscore cursor,
evoking a terminal ready for the next problem.
"""
import math
import struct
import sys
import zlib

N = 1024
BG = (22, 24, 29)
AMBER = (255, 161, 22)
GREEN = (44, 187, 93)


def rounded_square_sdf(x, y, half, radius):
    qx, qy = abs(x) - (half - radius), abs(y) - (half - radius)
    outside = math.hypot(max(qx, 0), max(qy, 0))
    return outside + min(max(qx, qy), 0) - radius


def segment_sdf(px, py, ax, ay, bx, by):
    pax, pay, bax, bay = px - ax, py - ay, bx - ax, by - ay
    h = max(0.0, min(1.0, (pax * bax + pay * bay) / (bax * bax + bay * bay)))
    return math.hypot(pax - bax * h, pay - bay * h)


def cover(d):
    """Anti-aliased coverage from a signed distance (negative inside)."""
    return max(0.0, min(1.0, 0.5 - d))


def blend(dst, src, a):
    return tuple(d + (s - d) * a for d, s in zip(dst, src))


rows = []
for j in range(N):
    row = bytearray([0])
    for i in range(N):
        x, y = i + 0.5 - N / 2, j + 0.5 - N / 2
        a_bg = cover(rounded_square_sdf(x, y, 440, 190))
        color = BG
        stroke = 58
        d_chev = min(
            segment_sdf(x, y, -250, -170, -60, 0),
            segment_sdf(x, y, -60, 0, -250, 170),
        ) - stroke
        color = blend(color, AMBER, cover(d_chev))
        d_cursor = segment_sdf(x, y, 40, 170, 250, 170) - stroke
        color = blend(color, GREEN, cover(d_cursor))
        row += bytes(int(round(c)) for c in color) + bytes([int(round(255 * a_bg))])
    rows.append(bytes(row))


def chunk(tag, data):
    return struct.pack(">I", len(data)) + tag + data + struct.pack(">I", zlib.crc32(tag + data) & 0xFFFFFFFF)


png = b"\x89PNG\r\n\x1a\n" + chunk(b"IHDR", struct.pack(">IIBBBBB", N, N, 8, 6, 0, 0, 0))
png += chunk(b"IDAT", zlib.compress(b"".join(rows), 9)) + chunk(b"IEND", b"")
with open(sys.argv[1] if len(sys.argv) > 1 else "icon.png", "wb") as f:
    f.write(png)
