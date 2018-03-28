use tree::core::NodeId;
use tree::Tree;

pub struct NodeMut<'a, T: 'a> {
    pub(crate) node_id: NodeId,
    pub(crate) tree: &'a mut Tree<T>,
}

impl<'a, T: 'a> NodeMut<'a, T> {
    pub fn data(&mut self) -> &mut T { unimplemented!() }

    pub fn parent(&mut self) -> Option<NodeMut<T>> {
        let parent_id = {
            let node = unsafe { self.tree.get_node_unchecked(&self.node_id) };
            node.parent.clone()?
        };
        let parent = unsafe { self.tree.get_unchecked_mut(&parent_id) };
        Some(parent)
    }

    pub fn prev_sibling(&mut self) -> Option<NodeMut<T>> { unimplemented!() }
    pub fn next_sibling(&mut self) -> Option<NodeMut<T>> { unimplemented!() }
    pub fn first_child(&mut self) -> Option<NodeMut<T>> { unimplemented!() }
    pub fn last_child(&mut self) -> Option<NodeMut<T>> { unimplemented!() }
    pub fn append(&mut self, data: T) -> NodeId { unimplemented!() }
    pub fn prepend(&mut self, data: T) -> NodeId { unimplemented!() }
    pub fn remove_first(&mut self) -> Option<T> { unimplemented!() }
    pub fn remove_last(&mut self) -> Option<T> { unimplemented!() }
}