use tree::Tree;
use tree::core::NodeId;

pub struct NodeRef<'a, T: 'a> {
    pub(crate) node_id: NodeId,
    pub(crate) tree: &'a Tree<T>,
}

impl<'a, T: 'a> NodeRef<'a, T> {
    pub fn data(&self) -> &T { unimplemented!() }
    pub fn parent(&self) -> Option<NodeRef<T>> { unimplemented!() }
    pub fn prev_sibling(&self) -> Option<NodeRef<T>> { unimplemented!() }
    pub fn next_sibling(&self) -> Option<NodeRef<T>> { unimplemented!() }
    pub fn first_child(&self) -> Option<NodeRef<T>> { unimplemented!() }
    pub fn last_child(&self) -> Option<NodeRef<T>> { unimplemented!() }
    pub fn append(&self, data: T) -> NodeId { unimplemented!() }
    pub fn prepend(&self, data: T) -> NodeId { unimplemented!() }
    pub fn remove_first(&self) -> Option<T> { unimplemented!() }
    pub fn remove_last(&self) -> Option<T> { unimplemented!() }
}
