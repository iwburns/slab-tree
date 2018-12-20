#![forbid(unsafe_code)]

mod core_tree;
pub mod iter;
pub mod node;
mod slab;
pub mod tree;

pub use crate::iter::Ancestors;
pub use crate::iter::NextSiblings;
pub use crate::node::NodeMut;
pub use crate::node::NodeRef;
pub use crate::tree::Tree;
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
