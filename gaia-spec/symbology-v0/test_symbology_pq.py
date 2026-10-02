import json, random, collections, time, math
from symbology_v0 import *
from symbology_pq import *

rng = random.Random(2026)
R = {}
ed_sk = Ed25519PrivateKey.generate(); ed_pk = ed_sk.public_key()
t = time.time(); pq_pk, pq_sk = ML_DSA_44.keygen(); R['mldsa_keygen_s'] = round(time.time() - t, 3)
kid = hybrid_key_id(ed_pk, pq_pk); keyring = {kid: (ed_pk, pq_pk)}
payload = pack_payload(1, 3, bytes.fromhex('a1b2c3d4e5f60718'), 1790000000, kid)
t = time.time(); data = encode_pq(payload, ed_sk, pq_sk); R['encode_s'] = round(time.time() - t, 3)
t = time.time(); R['roundtrip'] = decode_pq(data, keyring)[0]; R['decode_verify_s'] = round(time.time() - t, 3)

n = len(data)
R['sizes'] = dict(ed25519_only_glyphs=107, hybrid_glyphs=n, growth_factor=round(n / 107, 1), mldsa44_signature_bytes=PQ_SIG, mldsa44_public_key_bytes=len(pq_pk))
R['sheet_pixels'] = {f'{c}_columns': [c * STEP + 2 * MARGIN, math.ceil(n / c) * STEP + 2 * MARGIN] for c in (12, 24, 48)}

def dmg(d, k, burst):
    d = bytearray(d)
    s = rng.randrange(0, len(d) - k + 1) if burst else 0
    idx = range(s, s + k) if burst else rng.sample(range(len(d)), k)
    for i in idx: d[i] = (d[i] + rng.randrange(1, 256)) % 256
    return bytes(d)

rows = []
for mode in ('scattered', 'burst'):
    for k in (0, 12, 24, 48, 96, 144, 192):
        c = collections.Counter()
        for _ in range(12):
            st, fr = decode_pq(dmg(data, k, mode == 'burst'), keyring)
            if st == 'verified' and fr[1:1 + PAYLOAD_LEN] != payload: st = 'ACCEPTED_WRONG'
            c['verified' if st == 'verified' else 'ACCEPTED_WRONG' if st == 'ACCEPTED_WRONG' else 'rejected'] += 1
        rows.append(dict(mode=mode, damaged_glyphs=k, trials=12, **c))
R['damage'] = rows
R['accepted_wrong_total'] = sum(r.get('ACCEPTED_WRONG', 0) for r in rows)

body = bytes([VERSION_PQ]) + payload
good_ed, good_pq = ed_sk.sign(body), ML_DSA_44.sign(pq_sk, body)
evil = Ed25519PrivateKey.generate(); epq_pk, epq_sk = ML_DSA_44.keygen()
tamper = bytearray(body); tamper[3] ^= 1; tamper = bytes(tamper)
cases = {
 'tampered_field_old_signatures': tamper + good_ed + good_pq,
 'ed25519_BROKEN_attacker_resigns_ed_keeps_old_mldsa': tamper + ed_sk.sign(tamper) + good_pq,
 'mldsa_replaced_with_random': body + good_ed + bytes(rng.randrange(256) for _ in range(PQ_SIG)),
 'attacker_own_both_keys_claiming_victim_id': body + evil.sign(body) + ML_DSA_44.sign(epq_sk, body),
 'ed25519_only_frame_v1_presented_to_v2_verifier': bytes([1]) + payload + good_ed + bytes(PQ_SIG),
}
R['forgery_cases'] = {k: decode_pq(bytes(rs.encode(v)), keyring)[0] for k, v in cases.items()}
R['note'] = 'dilithium-py is pure Python and not constant time: prototype only. Replay still not prevented.'
json.dump(R, open('results_pq.json', 'w'), indent=1)
print(json.dumps(R, indent=1))
