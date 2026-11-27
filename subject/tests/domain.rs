// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Concrete anchors for the domain.
//!
//! These are ordinary example-based tests, and they are deliberately *not* the interesting ones. The
//! exhaustive proofs of this crate's invariants live in `synthyrion_verify`'s `tests/connection.rs` and
//! `tests/grouping.rs`, where the engine ranges over the whole of [`Clearance::ALL`] and
//! `Wing::ALL × Wing::ALL`. What is here pins the behaviour those proofs assume — that the
//! operations do anything at all — so a proof cannot pass by ranging over a domain in which nothing
//! ever happens.

use synthyrion_verify_subject::{
    BadgeId, Clearance, Door, Site, Wing, MAX_BADGES, MAX_ROLE, ROLE_STAFF, ROLE_VISITOR,
    ROLE_WARDEN,
};

fn site() -> Site {
    Site::new(
        BadgeId(1),
        Door {
            building: 4,
            number: 12,
        },
        "north-gate",
    )
}

#[test]
fn an_uncommissioned_site_refuses_every_mutation() {
    let mut s = site();
    assert!(!s.is_commissioned());
    assert!(!s.issue(BadgeId(2), ROLE_STAFF));
    assert!(!s.grant(ROLE_STAFF, Clearance::OpenDoor));
    assert!(!s.revoke(BadgeId(2)));
    assert!(!s.cover_wing(Wing::Lobby));
    assert!(!s.reinstate(BadgeId(2)));
    assert_eq!(s.holder_count(), 0);
    assert_eq!(s.wing_count(), 0);
    // And nothing leaked through the refusals.
    for c in Clearance::ALL {
        assert!(!s.may(BadgeId(2), c));
        assert!(!s.may(s.warden(), c));
    }
}

#[test]
fn commissioning_issues_the_warden_badge_and_is_idempotent() {
    let mut s = site();
    assert!(s.commission());
    assert!(!s.commission(), "second commission is a no-op, and says so");
    assert_eq!(s.holder_count(), 1);
    assert_eq!(s.role_of(s.warden()), Some(ROLE_WARDEN));
}

#[test]
fn a_grant_is_visible_and_an_ungrant_undoes_it() {
    let mut s = site();
    s.commission();
    s.issue(BadgeId(2), ROLE_STAFF);
    assert!(
        !s.may(BadgeId(2), Clearance::OpenDoor),
        "nothing by default"
    );
    assert!(s.grant(ROLE_STAFF, Clearance::OpenDoor));
    assert!(s.may(BadgeId(2), Clearance::OpenDoor));
    assert!(s.ungrant(ROLE_STAFF, Clearance::OpenDoor));
    assert!(!s.may(BadgeId(2), Clearance::OpenDoor));
}

#[test]
fn an_out_of_range_role_cannot_be_issued_or_granted() {
    let mut s = site();
    s.commission();
    let bad = MAX_ROLE + 1;
    assert!(!s.issue(BadgeId(2), bad));
    assert!(!s.grant(bad, Clearance::OpenDoor));
    assert!(!s.issue(BadgeId(3), 0), "there is no role 0");
    assert!(!s.grant(0, Clearance::OpenDoor));
    assert_eq!(s.holder_count(), 1, "only the warden");
}

#[test]
fn a_revoked_badge_is_not_silently_reissued() {
    let mut s = site();
    s.commission();
    s.grant(ROLE_STAFF, Clearance::OpenDoor);
    assert!(s.issue(BadgeId(2), ROLE_STAFF));
    assert!(s.revoke(BadgeId(2)));
    assert!(s.is_revoked(BadgeId(2)));
    assert!(
        !s.issue(BadgeId(2), ROLE_STAFF),
        "re-issue must be refused while revoked"
    );
    assert!(!s.may(BadgeId(2), Clearance::OpenDoor));
    // Only an explicit reinstatement opens the door again.
    assert!(s.reinstate(BadgeId(2)));
    assert!(s.issue(BadgeId(2), ROLE_STAFF));
    assert!(s.may(BadgeId(2), Clearance::OpenDoor));
}

#[test]
fn the_badge_table_is_bounded_and_reports_being_full() {
    let mut s = site();
    s.commission(); // takes one slot
    for i in 0..(MAX_BADGES as u32 - 1) {
        assert!(s.issue(BadgeId(100 + i), ROLE_VISITOR), "slot {i}");
    }
    assert_eq!(s.holder_count(), MAX_BADGES);
    assert!(
        !s.issue(BadgeId(9999), ROLE_VISITOR),
        "a full site refuses rather than evicting"
    );
    assert!(s.role_of(BadgeId(9999)).is_none());
}

#[test]
fn wings_are_set_independently() {
    let mut s = site();
    s.commission();
    assert!(s.cover_wing(Wing::DataHall));
    assert!(s.covers(Wing::DataHall));
    assert!(!s.covers(Wing::Lobby));
    assert_eq!(s.wing_count(), 1);
}

#[test]
fn door_codes_round_trip_at_the_boundaries() {
    for d in [
        Door {
            building: 0,
            number: 0,
        },
        Door {
            building: 0,
            number: u16::MAX,
        },
        Door {
            building: u16::MAX,
            number: 0,
        },
        Door {
            building: u16::MAX,
            number: u16::MAX,
        },
        Door {
            building: 4,
            number: 12,
        },
    ] {
        assert_eq!(Door::decode(d.code()), d, "{d:?}");
    }
    assert_eq!(Door::decode(0xFFFF_0000).building, u16::MAX);
    assert_eq!(Door::decode(0x0000_FFFF).number, u16::MAX);
}

#[test]
fn the_office_preset_produces_a_usable_site() {
    let mut s = site();
    s.office_preset();
    s.issue(BadgeId(2), ROLE_STAFF);
    assert!(s.may(BadgeId(2), Clearance::OpenDoor));
    assert!(!s.may(BadgeId(2), Clearance::OpenDoorAfterHours));
    assert!(s.may(s.warden(), Clearance::OpenDoorAfterHours));
    assert_eq!(s.wing_count(), 2);
}

#[test]
fn the_two_finite_domains_are_the_whole_enums() {
    // If ALL ever fell behind its enum, every exhaustive proof over it would silently narrow. These
    // are the denominators the proofs in synthyrion_verify quote, so they are asserted here.
    assert_eq!(Clearance::ALL.len(), 10);
    assert_eq!(Wing::ALL.len(), 12);
    let mut cs = Clearance::ALL;
    cs.sort();
    cs.windows(2)
        .for_each(|w| assert_ne!(w[0], w[1], "duplicate clearance in ALL"));
    let mut ws = Wing::ALL;
    ws.sort();
    ws.windows(2)
        .for_each(|w| assert_ne!(w[0], w[1], "duplicate wing in ALL"));
}
