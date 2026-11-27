// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! # synthyrion_verify_subject — a proof *subject*, not a proof *tool*
//!
//! This crate exists so that [`synthyrion_verify`](https://docs.rs/synthyrion_verify)'s published test suite can
//! demonstrate the thing that actually matters about a proof engine: **proving another crate's
//! invariants**, not its own.
//!
//! ## Why a separate crate at all
//!
//! A test in which an engine checks a predicate it also defined proves that the engine agrees with
//! itself. The interesting claim is different: *point the engine at a piece of ordinary code that was
//! written without it in mind, and it establishes — or refutes, with a witness — that code's stated
//! invariants over the whole of a finite domain.* That requires a subject the engine does not own.
//!
//! So the dependency arrow runs one way and only one way:
//!
//! ```text
//!   synthyrion_verify_subject   ← knows nothing about synthyrion_verify (zero dependencies, including it)
//!         ▲
//!         │ dev-dependency only
//!   synthyrion_verify           ← its integration tests reason ABOUT this crate
//! ```
//!
//! `synthyrion_verify_subject` does not depend on `synthyrion_verify`, does not import it, and contains no
//! verification machinery of any kind. It is a small access-control domain: badges, roles, clearances,
//! wings of a building, and revocation. Nothing in here is a mirror of the engine.
//!
//! ## What is worth proving here
//!
//! The invariants below are the ones an access-control system is *for*, and each is stated over a
//! finite domain small enough that exhaustion is a genuine proof rather than a sample:
//!
//! 1. **Grants are exact.** A role granted exactly one clearance holds that one and *no other* —
//!    the whole `Clearance` domain, not the granted one. (Checking only the granted clearance is the
//!    classic authorization test that cannot fail: it never looks at what leaked.)
//! 2. **Wings are independent.** Enabling one wing never enables another — over the *product*
//!    `Wing × Wing`, so a leak in any direction is visible.
//! 3. **Revocation is total.** A revoked badge holds *no* clearance afterwards, for every clearance,
//!    and cannot be re-issued into authority by a later grant to its old role.
//! 4. **Outsiders hold nothing.** A badge that was never issued holds no clearance, however
//!    generously the roles were granted.
//! 5. **Door codes round-trip.** [`Door::code`] and [`Door::decode`] are inverse over the entire
//!    building × number space, which is 2^32 — too large for tier 4, and exactly the shape tier 5
//!    reasons about symbolically.
//!
//! ## Deliberate design choices
//!
//! - **No `alloc`.** Fixed-capacity arrays and bitmasks, so the crate is usable on bare metal and so
//!   every operation has a stated, checkable capacity bound rather than an implicit one.
//! - **Bitmask authority.** A role's clearances are a single `u16`; a wing set is a single `u16`.
//!   This makes the leak questions above *sharp*: a bitmask off-by-one is a leak, not a rounding
//!   error, and the exhaustive checks catch it as a concrete counterexample.
//! - **Uncommissioned sites refuse everything.** Every mutator returns `false` before commissioning
//!   rather than silently succeeding, so "nothing happened" is distinguishable from "it worked".
//!
//! ```
//! use synthyrion_verify_subject::{BadgeId, Clearance, Door, Site, ROLE_STAFF};
//!
//! let mut s = Site::new(BadgeId(1), Door { building: 4, number: 12 }, "north-gate");
//! s.commission();
//! s.issue(BadgeId(2), ROLE_STAFF);
//! s.grant(ROLE_STAFF, Clearance::OpenDoor);
//!
//! assert!(s.may(BadgeId(2), Clearance::OpenDoor));
//! assert!(!s.may(BadgeId(2), Clearance::DisarmAlarm)); // granted one, holds one
//! s.revoke(BadgeId(2));
//! assert!(!s.may(BadgeId(2), Clearance::OpenDoor));    // revocation is total
//! ```

#![no_std]
#![forbid(unsafe_code)]
#![deny(missing_docs)]

/// A physical credential presented at a door. Opaque to the site: two badges are the same badge iff
/// their numbers match, and the number carries no authority by itself.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BadgeId(pub u32);

/// A controlled door, identified by the building it is in and its number within that building.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Door {
    /// Which building on the site.
    pub building: u16,
    /// Which door within that building.
    pub number: u16,
}

impl Door {
    /// Pack this door into the single 32-bit code a reader panel transmits.
    ///
    /// The inverse of [`decode`](Door::decode) over the entire `u32` space — a property with 2^32
    /// cases, which is why it is stated here rather than merely tested at a handful of points.
    pub const fn code(self) -> u32 {
        ((self.building as u32) << 16) | (self.number as u32)
    }

    /// Recover a door from the code a reader panel transmitted. Total: every `u32` is a valid code.
    pub const fn decode(code: u32) -> Door {
        Door {
            building: (code >> 16) as u16,
            number: (code & 0xFFFF) as u16,
        }
    }
}

/// A role identifier. Authority is attached to roles, never to badges directly, so revoking a badge
/// and re-scoping a role are separate operations that cannot be confused for one another.
pub type RoleId = u8;

/// The role held by whoever commissioned the site: may do everything, including issuing badges.
pub const ROLE_WARDEN: RoleId = 1;
/// Ordinary personnel.
pub const ROLE_STAFF: RoleId = 2;
/// Escorted, time-limited presence.
pub const ROLE_VISITOR: RoleId = 3;
/// Maintenance personnel, present outside hours.
pub const ROLE_CONTRACTOR: RoleId = 4;

/// The highest role identifier the site can hold authority for. A grant to a larger id is refused
/// rather than wrapped, so authority cannot be smuggled in through an out-of-range role.
pub const MAX_ROLE: RoleId = ROLE_CONTRACTOR;

/// How many badges one site can hold at once. Fixed because the crate does not allocate; `issue`
/// returns `false` when full rather than evicting, so a full site is a visible condition.
pub const MAX_BADGES: usize = 16;

/// Something a badge-holder is permitted to do on the site.
///
/// Finite and enumerable on purpose: [`Clearance::ALL`] is the domain the exhaustive proofs range
/// over, and a proof that ranges over the whole of it is a proof, not a sample.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Clearance {
    /// Pass through a controlled door during published hours.
    OpenDoor,
    /// Pass through a controlled door outside published hours.
    OpenDoorAfterHours,
    /// Silence an alarm that has already sounded.
    DisarmAlarm,
    /// Place a wing into or out of lockdown.
    SetLockdown,
    /// Issue a badge to somebody else.
    IssueBadge,
    /// Withdraw somebody else's badge.
    RevokeBadge,
    /// Read the door-event log.
    ReadLog,
    /// Delete entries from the door-event log.
    PurgeLog,
    /// Escort a visitor who holds no clearance of their own.
    EscortVisitor,
    /// Change which wings the site covers.
    ConfigureSite,
}

impl Clearance {
    /// Every clearance — the finite domain the exhaustive proofs range over.
    pub const ALL: [Clearance; 10] = [
        Clearance::OpenDoor,
        Clearance::OpenDoorAfterHours,
        Clearance::DisarmAlarm,
        Clearance::SetLockdown,
        Clearance::IssueBadge,
        Clearance::RevokeBadge,
        Clearance::ReadLog,
        Clearance::PurgeLog,
        Clearance::EscortVisitor,
        Clearance::ConfigureSite,
    ];

    /// This clearance's bit within a role's authority mask.
    const fn bit(self) -> u16 {
        1u16 << (self as u8)
    }
}

/// A physically distinct part of the site, controlled independently of the others.
///
/// Independence is the point: a site covers the wings it was configured for and no others, and
/// [`Wing::ALL`] makes "no others" a checkable claim over the whole product `Wing × Wing`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Wing {
    /// Public entrance and reception.
    Lobby,
    /// General office floors.
    Offices,
    /// Server and comms rooms.
    DataHall,
    /// Goods in and out.
    LoadingBay,
    /// Bulk storage.
    Warehouse,
    /// Wet labs.
    Laboratory,
    /// Clean rooms.
    CleanRoom,
    /// Records and archives.
    Archive,
    /// Plant, HVAC, electrical.
    PlantRoom,
    /// Roof and roof plant.
    Rooftop,
    /// Below-ground parking.
    Basement,
    /// Perimeter fence line and gates.
    Perimeter,
}

impl Wing {
    /// Every wing — the finite domain the exhaustive proofs range over.
    pub const ALL: [Wing; 12] = [
        Wing::Lobby,
        Wing::Offices,
        Wing::DataHall,
        Wing::LoadingBay,
        Wing::Warehouse,
        Wing::Laboratory,
        Wing::CleanRoom,
        Wing::Archive,
        Wing::PlantRoom,
        Wing::Rooftop,
        Wing::Basement,
        Wing::Perimeter,
    ];

    /// This wing's bit within a site's coverage mask.
    const fn bit(self) -> u16 {
        1u16 << (self as u8)
    }
}

/// One access-control site: which badges are issued, what each role may do, which wings are covered,
/// and which badges have been revoked.
///
/// Created uncommissioned. An uncommissioned site refuses every mutation — see [`Site::commission`].
#[derive(Clone, Debug)]
pub struct Site {
    warden: BadgeId,
    door: Door,
    name: &'static str,
    commissioned: bool,
    /// Issued badges: `(badge number, role)`. `None` is a free slot.
    holders: [Option<(u32, RoleId)>; MAX_BADGES],
    /// Per-role clearance bitmask, indexed by `RoleId`. Index 0 is unused (there is no role 0).
    role_clearances: [u16; MAX_ROLE as usize + 1],
    /// Coverage bitmask over [`Wing`].
    wings: u16,
    /// Revoked badge numbers. `None` is a free slot.
    revoked: [Option<u32>; MAX_BADGES],
}

impl Site {
    /// A fresh, uncommissioned site. `warden` is the badge that will hold [`ROLE_WARDEN`] once the
    /// site is commissioned; `door` is the site's principal entrance; `name` labels it in logs.
    pub const fn new(warden: BadgeId, door: Door, name: &'static str) -> Self {
        Site {
            warden,
            door,
            name,
            commissioned: false,
            holders: [None; MAX_BADGES],
            role_clearances: [0; MAX_ROLE as usize + 1],
            wings: 0,
            revoked: [None; MAX_BADGES],
        }
    }

    /// The site's principal entrance.
    pub const fn door(&self) -> Door {
        self.door
    }

    /// The site's label.
    pub const fn name(&self) -> &'static str {
        self.name
    }

    /// The badge that holds [`ROLE_WARDEN`].
    pub const fn warden(&self) -> BadgeId {
        self.warden
    }

    /// Bring the site into service and issue the warden their badge.
    ///
    /// Idempotent, and the precondition for every other mutation: before this call `issue`, `grant`,
    /// `revoke` and `cover_wing` all return `false` and change nothing. That is deliberate — an
    /// access-control system that quietly accepts configuration while out of service is one whose
    /// configuration nobody can trust.
    pub fn commission(&mut self) -> bool {
        if self.commissioned {
            return false;
        }
        self.commissioned = true;
        let w = self.warden.0;
        self.holders[0] = Some((w, ROLE_WARDEN));
        true
    }

    /// Whether the site is in service.
    pub const fn is_commissioned(&self) -> bool {
        self.commissioned
    }

    /// How many badges are currently issued.
    pub fn holder_count(&self) -> usize {
        self.holders.iter().filter(|h| h.is_some()).count()
    }

    /// The role currently attached to `badge`, if it holds one.
    pub fn role_of(&self, badge: BadgeId) -> Option<RoleId> {
        self.holders
            .iter()
            .flatten()
            .find(|(n, _)| *n == badge.0)
            .map(|(_, r)| *r)
    }

    /// Issue `badge` under `role`.
    ///
    /// Returns `false` — changing nothing — if the site is not commissioned, if `role` exceeds
    /// [`MAX_ROLE`], if the badge is already issued, if it has been revoked, or if the site is full.
    /// A revoked badge is never silently re-issued: revocation has to be undone explicitly by
    /// [`reinstate`](Site::reinstate).
    pub fn issue(&mut self, badge: BadgeId, role: RoleId) -> bool {
        if !self.commissioned || role == 0 || role > MAX_ROLE {
            return false;
        }
        if self.is_revoked(badge) || self.role_of(badge).is_some() {
            return false;
        }
        for slot in self.holders.iter_mut() {
            if slot.is_none() {
                *slot = Some((badge.0, role));
                return true;
            }
        }
        false
    }

    /// Grant `clearance` to every present and future holder of `role`.
    ///
    /// Returns `false` — changing nothing — if the site is not commissioned or `role` is out of
    /// range. Granting an already-granted clearance is a no-op that still reports `true`.
    pub fn grant(&mut self, role: RoleId, clearance: Clearance) -> bool {
        if !self.commissioned || role == 0 || role > MAX_ROLE {
            return false;
        }
        self.role_clearances[role as usize] |= clearance.bit();
        true
    }

    /// Withdraw `clearance` from `role`.
    pub fn ungrant(&mut self, role: RoleId, clearance: Clearance) -> bool {
        if !self.commissioned || role == 0 || role > MAX_ROLE {
            return false;
        }
        self.role_clearances[role as usize] &= !clearance.bit();
        true
    }

    /// Whether `badge` may exercise `clearance` right now.
    ///
    /// The whole authority decision, in one place: the badge must not be revoked, must be issued,
    /// and its role must hold the clearance. Nothing else confers authority — in particular, being
    /// the warden's badge number confers nothing until the site is commissioned and the badge issued.
    pub fn may(&self, badge: BadgeId, clearance: Clearance) -> bool {
        if self.is_revoked(badge) {
            return false;
        }
        match self.role_of(badge) {
            Some(role) if role <= MAX_ROLE => {
                self.role_clearances[role as usize] & clearance.bit() != 0
            }
            _ => false,
        }
    }

    /// Withdraw `badge` from the site entirely.
    ///
    /// Drops its issue record *and* records the revocation, so afterwards it holds no clearance and
    /// cannot be re-issued into authority by a later grant to the role it used to hold. Returns
    /// whether the badge was issued. Returns `false` if the site is not commissioned.
    pub fn revoke(&mut self, badge: BadgeId) -> bool {
        if !self.commissioned {
            return false;
        }
        let mut was_issued = false;
        for slot in self.holders.iter_mut() {
            if let Some((n, _)) = slot {
                if *n == badge.0 {
                    *slot = None;
                    was_issued = true;
                }
            }
        }
        if !self.is_revoked(badge) {
            for slot in self.revoked.iter_mut() {
                if slot.is_none() {
                    *slot = Some(badge.0);
                    break;
                }
            }
        }
        was_issued
    }

    /// Whether `badge` has been revoked.
    pub fn is_revoked(&self, badge: BadgeId) -> bool {
        self.revoked.iter().flatten().any(|n| *n == badge.0)
    }

    /// Lift a revocation, so the badge can be issued again. Explicit by design: the only way out of
    /// the revoked set is a deliberate act, never a side effect of another operation.
    pub fn reinstate(&mut self, badge: BadgeId) -> bool {
        if !self.commissioned {
            return false;
        }
        let mut found = false;
        for slot in self.revoked.iter_mut() {
            if *slot == Some(badge.0) {
                *slot = None;
                found = true;
            }
        }
        found
    }

    /// Put `wing` under this site's control.
    pub fn cover_wing(&mut self, wing: Wing) -> bool {
        if !self.commissioned {
            return false;
        }
        self.wings |= wing.bit();
        true
    }

    /// Whether this site controls `wing`.
    pub const fn covers(&self, wing: Wing) -> bool {
        self.wings & wing.bit() != 0
    }

    /// How many wings this site controls.
    pub const fn wing_count(&self) -> u32 {
        self.wings.count_ones()
    }

    /// A small preset: a single-building office site.
    pub fn office_preset(&mut self) {
        self.commission();
        self.cover_wing(Wing::Lobby);
        self.cover_wing(Wing::Offices);
        self.grant(ROLE_WARDEN, Clearance::OpenDoor);
        self.grant(ROLE_WARDEN, Clearance::OpenDoorAfterHours);
        self.grant(ROLE_WARDEN, Clearance::IssueBadge);
        self.grant(ROLE_WARDEN, Clearance::RevokeBadge);
        self.grant(ROLE_WARDEN, Clearance::ReadLog);
        self.grant(ROLE_STAFF, Clearance::OpenDoor);
        self.grant(ROLE_VISITOR, Clearance::EscortVisitor);
    }
}
