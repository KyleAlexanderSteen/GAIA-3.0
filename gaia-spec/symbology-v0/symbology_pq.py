"""Symbology v0 hybrid frame: Ed25519 + ML-DSA-44 (FIPS 204). BOTH signatures must verify.
Uses dilithium-py, a pure-Python implementation: prototype only; production needs a vetted, constant-time library."""
import hashlib, struct
from dilithium_py.ml_dsa import ML_DSA_44
from symbology_v0 import (rs, ReedSolomonError, Ed25519PrivateKey, InvalidSignature, pub_bytes, PAYLOAD_LEN, pack_payload)

VERSION_PQ = 2
ED_SIG, PQ_SIG = 64, 2420
FRAME_LEN_PQ = 1 + PAYLOAD_LEN + ED_SIG + PQ_SIG

def hybrid_key_id(ed_pk, pq_pk):
    return hashlib.sha256(pub_bytes(ed_pk) + pq_pk).digest()[:4]

def encode_pq(payload, ed_sk, pq_sk):
    body = bytes([VERSION_PQ]) + payload
    return bytes(rs.encode(body + ed_sk.sign(body) + ML_DSA_44.sign(pq_sk, body)))

def decode_pq(data, keyring):
    try:
        frame = bytes(rs.decode(bytearray(data))[0])
    except ReedSolomonError:
        return 'rejected_rs', None
    if len(frame) != FRAME_LEN_PQ or frame[0] != VERSION_PQ:
        return 'rejected_format', None
    body = frame[:1 + PAYLOAD_LEN]
    ed_sig, pq_sig = frame[len(body):len(body) + ED_SIG], frame[len(body) + ED_SIG:]
    keys = keyring.get(body[-4:])
    if keys is None:
        return 'rejected_unknown_key', None
    ed_pk, pq_pk = keys
    try:
        ed_pk.verify(ed_sig, body)
    except InvalidSignature:
        return 'rejected_ed25519', None
    if not ML_DSA_44.verify(pq_pk, body, pq_sig):
        return 'rejected_mldsa', None
    return 'verified', frame
