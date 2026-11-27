> **Provenance.** This directory is the proof subject as published on the Rust package registry, version 1.0.0, adapted for this repository (statements below about versions published on the Rust package registry describe the earlier crate this one is adapted from, not a registry package of this name): the crate and module names are the public Synthyrion names, internal references in documentation were generalised, and the source is otherwise unchanged. The engine used inside the private kernel workspace differs from this snapshot in a small number of lines. See the repository README for what can and cannot be verified from here.

# synthyrion_verify_subject

**A proof *subject*, not a proof *tool*.**

This crate is a small, dependency-free access-control domain — badges, roles, clearances, wings of a
building, revocation. It exists for one reason: so that
`synthyrion_verify`'s **published** test suite can demonstrate the
claim that actually matters about a proof engine — *proving another crate's invariants*, not its own.

```text
  synthyrion_verify_subject   ← knows nothing about synthyrion_verify (zero dependencies, including it)
        ▲
        │ dev-dependency only
  synthyrion_verify           ← its integration tests reason ABOUT this crate
```

## Why this is a separate crate

A test in which an engine checks a predicate it also defined proves that the engine agrees with
itself. The interesting claim is different: point the engine at ordinary code that was written
without it in mind, and have it establish — or refute, *with a witness* — that code's stated
invariants over the whole of a finite domain.

That needs a subject the engine does not own. Before this crate existed, `synthyrion_verify`'s only
"proves another crate" tests lived inside a private workspace and used a private sibling crate by
path. `cargo publish` strips path-only dev-dependencies, so those two tests could never have shipped
in a form anyone outside could run. This crate is the fix: published first, depended on **by
version**, and therefore resolvable by everybody.

## What is worth proving here

Every invariant below is stated over a domain small enough that exhaustion is a genuine proof rather
than a sample — that is the whole point of the shape:

| # | Invariant | Domain |
|---|---|---|
| 1 | A role granted exactly one clearance holds that one **and no other** | all 10 of `Clearance::ALL` |
| 2 | Covering one wing never covers another | the product `Wing × Wing`, 144 cases |
| 3 | A revoked badge holds **no** clearance, and cannot be re-issued into authority | all 10 clearances |
| 4 | A badge that was never issued holds nothing, however generous the grants | all 10 clearances |
| 5 | `Door::code` and `Door::decode` are inverse | all 2³² codes — tier 5 territory |

Invariant 1 is the one worth dwelling on. The obvious authorization test — *grant `c`, assert the
holder may do `c`* — cannot fail in the direction that matters: it never looks at what leaked. The
proof here ranges over the **whole** `Clearance` domain and asserts `may(badge, x) == (x == granted)`
for every `x`, so a bitmask off-by-one surfaces as a concrete counterexample rather than a pass.

## Design choices, and why

- **No `alloc`.** Fixed-capacity arrays and bitmasks. Every operation has a stated, checkable bound
  instead of an implicit one, and the crate runs on bare metal.
- **Bitmask authority.** A role's clearances are one `u16`; a site's wings are one `u16`. This makes
  the leak questions sharp — a bitmask error is a leak, not a rounding error.
- **An uncommissioned site refuses everything.** Every mutator returns `false` and changes nothing
  before `commission()`, so "nothing happened" is distinguishable from "it worked". An
  access-control system that quietly accepts configuration while out of service is one whose
  configuration nobody can trust.
- **Revocation is explicit in both directions.** `revoke` drops the badge *and* records the
  revocation, so a later grant to the role it used to hold cannot re-arm it. The only way out of the
  revoked set is a deliberate `reinstate`.

## Usage

```rust
use synthyrion_verify_subject::{BadgeId, Clearance, Door, Site, ROLE_STAFF};

let mut s = Site::new(BadgeId(1), Door { building: 4, number: 12 }, "north-gate");
s.commission();
s.issue(BadgeId(2), ROLE_STAFF);
s.grant(ROLE_STAFF, Clearance::OpenDoor);

assert!(s.may(BadgeId(2), Clearance::OpenDoor));
assert!(!s.may(BadgeId(2), Clearance::DisarmAlarm)); // granted one, holds one
s.revoke(BadgeId(2));
assert!(!s.may(BadgeId(2), Clearance::OpenDoor));    // revocation is total
```

You are welcome to use it as an ordinary tiny access-control model, but its reason for existing is to
be reasoned *about*. The exhaustive proofs of the invariants above are in `synthyrion_verify`'s
`tests/connection.rs` and `tests/grouping.rs`.

## Stability

The version is `1.0.0` rather than `0.1.0` because the API is frozen by construction:
`Clearance::ALL` and `Wing::ALL` are the **denominators** quoted in `synthyrion_verify`'s published proofs.
Changing them would silently change what those proofs mean. Additive `1.x` releases are fine; a
breaking change to a proof subject is a `2.0`.

## License

MPL-2.0. Same as `synthyrion_verify`.
