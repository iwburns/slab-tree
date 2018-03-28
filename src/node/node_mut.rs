use tree::core::NodeId;
use tree::Tree;

pub struct NodeMut<'a, T: 'a> {
    pub(crate) node_id: NodeId,
    pub(crate) tree: &'a mut Tree<T>,
}

impl<'a, T: 'a> NodeMut<'a, T> {
    pub fn parent(&mut self) -> Option<NodeMut<T>> {
        // todo: fix when non-lexical-lifetimes comes out
        let parent_id;
        {
            let node = unsafe {
                self.tree.get_node_unchecked(&self.node_id)
            };
            parent_id = node.parent.clone()?;
        }
        let parent = unsafe {
            self.tree.get_unchecked_mut(&parent_id)
        };
        Some(parent)
    }

    pub fn append() {
        unimplemented!()
    }

    pub fn prepend() {
        unimplemented!()
    }

    pub fn remove_first() {
        unimplemented!()
    }

    pub fn remove_last() {
        unimplemented!()
    }
}