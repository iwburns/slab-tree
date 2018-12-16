#![forbid(unsafe_code)]

extern crate snowflake;

mod core_tree;
pub mod error;
pub mod iter;
pub mod node;
mod slab;
pub mod tree;

pub use iter::Ancestors;
pub use iter::NextSiblings;
pub use node::NodeMut;
pub use node::NodeRef;
use snowflake::ProcessUniqueId;
pub use tree::Tree;

///
/// An identifier used to differentiate between Nodes and tie
/// them to a specific tree.
///
#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub struct NodeId {
    tree_id: ProcessUniqueId,
    index: slab::Index,
}
