"""Check results/MANIFEST.sha512 (SHA-512 of every published result file). Standard library only.
Run from the repository root:  python -I tools/verify_manifest.py"""
import hashlib, os, sys
R = os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', 'results')
bad = 0; n = 0
for line in open(os.path.join(R, 'MANIFEST.sha512'), encoding='utf8'):
    line = line.rstrip('\n')
    if not line: continue
    digest, name = line.split('  ', 1)
    got = hashlib.sha512(open(os.path.join(R, name), 'rb').read()).hexdigest()
    n += 1
    if got != digest:
        bad += 1; print('MISMATCH', name)
print('%d files checked, %d mismatches' % (n, bad))
sys.exit(1 if bad else 0)
