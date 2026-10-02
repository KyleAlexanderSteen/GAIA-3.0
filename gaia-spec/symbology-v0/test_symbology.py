import json, random, csv, math, collections, urllib.request, time, re
import numpy as np
from symbology_v0 import *

rng = random.Random(1337)
R = {}
sk = Ed25519PrivateKey.generate(); pk = sk.public_key()
keyring = {key_id(pk): pk}
payload = pack_payload(1, 3, bytes.fromhex('a1b2c3d4e5f60718'), 1790000000, key_id(pk))
data = encode(payload, sk)
R['glyph_count'] = len(data); R['frame_bytes'] = FRAME_LEN; R['parity_glyphs'] = NSYM

def dmg(d, k, burst=False):
    d = bytearray(d)
    pos = list(range(rng.randrange(0, len(d) - k + 1), 0)) if False else None
    idx = list(range((s := rng.randrange(0, len(d) - k + 1)), s + k)) if burst else rng.sample(range(len(d)), k)
    for i in idx:
        d[i] = (d[i] + rng.randrange(1, 256)) % 256
    return bytes(d)

t0 = time.time()
r0 = decode(data, keyring)
R['roundtrip_bytes'] = r0[0]
img = render(data); img.save('glyph_sample.png')
R['roundtrip_image'] = decode_image(img, keyring)[0]
R['rotations'] = {str(90 * k): decode_image(img.rotate(90 * k, expand=True), keyring)[0] for k in range(4)}

curve = []
for mode in ('scattered', 'burst'):
    for k in range(0, 31):
        c = collections.Counter()
        for _ in range(300):
            st, fr = decode(dmg(data, k, mode == 'burst'), keyring)
            if st == 'verified' and fr[:-64][1:] != payload:
                st = 'ACCEPTED_WRONG'
            c[st] += 1
        curve.append(dict(mode=mode, damaged_glyphs=k, trials=300, verified=c['verified'],
                          rejected=sum(v for s, v in c.items() if s.startswith('rejected')), accepted_wrong=c['ACCEPTED_WRONG']))
with open('damage_curve.csv', 'w', newline='') as f:
    w = csv.DictWriter(f, fieldnames=list(curve[0])); w.writeheader(); w.writerows(curve)
R['max_damage_fully_recovered'] = {m: max((x['damaged_glyphs'] for x in curve if x['mode'] == m and x['verified'] == x['trials']), default=-1) for m in ('scattered', 'burst')}
R['accepted_wrong_total'] = sum(x['accepted_wrong'] for x in curve)
R['damage_trials_total'] = sum(x['trials'] for x in curve)

nimg = collections.Counter()
for t in range(60):
    im = render(data).copy(); a = np.asarray(im).copy()
    for _ in range(rng.randrange(0, 13)):
        r, c = divmod(rng.randrange(GLYPHS), COLS)
        y0, x0 = MARGIN + r * STEP, MARGIN + c * STEP
        a[y0:y0 + STEP, x0:x0 + STEP] = np.random.RandomState(t).choice([0, 255], size=(STEP, STEP))
    noise = np.random.RandomState(t + 99).rand(*a.shape) < 0.01
    a[noise] = 255 - a[noise]
    nimg[decode_image(Image.fromarray(a), keyring)[0]] += 1
R['image_noise_and_blotch_<=12glyphs_plus_1pct_pixel_noise'] = dict(nimg)
dm = np.asarray(render(data)).copy()
for i in (3, 20, 41, 77, 90):
    r, c = divmod(i, COLS); dm[MARGIN + r * STEP:MARGIN + (r + 1) * STEP, MARGIN + c * STEP:MARGIN + (c + 1) * STEP] = 128
Image.fromarray(dm).save('glyph_damaged.png')
R['damaged_png_decodes'] = decode_image(Image.fromarray(dm), keyring)[0]

f = collections.Counter()
other = Ed25519PrivateKey.generate()
for _ in range(2000):
    body = bytearray(bytes([VERSION]) + payload)
    sig = sk.sign(bytes(body))
    j = rng.randrange(1, len(body) - 4)
    body[j] ^= 1 << rng.randrange(8)
    f['tamper_field_reencoded_with_valid_parity_' + decode(bytes(rs.encode(bytes(body) + sig)), keyring)[0]] += 1
    b2 = bytes([VERSION]) + payload
    f['wrong_key_claiming_victim_id_' + decode(bytes(rs.encode(b2 + other.sign(b2))), keyring)[0]] += 1
    p2 = pack_payload(1, 3, bytes(rng.randrange(256) for _ in range(8)), rng.randrange(2**32), key_id(other.public_key()))
    b3 = bytes([VERSION]) + p2
    f['attacker_own_key_unknown_to_verifier_' + decode(bytes(rs.encode(b3 + other.sign(b3))), keyring)[0]] += 1
    p4 = pack_payload(1, 3, bytes(rng.randrange(256) for _ in range(8)), 0, key_id(pk))
    b4 = bytes([VERSION]) + p4
    f['random_signature_forgery_' + decode(bytes(rs.encode(b4 + bytes(rng.randrange(256) for _ in range(64)))), keyring)[0]] += 1
R['forgery_2000_each'] = dict(f)
R['replay_of_old_valid_glyph'] = decode(data, keyring)[0] + ' (replay is NOT prevented by v0; needs nonce/expiry policy)'

m = b'GAIA attack at dawn'
k = bytes(rng.randrange(256) for _ in range(len(m)))
c = bytes(a ^ b for a, b in zip(m, k))
alt = b'GAIA stand down now'
k2 = bytes(a ^ b for a, b in zip(c, alt))
R['otp_any_same_length_plaintext_consistent'] = bytes(a ^ b for a, b in zip(c, k2)) == alt and len(alt) == len(m)
R['otp_key_bytes_per_message_byte'] = 1

def getcorpus():
    for u in ('https://www.gutenberg.org/cache/epub/1342/pg1342.txt', 'https://www.gutenberg.org/files/1342/1342-0.txt'):
        try:
            return urllib.request.urlopen(u, timeout=20).read().decode('utf8', 'ignore')
        except Exception:
            pass
try:
    raw = getcorpus(); txt = re.sub('[^a-z]', '', raw.lower())
    split = int(len(txt) * 0.8); train, held = txt[:split], txt[split:]
    tri = collections.Counter(train[i:i + 3] for i in range(len(train) - 2)); tot = sum(tri.values())
    lp = {g: math.log(v / tot) for g, v in tri.items()}; floor = math.log(0.1 / tot)
    uni = collections.Counter(train); eng_rank = [ch for ch, _ in uni.most_common()]
    AL = 'abcdefghijklmnopqrstuvwxyz'
    def sc(s): return sum(lp.get(s[i:i + 3], floor) for i in range(len(s) - 2))
    def crack(ct, rs_, iters=3000, restarts=6):
        cr = [ch for ch, _ in collections.Counter(ct).most_common()] + [ch for ch in AL if ch not in ct]
        best = None
        for r in range(restarts):
            m = dict(zip(cr, eng_rank + [x for x in AL if x not in eng_rank]))
            if r:
                ks = list(m)
                for _ in range(10 * r): a, b = rs_.sample(ks, 2); m[a], m[b] = m[b], m[a]
            cur = sc(''.join(m[ch] for ch in ct))
            for _ in range(iters):
                a, b = rs_.sample(list(m), 2); m[a], m[b] = m[b], m[a]
                s2 = sc(''.join(m[ch] for ch in ct))
                if s2 > cur: cur = s2
                else: m[a], m[b] = m[b], m[a]
            if best is None or cur > best[0]: best = (cur, dict(m))
        return best[1]
    accs = []
    for n_letters in (300, 600, 1200):
        pl = held[5000:5000 + n_letters]
        perm = list(AL); rng.shuffle(perm); enc = dict(zip(AL, perm))
        ct = ''.join(enc[ch] for ch in pl)
        mp = crack(ct, rng)
        rec = ''.join(mp[ch] for ch in ct)
        accs.append(dict(ciphertext_letters=n_letters, letters_recovered_pct=round(100 * sum(x == y for x, y in zip(rec, pl)) / len(pl), 1)))
    R['substitution_cipher_(wakandan_style)_frequency_attack'] = accs
except Exception as e:
    R['substitution_cipher_(wakandan_style)_frequency_attack'] = 'skipped: ' + repr(e)
R['runtime_seconds'] = round(time.time() - t0, 1)
json.dump(R, open('results.json', 'w'), indent=1)
print(json.dumps(R, indent=1))
