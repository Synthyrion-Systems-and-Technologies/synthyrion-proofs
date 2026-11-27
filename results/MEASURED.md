# What was measured, when, and how

Measured on the private repository at commit `0368426a0fcb8282c74848615e991b157cfe12f7` (committed 2026-11-26 21:26 -0800), in a clean checkout, on 2026-11-26 and 2026-11-27. The tables were read from the same commit's committed files; the checker outputs are the tools' own output at that commit. A diff confirmed that no kernel source file, timing table or census file differed at the later commit `b24945620` on which the methods report's other counts were taken.

| File | Derived how | Counts |
|---|---|---|
| `census.tsv` | every row of the committed panic-freedom census file; where a function has several rows (rows are appended as code changes), the last row wins | 5,659 functions: 5,101 proven, 426 proven by induction, 105 blocked, 27 outside the checker's reach; equals the census file's own totals line (`checker_outputs/census_totals_line.txt`) |
| `wcet_entries.tsv` | the timing registry (140 rows) joined with the bounds in the analyser's committed table; kind: `kernel-path` (a gate call or a scheduling or dispatch path), `table-method` (a method of a fixed-capacity table type), `reference` (reference implementation outside the kernel closure) | 126 BOUNDED (84 kernel-path, 42 table-method), 9 PENDING, 5 REFERENCE |
| `wcet_functions.tsv` | every entry row of the analyser's committed table, per crate | 140 rows, MIR statements only (the two cycle bounds are in `wcet_x86_cycles.tsv`, a different unit) |
| `wcet_loop_rows.tsv` | the 240 loop-bound rows with the class of evidence that checks each: SOURCE (a test reads the loop and the row), MIR (the analyser's binder proves it), BEHAVIOURAL (a test runs the code), ASSUMED (nothing checks it) | 148 / 54 / 8 / 30 |
| `wcet_callee_costs.tsv` | the 594 callee-cost rows; `labelled_twin_verified` is yes where the row's evidence note says it was verified against an equivalent body; a cost starting with `@` is derived by analysing the callee | 466 twin-labelled, 75 derived |
| `wcet_constants.tsv` | the 75 capacity constants the kernel enforces | 75 |
| `wcet_x86_cycles.tsv` | the two entries bounded in cycles on one compiled object (object SHA-512 given); Intel cores only, under the assumptions printed in the analyser's table | 2 |
| `checker_outputs/` | the timing analyser's check (0 problems), its missing-rows report (186 distinct callee-cost rows and 128 loop rows missing over the nine pending entries), the proof-source counts, the census totals line | |
| `bounded_checks.tsv` | written from the published subject's tests (re-run here) and from the project's recorded equivalence table (kernel checks, not re-run here) | 15 rows |
| `test_run.txt` | `cargo test --all-targets --all-features` in this repository | 84 tests passed |

## What was left out, and why

* The free-text evidence notes of the timing tables and the census. They carry repository paths and prose about internal work; the tables here keep every number and every classification.
* The kernel source, the analysers and checkers themselves, the landing gate and the validator.
* The validator's charter output and the full timing document: their notes carry internal paths.

## Public names

Crate names in the tables and outputs are the public names (the internal prefix of the crate names was replaced by `synthyrion_`). The SHA-512 of each checker output *as the tool wrote it*, before that substitution, is in `raw_output_hashes.txt`.

## Limits

The timing bounds are in MIR statements, not cycles. The census proves absence of panics, not correctness. 30 loop rows are assumed. The checkers are first-party and not verified. See the methods report.
