mod node_mut;
mod node_ref;

pub use self::node_mut::NodeMut;
pub use self::node_ref::NodeRef;

use tree::core::NodeId;

#[derive(Clone)]
pub(crate) struct Relatives {
    pub(crate) parent: Option<NodeId>,
    pub(crate) prev_sibling: Option<NodeId>,
    pub(crate) next_sibling: Option<NodeId>,
    pub(crate) first_child: Option<NodeId>,
    pub(crate) last_child: Option<NodeId>,
}

pub(crate) struct Node<T> {
    pub(crate) data: T,
    pub(crate) relatives: Relatives,
}

impl<T> Node<T> {
    pub(crate) fn new(data: T) -> Node<T> {
        Node {
            data,
            relatives: Relatives {
                parent: None,
                prev_sibling: None,
                next_sibling: None,
                first_child: None,
                last_child: None,
            }
        }
    }
}
