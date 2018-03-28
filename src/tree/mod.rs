pub mod core;
pub mod error;

use node::Node;
use node::node_ref::NodeRef;
use node::node_mut::NodeMut;
use self::core::CoreTree;
use self::core::NodeId;
use self::error::NodeIdError;

pub struct Tree<T> {
    root_id: Option<NodeId>,
    core_tree: CoreTree<T>,
}

// todo: make a builder for this

impl<T> Tree<T> {
    pub fn new() -> Tree<T> {
        Tree {
            root_id: None,
            core_tree: CoreTree::new(0),
        }
    }

    pub fn root(&self) -> Option<NodeRef<T>> {
        self.root_id
            .clone()
            .map(|id| self.new_node_ref(id))
    }

    pub fn root_mut(&mut self) -> Option<NodeMut<T>> {
        self.root_id
            .clone()
            .map(move |id| self.new_node_mut(id))
    }

    pub fn get(&self, node_id: &NodeId) -> Result<NodeRef<T>, NodeIdError> {
        let _ = self.core_tree.get(node_id)?;
        Ok(self.new_node_ref(node_id.clone()))
    }

    pub fn get_mut(&mut self, node_id: &NodeId) -> Result<NodeMut<T>, NodeIdError> {
        let _ = self.core_tree.get(node_id)?;
        Ok(self.new_node_mut(node_id.clone()))
    }

    pub unsafe fn get_unchecked(&self, node_id: &NodeId) -> NodeRef<T> {
        self.new_node_ref(node_id.clone())
    }

    pub unsafe fn get_unchecked_mut(&mut self, node_id: &NodeId) -> NodeMut<T> {
        self.new_node_mut(node_id.clone())
    }

    pub(crate) unsafe fn get_node_unchecked(&self, node_id: &NodeId) -> &Node<T> {
        self.core_tree.get_unchecked(node_id)
    }

    pub(crate) unsafe fn get_node_unchecked_mut(&mut self, node_id: &NodeId) -> &mut Node<T> {
        self.core_tree.get_unchecked_mut(node_id)
    }

    pub(crate) fn new_node_ref(&self, node_id: NodeId) -> NodeRef<T> {
        NodeRef {
            node_id,
            tree: self,
        }
    }

    pub(crate) fn new_node_mut(&mut self, node_id: NodeId) -> NodeMut<T> {
        NodeMut {
            node_id,
            tree: self,
        }
    }
}


