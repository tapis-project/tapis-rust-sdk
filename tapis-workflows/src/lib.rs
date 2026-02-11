#![allow(unused_imports)]
#![allow(clippy::too_many_arguments)]
#![allow(deprecated)]
#![allow(non_snake_case)]

extern crate serde_repr;
extern crate serde;
extern crate serde_json;
extern crate url;
extern crate reqwest;

pub mod apis;
pub mod models;
pub mod client;
pub use client::TapisWorkflows;
