use slab::Slab;
use snowflake::ProcessUniqueId;

use node::Node;
use tree::error::*;

// todo: document this

///
/// An identifier used to differentiate between Nodes and tie
/// them to a specific tree.
///
#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub struct NodeId {
    tree_id: ProcessUniqueId,
    index: usize,
}

///
/// A wrapper around a Slab containing Node<T> values.
///
/// Groups a collection of Node<T>s with a process unique id.
///
pub(crate) struct CoreTree<T> {
    id: ProcessUniqueId,
    slab: Slab<Node<T>>,
}

impl<T> CoreTree<T> {
    pub(crate) fn new(capacity: usize) -> CoreTree<T> {
        CoreTree {
            id: ProcessUniqueId::new(),
            slab: Slab::with_capacity(capacity),
        }
    }

    pub(crate) fn capacity(&self) -> usize {
        self.slab.capacity()
    }

    pub(crate) fn reserve(&mut self, additional: usize) {
        self.slab.reserve(additional);
    }

    pub(crate) fn reserve_exact(&mut self, additional: usize) {
        self.slab.reserve_exact(additional);
    }

    pub(crate) fn shrink_to_fit(&mut self) {
        self.slab.shrink_to_fit();
    }

    pub(crate) fn clear(&mut self) {
        self.slab.clear();
    }

    pub(crate) fn len(&self) -> usize {
        self.slab.len()
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.slab.is_empty()
    }

    pub(crate) fn insert(&mut self, node: Node<T>) -> NodeId {
        let key = self.slab.insert(node);
        self.new_node_id(key)
    }

    pub(crate) fn remove(&mut self, node_id: NodeId) -> Node<T> {
        self.slab.remove(node_id.index)
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

    pub(crate) unsafe fn get_unchecked(&self, node_id: &NodeId) -> &Node<T> {
        self.slab.get_unchecked(node_id.index)
    }

    pub(crate) unsafe fn get_unchecked_mut(&mut self, node_id: &NodeId) -> &mut Node<T> {
        self.slab.get_unchecked_mut(node_id.index)
    }

    fn new_node_id(&self, index: usize) -> NodeId {
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
    fn reserve() {
        let capacity = 1;
        let extra = 5;

        let mut tree = CoreTree::new(capacity);
        assert_eq!(tree.capacity(), capacity);

        tree.insert(Node::new(1));

        tree.reserve(extra);
        assert!(tree.capacity() >= capacity + extra);
    }

    #[test]
    fn reserve_exact() {
        let capacity = 1;
        let extra = 5;

        let mut tree = CoreTree::new(capacity);
        assert_eq!(tree.capacity(), capacity);

        tree.insert(Node::new(1));

        tree.reserve_exact(extra);
        assert_eq!(tree.capacity(), capacity + extra);
    }

    #[test]
    fn shrink_to_fit() {
        let capacity = 2;

        let mut tree = CoreTree::new(capacity);
        assert_eq!(tree.capacity(), capacity);

        tree.insert(Node::new(1));

        tree.shrink_to_fit();
        assert_eq!(tree.capacity(), 1);
    }

    #[test]
    fn clear() {
        let mut tree = CoreTree::new(0);

        let id = tree.insert(Node::new(1));
        assert_eq!(tree.get(&id).unwrap().data, 1);

        tree.clear();
        let res = tree.get(&id);

        assert!(res.is_err());
        assert_eq!(res.err().unwrap(), NodeIdError::BadNodeId);
    }

    #[test]
    fn len() {
        let mut tree = CoreTree::new(0);
        assert_eq!(tree.len(), 0);

        tree.insert(Node::new(1));
        assert_eq!(tree.len(), 1);

        tree.insert(Node::new(3));
        assert_eq!(tree.len(), 2);
    }

    #[test]
    fn is_empty() {
        let mut tree = CoreTree::new(0);
        assert!(tree.is_empty());

        tree.insert(Node::new(1));
        assert!(!tree.is_empty());
    }

    #[test]
    fn insert() {
        let mut tree = CoreTree::new(0);

        let id = tree.insert(Node::new(1));
        let id2 = tree.insert(Node::new(3));

        assert_eq!(tree.get(&id).unwrap().data, 1);
        assert_eq!(tree.get(&id2).unwrap().data, 3);
    }

    #[test]
    fn remove() {
        let mut tree = CoreTree::new(0);

        let id = tree.insert(Node::new(1));
        assert_eq!(tree.get(&id).unwrap().data, 1);

        let one = tree.remove(id);
        assert_eq!(one.data, 1);
    }

    #[test]
    fn get() {
        let mut tree = CoreTree::new(0);

        let id = tree.insert(Node::new(1));
        let id2 = tree.insert(Node::new(3));

        assert_eq!(tree.get(&id).unwrap().data, 1);
        assert_eq!(tree.get(&id2).unwrap().data, 3);
    }

    #[test]
    fn get_mut() {
        let mut tree = CoreTree::new(0);

        let id = tree.insert(Node::new(1));
        let id2 = tree.insert(Node::new(3));

        assert_eq!(tree.get_mut(&id).unwrap().data, 1);
        assert_eq!(tree.get_mut(&id2).unwrap().data, 3);
    }

    #[test]
    fn get_unchecked() {
        let mut tree = CoreTree::new(0);

        let id = tree.insert(Node::new(1));
        let id2 = tree.insert(Node::new(3));

        unsafe {
            assert_eq!(tree.get_unchecked(&id).data, 1);
        }
        unsafe {
            assert_eq!(tree.get_unchecked(&id2).data, 3);
        }
    }

    #[test]
    fn get_unchecked_mut() {
        let mut tree = CoreTree::new(0);

        let id = tree.insert(Node::new(1));
        let id2 = tree.insert(Node::new(3));

        unsafe {
            assert_eq!(tree.get_unchecked_mut(&id).data, 1);
        }
        unsafe {
            assert_eq!(tree.get_unchecked_mut(&id2).data, 3);
        }
    }
}
