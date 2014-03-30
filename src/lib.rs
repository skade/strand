#![crate_type = "lib"]
#![crate_id = "strain#0.0.1"]
#![feature(globs,phase)]
#![phase(syntax, link)] extern crate log;

extern crate leveldb;

pub mod state;
pub mod mutable;
//pub mod immutable;
pub mod errors;
pub mod strain;
pub mod branchable;