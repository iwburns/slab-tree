use tree::Tree;
use tree::core::NodeId;

pub struct NodeRef<'a, T: 'a> {
    pub(crate) node_id: NodeId,
    pub(crate) tree: &'a Tree<T>,
}

impl<'a, T: 'a> NodeRef<'a, T> {

}
