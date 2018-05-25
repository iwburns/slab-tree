use node::Node;
use tree::Tree;
use tree::core::NodeId;

pub struct NodeMut<'a, T: 'a> {
    pub(crate) node_id: NodeId,
    pub(crate) tree: &'a mut Tree<T>,
}

impl<'a, T: 'a> NodeMut<'a, T> {
    pub fn data(&mut self) -> &mut T {
        unsafe { &mut self.tree.get_node_unchecked_mut(&self.node_id).data }
    }

    fn get_self_as_node(&mut self) -> &Node<T> {
        unsafe { self.tree.get_node_unchecked(&self.node_id) }
    }

    pub fn parent(&mut self) -> Option<NodeMut<T>> {
        self.get_self_as_node()
            .parent
            .clone()
            .map(move |parent_id| unsafe { self.tree.get_unchecked_mut(&parent_id) })
    }

    pub fn prev_sibling(&mut self) -> Option<NodeMut<T>> {
        unimplemented!()
    }
    pub fn next_sibling(&mut self) -> Option<NodeMut<T>> {
        unimplemented!()
    }
    pub fn first_child(&mut self) -> Option<NodeMut<T>> {
        unimplemented!()
    }
    pub fn last_child(&mut self) -> Option<NodeMut<T>> {
        unimplemented!()
    }
    pub fn append(&mut self, data: T) -> NodeId {
        unimplemented!()
    }
    pub fn prepend(&mut self, data: T) -> NodeId {
        unimplemented!()
    }
    pub fn remove_first(&mut self) -> Option<T> {
        unimplemented!()
    }
    pub fn remove_last(&mut self) -> Option<T> {
        unimplemented!()
    }
}
