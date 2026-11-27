# Synthyrion proofs

Proofs, results and checker outputs behind the Synthyrion Core L5 kernel, from Synthyrion Systems and Technologies. This repository holds **results, the method and a runnable proof engine. It does not contain the kernel source code.**

> **Status, plainly.** Synthyrion Core L5 is under active development. It has **no safety, security or government certification**, **no whole-kernel functional-correctness proof**, and is **not FIPS 140-3 validated**. Everything here is *tested*, *bounded* or *panic-free* evidence, labelled as such. A bounded result is a proof about its finite domain only. Nothing in this repository is a proof that the kernel is correct.

It accompanies two reports by Matthew Gelfman and Melissa Behrends-Gelfman: the architecture and security report, and the methods report (*How the Synthyrion kernel is proven and finished*). The architecture and security report is at https://doi.org/10.5281/zenodo.24581739 and the methods report at https://doi.org/10.5281/zenodo.24581779. Read the methods report first: it explains what each file below means and what it does not show.

## What is here

| Path | What it is |
|---|---|
| `results/bounded_checks.tsv` | Every bounded-proof statement, its domain, its size and its outcome: the published subject's proofs (re-run here) and the kernel's bounded refinement checks (as recorded, not re-runnable without the kernel source) |
| `results/census.tsv` | The panic-freedom census: one row per kernel decision function (5,659), with its state: proven, proven by induction, blocked (with the reason) or outside the checker's reach |
| `results/wcet_entries.tsv` | The timing registry: 140 kernel entry points, each bounded, pending (with the first obstacle the analyser reports) or reference-only, with its bound in MIR statements |
| `results/wcet_functions.tsv`, `wcet_loop_rows.tsv`, `wcet_callee_costs.tsv`, `wcet_constants.tsv`, `wcet_x86_cycles.tsv` | The tables a bound is computed from: functions, loop rows with the class of evidence that checks each, callee costs, enforced capacities, and the two cycle bounds on one compiled object |
| `results/checker_outputs/` | The checkers' own outputs (timing-analyser check, its missing-rows report, the census totals line, proof-source counts) |
| `results/MANIFEST.sha512`, `results/raw_output_hashes.txt` | SHA-512 of every published result file, and of the checker outputs as the tools wrote them |
| `results/MEASURED.md` | The commit and date every number was measured on, how each table was derived, what was left out |
| `results/test_run.txt` | The run of the engine's and the subject's test suites in this repository |
| `engine/`, `subject/` | The bounded proof engine and a small access-control subject it is demonstrated on (Rust, `no_std`, no dependencies, no unsafe code) |
| `tools/recount.py`, `tools/verify_manifest.py` | Re-derive every headline count from the published tables; check the manifest |

Units: a timing bound is counted in **MIR statements** (the compiler's mid-level intermediate representation), **not cycles**. Cycles exist for two entries only, on one compiled x86-64 object, under stated assumptions (`results/wcet_x86_cycles.tsv`).

## Run it

```text
python -I tools/verify_manifest.py        # SHA-512 of every result file
python -I tools/recount.py                # census, timing, loop-row and callee counts re-derived from the tables
cargo test --all-targets --all-features   # the engine's tests and the proofs about the subject (84 tests at the measured run)
```

The subject's invariants, for example *a role granted exactly one clearance holds that one and no other*, are proved by the engine over the whole domain: `engine/tests/connection.rs` and `engine/tests/grouping.rs`.

## What can and cannot be verified from this repository alone

| You can | You cannot (without the kernel source) |
|---|---|
| Re-run the engine and the proofs about its subject, from source | Re-run the panic-freedom checker, the timing analyser, the landing gate or the validator: they run on the kernel's source |
| Re-derive every count in the reports from the published tables (`tools/recount.py`) | Confirm that the tables are what the analyser produces from the current kernel: that needs the kernel source and the analyser |
| Check that no published result file was altered since measurement (SHA-512 manifest) | Re-run the kernel's bounded refinement checks or its test suites |
| Read, for each entry point, the bound, the loop rows behind it and the evidence class of each loop row | Judge the checkers' soundness: they are first-party and not verified |

**How to get the kernel source and re-run the proofs.** The kernel is distributed under the Synthyrion Core L5 source-available licence, version 2.3, published at <https://synthyrion.com/legal/licence.html>. Quoting that page: a licensee "may download, install, copy, compile, execute, test and use the Software for any lawful personal, educational, research, evaluation or development purpose, and in production on no more than one hundred (100) Active Devices while Licensee's Cumulative Gross Revenue is below US$100,000.00"; the licence "does not require Licensee to send Synthyrion any telemetry"; and Synthyrion "will not remotely disable, or distribute a version of the Software that stops booting because of, a licence check". No account is needed for the free tier. The signed release package contains the clean source export. **As of 27 November 2026 the site states that the previous download was withdrawn and that no download is available**; the next release follows the first phase of the kernel work and has no date. When a release is available, a reader who accepts the licence can obtain the source and re-run the checkers whose outputs are published here. Read the licence page for the controlling text; this paragraph is not a licence.

## Provenance and limits

* Measured on the private repository at commit `0368426a0fcb8282c74848615e991b157cfe12f7` (committed 2026-11-26) in a clean checkout; no kernel source, timing table or census file differed on the day this repository was prepared. `results/MEASURED.md` lists what was not published and why.
* The numbers change daily while the kernel is being finished. Each report states its measurement date; the website is the live record.
* `engine/` and `subject/` are adapted from the first author's earlier public crates (versions 3.9.0 and 1.0.0): crate and module names are the public names and internal references in the documentation were generalised; the source is otherwise unchanged. The engine used inside the private workspace differs from this snapshot by about 130 lines in the shared source files and has two further modules (666 lines in all) that this snapshot does not contain.
* Checker outputs and tables use the public crate names; the free-text notes of the tables, which carry repository paths, are not published.

## Licences

* `engine/` and `subject/`: Mozilla Public License 2.0, as published (`engine/LICENSE`, `subject/LICENSE`).
* `results/`, this README and `tools/`: Creative Commons Attribution 4.0 International (`LICENSE` at the repository root).
* The kernel itself is under the separate Synthyrion Core L5 licence above; this repository grants no rights in it.

## Citing

See `CITATION.cff`. Releases of this repository are archived on Zenodo, which assigns a DOI per release.
