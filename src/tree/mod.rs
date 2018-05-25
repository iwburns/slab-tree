pub mod core;
pub mod error;

use self::core::CoreTree;
use self::core::NodeId;
use self::error::NodeIdError;
use node::Node;
use node::node_mut::NodeMut;
use node::node_ref::NodeRef;

//todo: document this

pub struct TreeBuilder<T> {
    root: Option<T>,
    capacity: Option<usize>,
}

impl<T> TreeBuilder<T> {
    pub fn new() -> TreeBuilder<T> {
        TreeBuilder {
            root: None,
            capacity: None,
        }
    }

    pub fn with_root(self, root: T) -> TreeBuilder<T> {
        TreeBuilder {
            root: Some(root),
            capacity: self.capacity,
        }
    }

    pub fn with_capacity(self, capacity: usize) -> TreeBuilder<T> {
        TreeBuilder {
            root: self.root,
            capacity: Some(capacity),
        }
    }

    pub fn build(self) -> Tree<T> {
        let mut core_tree = CoreTree::new(self.capacity.unwrap_or(0));
        let root_id = self.root.map(|data| core_tree.insert(data));

        Tree { root_id, core_tree }
    }
}

pub struct Tree<T> {
    root_id: Option<NodeId>,
    core_tree: CoreTree<T>,
}

impl<T> Tree<T> {
    pub fn new() -> Tree<T> {
        Tree {
            root_id: None,
            core_tree: CoreTree::new(0),
        }
    }

    pub fn root(&self) -> Option<NodeRef<T>> {
        self.root_id.clone().map(|id| self.new_node_ref(id))
    }

    pub fn root_mut(&mut self) -> Option<NodeMut<T>> {
        self.root_id.clone().map(move |id| self.new_node_mut(id))
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

#[cfg(test)]
mod tree_builder_tests {
    use super::*;

    #[test]
    fn with_root_and_capacity() {
        let tb = TreeBuilder::new().with_root(1).with_capacity(2);
        assert!(tb.root.is_some());
        assert_eq!(tb.root.unwrap(), 1);
        assert_eq!(tb.capacity.unwrap(), 2);
    }

    #[test]
    fn build() {
        let tree = TreeBuilder::new().with_root(1).with_capacity(2).build();
        assert!(tree.root_id.is_some());
        assert_eq!(tree.core_tree.capacity(), 2);
    }
}

#[cfg(test)]
mod tree_tests {
    use super::*;

    #[test]
    fn root() {
        {
            let tree: Tree<i32> = Tree::new();
            assert!(tree.root().is_none());
        }
        {
            let tree = TreeBuilder::new().with_root(1).build();
            assert!(tree.root().is_some());
            assert_eq!(tree.root().unwrap().data(), &1);
        }
    }

    #[test]
    fn root_mut() {
        {
            let mut tree: Tree<i32> = Tree::new();
            assert!(tree.root_mut().is_none());
        }
        {
            let mut tree = TreeBuilder::new().with_root(1).build();
            assert!(tree.root().is_some());
            assert_eq!(tree.root_mut().unwrap().data(), &mut 1);

            *tree.root_mut().unwrap().data() = 2;
            assert_eq!(tree.root_mut().unwrap().data(), &mut 2);
        }
    }
}
