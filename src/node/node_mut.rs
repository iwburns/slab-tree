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

    pub fn parent(&mut self) -> Option<NodeMut<T>> {
        self.get_self_as_node()
            .parent
            .clone()
            .map(move |parent_id| unsafe { self.tree.get_unchecked_mut(&parent_id) })
    }

    pub fn prev_sibling(&mut self) -> Option<NodeMut<T>> {
        self.get_self_as_node()
            .prev_sibling
            .clone()
            .map(move |parent_id| unsafe { self.tree.get_unchecked_mut(&parent_id) })
    }

    pub fn next_sibling(&mut self) -> Option<NodeMut<T>> {
        self.get_self_as_node()
            .next_sibling
            .clone()
            .map(move |parent_id| unsafe { self.tree.get_unchecked_mut(&parent_id) })
    }

    pub fn first_child(&mut self) -> Option<NodeMut<T>> {
        self.get_self_as_node()
            .first_child
            .clone()
            .map(move |parent_id| unsafe { self.tree.get_unchecked_mut(&parent_id) })
    }

    pub fn last_child(&mut self) -> Option<NodeMut<T>> {
        self.get_self_as_node()
            .last_child
            .clone()
            .map(move |parent_id| unsafe { self.tree.get_unchecked_mut(&parent_id) })
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

    fn get_self_as_node(&self) -> &Node<T> {
        unsafe { self.tree.get_node_unchecked(&self.node_id) }
    }
}

#[cfg(test)]
mod node_mut_tests {
    use tree::TreeBuilder;

    #[test]
    fn data() {
        let mut tree = TreeBuilder::new().with_root(1).build();
        let root_id = tree.root_id().cloned().unwrap();

        let mut root_mut = tree.get_mut(&root_id).ok().unwrap();
        assert_eq!(root_mut.data(), &mut 1);

        *root_mut.data() = 2;
        assert_eq!(root_mut.data(), &mut 2);
    }

    #[test]
    fn parent() {
        let mut tree = TreeBuilder::new().with_root(1).build();
        let root_id = tree.root_id().cloned().unwrap();
        let mut root_mut = tree.get_mut(&root_id).ok().unwrap();
        assert!(root_mut.parent().is_none());
    }

    #[test]
    fn prev_sibling() {
        let mut tree = TreeBuilder::new().with_root(1).build();
        let root_id = tree.root_id().cloned().unwrap();
        let mut root_mut = tree.get_mut(&root_id).ok().unwrap();
        assert!(root_mut.prev_sibling().is_none());
    }

    #[test]
    fn next_sibling() {
        let mut tree = TreeBuilder::new().with_root(1).build();
        let root_id = tree.root_id().cloned().unwrap();
        let mut root_mut = tree.get_mut(&root_id).ok().unwrap();
        assert!(root_mut.next_sibling().is_none());
    }

    #[test]
    fn first_child() {
        let mut tree = TreeBuilder::new().with_root(1).build();
        let root_id = tree.root_id().cloned().unwrap();
        let mut root_mut = tree.get_mut(&root_id).ok().unwrap();
        assert!(root_mut.first_child().is_none());
    }

    #[test]
    fn last_child() {
        let mut tree = TreeBuilder::new().with_root(1).build();
        let root_id = tree.root_id().cloned().unwrap();
        let mut root_mut = tree.get_mut(&root_id).ok().unwrap();
        assert!(root_mut.last_child().is_none());
    }
}
