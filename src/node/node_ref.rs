use node::Node;
use tree::core::NodeId;
use tree::Tree;

pub struct NodeRef<'a, T: 'a> {
    pub(crate) node_id: NodeId,
    pub(crate) tree: &'a Tree<T>,
}

impl<'a, T: 'a> NodeRef<'a, T> {
    pub fn data(&self) -> &T {
        unsafe { &self.tree.get_node_unchecked(&self.node_id).data }
    }

    pub fn parent(&self) -> Option<NodeRef<T>> {
        self.get_self_as_node()
            .parent
            .clone()
            .map(move |parent_id| unsafe { self.tree.get_unchecked(&parent_id) })
    }

    pub fn prev_sibling(&self) -> Option<NodeRef<T>> {
        self.get_self_as_node()
            .prev_sibling
            .clone()
            .map(move |parent_id| unsafe { self.tree.get_unchecked(&parent_id) })
    }

    pub fn next_sibling(&self) -> Option<NodeRef<T>> {
        self.get_self_as_node()
            .next_sibling
            .clone()
            .map(move |parent_id| unsafe { self.tree.get_unchecked(&parent_id) })
    }

    pub fn first_child(&self) -> Option<NodeRef<T>> {
        self.get_self_as_node()
            .first_child
            .clone()
            .map(move |parent_id| unsafe { self.tree.get_unchecked(&parent_id) })
    }

    pub fn last_child(&self) -> Option<NodeRef<T>> {
        self.get_self_as_node()
            .last_child
            .clone()
            .map(move |parent_id| unsafe { self.tree.get_unchecked(&parent_id) })
    }

    fn get_self_as_node(&self) -> &Node<T> {
        unsafe { self.tree.get_node_unchecked(&self.node_id) }
    }
}

#[cfg(test)]
mod node_ref_tests {
    use tree::TreeBuilder;

    #[test]
    fn data() {
        let tree = TreeBuilder::new().with_root(1).build();
        let root_id = tree.root_id().cloned().unwrap();
        let root_ref = tree.get(&root_id).ok().unwrap();
        assert_eq!(root_ref.data(), &1);
    }

    #[test]
    fn parent() {
        let tree = TreeBuilder::new().with_root(1).build();
        let root_id = tree.root_id().cloned().unwrap();
        let root_ref = tree.get(&root_id).ok().unwrap();
        assert!(root_ref.parent().is_none());
    }

    #[test]
    fn prev_sibling() {
        let tree = TreeBuilder::new().with_root(1).build();
        let root_id = tree.root_id().cloned().unwrap();
        let root_ref = tree.get(&root_id).ok().unwrap();
        assert!(root_ref.prev_sibling().is_none());
    }

    #[test]
    fn next_sibling() {
        let tree = TreeBuilder::new().with_root(1).build();
        let root_id = tree.root_id().cloned().unwrap();
        let root_ref = tree.get(&root_id).ok().unwrap();
        assert!(root_ref.next_sibling().is_none());
    }

    #[test]
    fn first_child() {
        let tree = TreeBuilder::new().with_root(1).build();
        let root_id = tree.root_id().cloned().unwrap();
        let root_ref = tree.get(&root_id).ok().unwrap();
        assert!(root_ref.first_child().is_none());
    }

    #[test]
    fn last_child() {
        let tree = TreeBuilder::new().with_root(1).build();
        let root_id = tree.root_id().cloned().unwrap();
        let root_ref = tree.get(&root_id).ok().unwrap();
        assert!(root_ref.last_child().is_none());
    }
}
