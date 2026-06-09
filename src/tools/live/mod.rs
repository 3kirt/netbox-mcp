//! Live integration tests — gated behind the `live-tests` feature.
//!
//! These verify the one risk the unit tests structurally cannot: fidelity to
//! the *real* NetBox API (filter param names, response shapes, the fields we
//! strip). They run the actual domain functions against a live, seeded
//! instance and apply the same `slim_value` transform the rmcp boundary
//! applies, so assertions run on exactly what an MCP client receives.
//!
//! Reads rely on a deterministic fixture (`scripts/seed_data.py`) rather than
//! self-seeding: tests assert *behavior* (filters honored, invariants hold)
//! rather than exact counts, so they survive seed-data drift. The write tools
//! (e.g. VM create/update/delete) are exercised by self-cleaning lifecycle
//! tests that create an object, mutate it, and delete it within one test, so
//! they leave the seeded instance as they found it (a write-enabled token is
//! required; otherwise NetBox returns 403 and the create fails).
//!
//! See `docs/testing.md` for the philosophy and how to run these.

mod harness;

mod circuits;
mod core;
mod dcim;
mod errors;
mod extras;
mod ipam;
mod lookup;
mod pagination;
mod tenancy;
mod users;
mod virtualization;
mod vpn;
mod wireless;
