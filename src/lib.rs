//! This crate is currently a binary crate, but it will soon switch to a library crate.

#![warn(unused_results, unused_extern_crates)]
#![deny(future_incompatible,)]

#[macro_use]
mod implmacro;

pub mod temperature;

pub mod length;

pub mod time;
pub mod mass;
pub mod electrical;

pub mod constants;