pub mod node_mut;
pub mod node_ref;

use tree::core::NodeId;

// todo: possibly move relatives into Node

pub(crate) struct Relatives {
    pub(crate) parent: Option<NodeId>,
    pub(crate) prev_sibling: Option<NodeId>,
    pub(crate) next_sibling: Option<NodeId>,
    pub(crate) first_child: Option<NodeId>,
    pub(crate) last_child: Option<NodeId>,
}

pub(crate) struct Node<T> {
    pub(crate) data: T,
    pub(crate) parent: Option<NodeId>,
    pub(crate) prev_sibling: Option<NodeId>,
    pub(crate) next_sibling: Option<NodeId>,
    pub(crate) first_child: Option<NodeId>,
    pub(crate) last_child: Option<NodeId>,
}

impl<T> Node<T> {
    pub(crate) fn new(data: T) -> Node<T> {
        Node {
            data,
            parent: None,
            prev_sibling: None,
            next_sibling: None,
            first_child: None,
            last_child: None,
        }
    }
}
