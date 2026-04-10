//! `tapis-core` — shared traits and types for the Tapis Rust SDK.
//!
//! This crate is intentionally tiny and has no network or HTTP dependencies.
//! Every service crate (tapis-jobs, tapis-systems, …) depends **only** on this
//! crate for the token-refresh contract, which keeps the dependency graph
//! acyclic: `tapis-authenticator` can implement [`TokenProvider`] and hand the
//! implementation into any service client without any service crate needing to
//! know about `tapis-authenticator`.

pub mod token;
pub use token::TokenProvider;
