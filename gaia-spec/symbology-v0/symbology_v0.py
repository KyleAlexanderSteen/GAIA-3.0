"""GAIA symbology v0: signed record -> Reed-Solomon -> glyph grid.
Security comes ONLY from the Ed25519 signature. Glyphs carry meaning, not secrecy."""
import hashlib, struct
from reedsolo import RSCodec, ReedSolomonError
from cryptography.hazmat.primitives.asymmetric.ed25519 import Ed25519PrivateKey, Ed25519PublicKey
from cryptography.hazmat.primitives import serialization
from cryptography.exceptions import InvalidSignature
from PIL import Image, ImageDraw
import numpy as np

VERSION = 1
NSYM = 24
rs = RSCodec(NSYM)
PAYLOAD_FMT = '>BB8sI4s'
PAYLOAD_LEN = struct.calcsize(PAYLOAD_FMT)
FRAME_LEN = 1 + PAYLOAD_LEN + 64
GLYPHS = FRAME_LEN + NSYM
COLS, CELL, PAD = 12, 12, 4
G = 3 * CELL
STEP = G + 2 * PAD
MARGIN = STEP

def pub_bytes(pk):
    return pk.public_bytes(serialization.Encoding.Raw, serialization.PublicFormat.Raw)

def key_id(pk):
    return hashlib.sha256(pub_bytes(pk)).digest()[:4]

def pack_payload(kind, trust, subject, issued, kid):
    return struct.pack(PAYLOAD_FMT, kind, trust, subject, issued, kid)

def unpack_payload(b):
    kind, trust, subject, issued, kid = struct.unpack(PAYLOAD_FMT, b)
    return dict(kind=kind, trust=trust, subject=subject.hex(), issued=issued, key_id=kid.hex())

def encode(payload, sk):
    body = bytes([VERSION]) + payload
    return bytes(rs.encode(body + sk.sign(body)))

def decode(data, keyring):
    try:
        frame = bytes(rs.decode(bytearray(data))[0])
    except ReedSolomonError:
        return 'rejected_rs', None
    if len(frame) != FRAME_LEN or frame[0] != VERSION:
        return 'rejected_format', None
    body, sig = frame[:-64], frame[-64:]
    kid = body[1 + PAYLOAD_LEN - 4:]
    pk = keyring.get(kid)
    if pk is None:
        return 'rejected_unknown_key', None
    try:
        pk.verify(sig, body)
    except InvalidSignature:
        return 'rejected_signature', None
    return 'verified', frame

def rows_for(n): return -(-n // COLS)

def render(data):
    rows = rows_for(len(data))
    W, H = COLS * STEP + 2 * MARGIN, rows * STEP + 2 * MARGIN
    img = Image.new('L', (W, H), 255)
    d = ImageDraw.Draw(img)
    for i, byte in enumerate(data):
        r, c = divmod(i, COLS)
        x0, y0 = MARGIN + c * STEP + PAD, MARGIN + r * STEP + PAD
        cells = [1] + [(byte >> b) & 1 for b in range(8)]
        for k, on in enumerate(cells):
            cy, cx = divmod(k, 3)
            if on:
                d.rectangle([x0 + cx * CELL, y0 + cy * CELL, x0 + cx * CELL + CELL - 1, y0 + cy * CELL + CELL - 1], fill=0)
    return img

def read_grid(img, n):
    a = np.asarray(img.convert('L'), dtype=float)
    out = bytearray()
    for i in range(n):
        r, c = divmod(i, COLS)
        x0, y0 = MARGIN + c * STEP + PAD, MARGIN + r * STEP + PAD
        byte = 0
        for k in range(1, 9):
            cy, cx = divmod(k, 3)
            px, py = x0 + cx * CELL + CELL // 2, y0 + cy * CELL + CELL // 2
            if a[py - 2:py + 3, px - 2:px + 3].mean() < 128:
                byte |= 1 << (k - 1)
        out.append(byte)
    return bytes(out)

def decode_image(img, keyring):
    W, H = COLS * STEP + 2 * MARGIN, rows_for(GLYPHS) * STEP + 2 * MARGIN
    last = ('rejected_geometry', None)
    for k in range(4):
        im = img.rotate(90 * k, expand=True)
        if im.size != (W, H):
            continue
        res = decode(read_grid(im, GLYPHS), keyring)
        if res[0] == 'verified':
            return res
        last = res
    return last
