//! Reserved for a FHIR Release 1 (DSTU1, 0.0.82) data model.
//!
//! **Unlike the other reservations in this workspace, the specification here
//! is real.** DSTU1 was published in 2012 and superseded by DSTU2 in 2015.
//! What is absent is the model, not the release: a DSTU1 model was built in
//! this workspace and withdrawn before it was ever published, because
//! carrying a trial model of a 2012 draft was not worth its maintenance.
//!
//! DSTU1 shares less with its successors than any other pair of releases.
//! It has no `Bundle` — collections travelled as Atom feeds — so it cannot
//! implement the `Release` trait the REST client is generic over. It has no
//! `isSummary`, so there is no `_summary` view to project. Its resources are
//! named differently (`MedicationPrescription`, not `MedicationRequest`),
//! and its `OperationOutcome.issue` has no `code`. None of that is hard to
//! model; all of it is a permanent divergence to maintain.
//!
//! This crate holds the name so the model can be published here if it is
//! ever wanted, and so the crate family stays contiguous: one crate per
//! release, all siblings of
//! [`fhir-core`](https://crates.io/crates/fhir-core). A gap in the sequence
//! is an invitation for an unrelated crate to occupy the name that matches
//! this scheme.
//!
//! It contains no types, deliberately. A placeholder `Patient` would be a
//! guess, and a wrong guess is worse than an absent one for anything that
//! touches clinical data.
//!
//! For a real FHIR model today, use [`fhir`](https://crates.io/crates/fhir).

// Nothing is exported, and that is the point. `#![no_std]` keeps the crate
// honest: it cannot accidentally acquire a dependency or an API before there
// is a decision to model this release.
#![no_std]
