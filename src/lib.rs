#![forbid(unsafe_code)]

extern crate snowflake;

mod slab;
mod core_tree;
pub mod error;
pub mod iter;
pub mod node;
pub mod tree;

pub use tree::Tree;
pub use node::NodeRef;
pub use node::NodeMut;
pub use iter::Ancestors;
pub use iter::NextSiblings;
use snowflake::ProcessUniqueId;

///
/// An identifier used to differentiate between Nodes and tie
/// them to a specific tree.
///
#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub struct NodeId {
    tree_id: ProcessUniqueId,
    index: slab::Index,
}
