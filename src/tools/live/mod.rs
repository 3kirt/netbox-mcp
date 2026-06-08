//! Live integration tests — gated behind the `live-tests` feature.
//!
//! These verify the one risk the unit tests structurally cannot: fidelity to
//! the *real* NetBox API (filter param names, response shapes, the fields we
//! strip). They run the actual domain functions against a live, seeded
//! instance and apply the same `slim_value` transform the rmcp boundary
//! applies, so assertions run on exactly what an MCP client receives.
//!
//! Because the server is read-only, the suite cannot self-seed via the API the
//! way a write-capable server would. Instead the deterministic fixture is
//! `scripts/seed_data.py`; tests assert *behavior* (filters honored, invariants
//! hold) rather than exact counts, so they survive seed-data drift.
//!
//! See `docs/testing.md` for the philosophy and how to run these.

mod harness;

mod dcim;
mod extras;
mod ipam;
mod lookup;
mod tenancy;
mod virtualization;
