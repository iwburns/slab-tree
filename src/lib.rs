#![forbid(unsafe_code)]

extern crate snowflake;

//todo: run clippy and get rid of extra clones now that NodeIds are safely Copy

mod slab;
pub mod iter;
pub mod node;
pub mod tree;
