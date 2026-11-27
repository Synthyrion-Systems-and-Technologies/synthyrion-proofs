"""Re-derive the headline counts of the results from the published tables. Standard library only.
Run from the repository root:  python -I tools/recount.py
Exit 0 only if every count equals the number the README states."""
import csv, collections, sys, os
R = os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', 'results')
def table(name):
    with open(os.path.join(R, name), encoding='utf8', newline='') as f:
        return list(csv.DictReader(f, delimiter='\t'))
ok = True
def check(label, got, want):
    global ok
    good = got == want
    ok &= good
    print('%-58s %s  (%s)' % (label, 'OK ' if good else 'FAIL', got))
c = collections.Counter(r['state'] for r in table('census.tsv'))
check('census functions', sum(c.values()), 5659)
check('census PROVEN', c['PROVEN'], 5101)
check('census PROVEN-INDUCTION', c['PROVEN-INDUCTION'], 426)
check('census BLOCKED', c['BLOCKED'], 105)
check('census NOT-DEP-FREE', c['NOT-DEP-FREE'], 27)
e = table('wcet_entries.tsv')
s = collections.Counter(r['status'] for r in e)
check('timing entries registered', len(e), 140)
check('timing entries BOUNDED', s['BOUNDED'], 126)
check('timing entries PENDING', s['PENDING'], 9)
check('timing entries REFERENCE', s['REFERENCE'], 5)
k = collections.Counter((r['kind'], r['status']) for r in e)
check('BOUNDED kernel-path entries', k[('kernel-path', 'BOUNDED')], 84)
check('BOUNDED table-method entries', k[('table-method', 'BOUNDED')], 42)
check('every BOUNDED entry has a numeric bound', all(r['bound_mir_statements'].isdigit() for r in e if r['status'] == 'BOUNDED'), True)
check('every PENDING entry names an obstacle', all(r['first_obstacle_if_pending'] for r in e if r['status'] == 'PENDING'), True)
b = collections.Counter(r['binding_class'] for r in table('wcet_loop_rows.tsv'))
check('loop rows', sum(b.values()), 240)
check('loop rows SOURCE/MIR/BEHAVIOURAL/ASSUMED', (b['SOURCE'], b['MIR'], b['BEHAVIOURAL'], b['ASSUMED']), (148, 54, 8, 30))
cc = table('wcet_callee_costs.tsv')
check('callee-cost rows', len(cc), 594)
check('callee rows labelled twin-verified', sum(1 for r in cc if r['labelled_twin_verified'] == 'yes'), 466)
check('callee rows derived (reference to another function)', sum(1 for r in cc if r['cost_mir_statements_or_derived_reference'].startswith('@')), 75)
check('capacity constants', len(table('wcet_constants.tsv')), 75)
print('ALL COUNTS AGREE' if ok else 'MISMATCH')
sys.exit(0 if ok else 1)
