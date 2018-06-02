use iter::Ancestors;
use iter::NextSiblings;
use node::Node;
use tree::core::NodeId;
use tree::Tree;

pub struct NodeRef<'a, T: 'a> {
    pub(crate) node_id: NodeId,
    pub(crate) tree: &'a Tree<T>,
}

impl<'a, T> NodeRef<'a, T> {
    pub fn data(&self) -> &T {
        unsafe { &self.tree.get_node_unchecked(&self.node_id).data }
    }

    pub fn parent(&self) -> Option<NodeRef<T>> {
        self.get_self_as_node()
            .parent
            .clone()
            .map(|id| self.tree.new_node_ref(id))
    }

    pub fn prev_sibling(&self) -> Option<NodeRef<T>> {
        self.get_self_as_node()
            .prev_sibling
            .clone()
            .map(|id| self.tree.new_node_ref(id))
    }

    pub fn next_sibling(&self) -> Option<NodeRef<T>> {
        self.get_self_as_node()
            .next_sibling
            .clone()
            .map(|id| self.tree.new_node_ref(id))
    }

    pub fn first_child(&self) -> Option<NodeRef<T>> {
        self.get_self_as_node()
            .first_child
            .clone()
            .map(|id| self.tree.new_node_ref(id))
    }

    pub fn last_child(&self) -> Option<NodeRef<T>> {
        self.get_self_as_node()
            .last_child
            .clone()
            .map(|id| self.tree.new_node_ref(id))
    }

    pub fn ancestors(&self) -> impl Iterator<Item = NodeRef<T>> {
        Ancestors::new(Some(self.node_id.clone()), self.tree)
    }

    pub fn children(&self) -> impl Iterator<Item = NodeRef<T>> {
        let first_child_id = self.tree.get_node_relatives(&self.node_id).first_child;
        NextSiblings::new(first_child_id, self.tree)
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

    #[test]
    fn ancestors() {
        let mut tree = TreeBuilder::new().with_root(1).build();
        let node_id;
        {
            let mut root_mut = tree.root_mut().unwrap();
            node_id = root_mut
                .append(2)
                .append(3)
                .append(4)
                .append(5)
                .node_id()
                .clone();
        }
        let tree = tree;

        let values = [4, 3, 2, 1];

        let bottom_node = tree.get(&node_id).ok().unwrap();
        for (i, node_ref) in bottom_node.ancestors().enumerate() {
            assert_eq!(node_ref.data(), &values[i]);
        }
    }

    #[test]
    fn children() {
        let mut tree = TreeBuilder::new().with_root(1).build();
        {
            let mut root_mut = tree.root_mut().unwrap();
            root_mut.append(2);
            root_mut.append(3);
            root_mut.append(4);
            root_mut.append(5);
        }
        let tree = tree;

        let values = [2, 3, 4, 5];
        let root = tree.root().unwrap();

        for (i, node_ref) in root.children().enumerate() {
            assert_eq!(node_ref.data(), &values[i]);
        }
    }
}
