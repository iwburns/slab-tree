use crate::node::Node;
use crate::slab;
use crate::NodeId;
use snowflake::ProcessUniqueId;

///
/// A wrapper around a Slab containing Node<T> values.
///
/// Groups a collection of Node<T>s with a process unique id.
///
#[derive(Debug, PartialEq, Clone)]
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

    pub(crate) fn remove(&mut self, node_id: NodeId) -> Option<T> {
        self.filter_by_tree_id(node_id)
            .and_then(|id| self.slab.remove(id.index))
            .map(|node| node.data)
    }

    pub(crate) fn get(&self, node_id: NodeId) -> Option<&Node<T>> {
        self.filter_by_tree_id(node_id)
            .and_then(|id| self.slab.get(id.index))
    }

    pub(crate) fn get_mut(&mut self, node_id: NodeId) -> Option<&mut Node<T>> {
        self.filter_by_tree_id(node_id)
            .and_then(move |id| self.slab.get_mut(id.index))
    }

    fn new_node_id(&self, index: slab::Index) -> NodeId {
        NodeId {
            tree_id: self.id,
            index,
        }
    }

    fn filter_by_tree_id(&self, node_id: NodeId) -> Option<NodeId> {
        if node_id.tree_id != self.id {
            return None;
        }
        Some(node_id)
    }
}

#[cfg_attr(tarpaulin, skip)]
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

        assert_eq!(tree.get(id).unwrap().data, 1);
        assert_eq!(tree.get(id2).unwrap().data, 3);
    }

    #[test]
    fn remove() {
        let mut tree = CoreTree::new(0);

        let id = tree.insert(1);
        assert_eq!(tree.get(id).unwrap().data, 1);

        let one = tree.remove(id);
        assert!(one.is_some());

        let one = one.unwrap();
        assert_eq!(one, 1);
    }

    #[test]
    fn get() {
        let mut tree = CoreTree::new(0);

        let id = tree.insert(1);
        let id2 = tree.insert(3);

        assert_eq!(tree.get(id).unwrap().data, 1);
        assert_eq!(tree.get(id2).unwrap().data, 3);
    }

    #[test]
    fn get_mut() {
        let mut tree = CoreTree::new(0);

        let id = tree.insert(1);
        let id2 = tree.insert(3);

        assert_eq!(tree.get_mut(id).unwrap().data, 1);
        assert_eq!(tree.get_mut(id2).unwrap().data, 3);
    }

    #[test]
    fn get_with_bad_id() {
        let mut tree = CoreTree::new(0);
        let tree2: CoreTree<i32> = CoreTree::new(0);

        let mut id = tree.insert(1);
        id.tree_id = tree2.id; // oops, wrong tree id.

        let result = tree.get(id);

        assert!(result.is_none());
    }
}
