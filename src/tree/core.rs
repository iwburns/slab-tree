use snowflake::ProcessUniqueId;
use std::mem;

use node::Node;
use tree::error::*;
use slab;

///
/// An identifier used to differentiate between Nodes and tie
/// them to a specific tree.
///
#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub struct NodeId {
    tree_id: ProcessUniqueId,
    index: slab::Index,
}

///
/// A wrapper around a Slab containing Node<T> values.
///
/// Groups a collection of Node<T>s with a process unique id.
///
pub(crate) struct CoreTree<T> {
    id: ProcessUniqueId,
    slab: slab::Slab<Node<T>>,
}

impl<T> CoreTree<T> {
    pub(crate) fn new(capacity: usize) -> CoreTree<T> {
        CoreTree {
            id: ProcessUniqueId::new(),
            slab: slab::Slab::new(capacity),
        }
    }

    pub(crate) fn capacity(&self) -> usize {
        self.slab.capacity()
    }

    pub(crate) fn insert(&mut self, data: T) -> NodeId {
        let key = self.slab.insert(Node::new(data));
        self.new_node_id(key)
    }

    // todo: return an Option<T> here instead
    pub(crate) fn remove(&mut self, node_id: NodeId) -> T {
        let node = self.slab.remove(node_id.index).expect("Invalid NodeId");
        mem::drop(node_id);
        node.data
    }

    pub(crate) fn get(&self, node_id: &NodeId) -> Result<&Node<T>, NodeIdError> {
        self.validate_node_id(node_id)?;
        self.slab.get(node_id.index).ok_or(NodeIdError::BadNodeId)
    }

    pub(crate) fn get_mut(&mut self, node_id: &NodeId) -> Result<&mut Node<T>, NodeIdError> {
        self.validate_node_id(node_id)?;
        self.slab
            .get_mut(node_id.index)
            .ok_or(NodeIdError::BadNodeId)
    }

    fn new_node_id(&self, index: slab::Index) -> NodeId {
        NodeId {
            tree_id: self.id,
            index,
        }
    }

    fn validate_node_id(&self, node_id: &NodeId) -> Result<(), NodeIdError> {
        if node_id.tree_id != self.id {
            return Err(NodeIdError::WrongTree);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn capacity() {
        let capacity = 5;
        let tree = CoreTree::<i32>::new(capacity);
        assert_eq!(tree.capacity(), capacity);
    }

    #[test]
    fn insert() {
        let mut tree = CoreTree::new(0);

        let id = tree.insert(1);
        let id2 = tree.insert(3);

        assert_eq!(tree.get(&id).unwrap().data, 1);
        assert_eq!(tree.get(&id2).unwrap().data, 3);
    }

    #[test]
    fn remove() {
        let mut tree = CoreTree::new(0);

        let id = tree.insert(1);
        assert_eq!(tree.get(&id).unwrap().data, 1);

        let one = tree.remove(id);
        assert_eq!(one, 1);
    }

    #[test]
    fn get() {
        let mut tree = CoreTree::new(0);

        let id = tree.insert(1);
        let id2 = tree.insert(3);

        assert_eq!(tree.get(&id).unwrap().data, 1);
        assert_eq!(tree.get(&id2).unwrap().data, 3);
    }

    #[test]
    fn get_mut() {
        let mut tree = CoreTree::new(0);

        let id = tree.insert(1);
        let id2 = tree.insert(3);

        assert_eq!(tree.get_mut(&id).unwrap().data, 1);
        assert_eq!(tree.get_mut(&id2).unwrap().data, 3);
    }
}
