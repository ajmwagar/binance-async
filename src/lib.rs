#![deny(
    unstable_features,
    unsafe_code
)]

#[macro_use]
extern crate error_chain;

#[macro_use]
extern crate serde_derive;

mod client;
pub mod errors;
mod util;

pub mod model;

pub mod account;
pub mod api;
pub mod general;
pub mod market;
pub mod userstream;
pub mod websockets;

pub mod futures;
