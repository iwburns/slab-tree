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
        let new_id = self.tree.core_tree.insert(data);
        let current_node_relatives = self.tree.get_node_relatives(&self.node_id);

        {
            let new_node = unsafe {
                self.tree.get_node_unchecked_mut(&new_id)
            };
            new_node.parent = Some(self.node_id.clone());
            new_node.prev_sibling = current_node_relatives.last_child.clone();
        }
        {
            let current_node = self.get_self_as_node_mut();
            current_node.first_child = current_node_relatives.first_child.or(Some(new_id.clone()));
            current_node.last_child = Some(new_id.clone());
        }

        {
            let mut new_node_mut = self.tree.get_mut(&new_id).ok().unwrap();
            new_node_mut.set_prev_siblings_next_sibling(new_id.clone());
        }

        return new_id;
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

    fn get_self_as_node_mut(&mut self) -> &mut Node<T> {
        unsafe { self.tree.get_node_unchecked_mut(&self.node_id) }
    }

    fn set_prev_siblings_next_sibling(&mut self, node_id: NodeId) {
        let prev = self.get_self_as_node().prev_sibling.clone();
        if let Some(prev_sibling) = prev.map(|id| unsafe { self.tree.get_node_unchecked_mut(&id) }) {
            prev_sibling.next_sibling = Some(node_id);
        }
    }

    fn set_next_sibling_prev_sibling(&mut self, node_id: NodeId) {
        let next = self.get_self_as_node().next_sibling.clone();
        if let Some(next_sibling) = next.map(|id| unsafe { self.tree.get_node_unchecked_mut(&id) }) {
            next_sibling.prev_sibling = Some(node_id);
        }
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

    #[test]
    fn append_no_children_present() {
        let mut tree = TreeBuilder::new().with_root(1).build();
        let root_id = tree.root_id().cloned().unwrap();

        let new_id;
        {
            let mut root_mut = tree.get_mut(&root_id).ok().unwrap();
            new_id = root_mut.append(2);
        }

        let root_node = unsafe { tree.get_node_unchecked(&root_id) };
        assert_eq!(root_node.first_child, Some(new_id.clone()));
        assert_eq!(root_node.last_child, Some(new_id.clone()));

        let new_node = unsafe { tree.get_node_unchecked(&new_id) };
        assert_eq!(new_node.parent, Some(root_id.clone()));
        assert_eq!(new_node.prev_sibling, None);
        assert_eq!(new_node.next_sibling, None);
        assert_eq!(new_node.first_child, None);
        assert_eq!(new_node.last_child, None);

        let root = tree.get(&root_id).ok().unwrap();
        assert_eq!(root.data(), &1);

        let new_node = root.first_child().unwrap();
        assert_eq!(new_node.data(), &2);
    }

    #[test]
    fn append_single_child_present() {
        let mut tree = TreeBuilder::new().with_root(1).build();
        let root_id = tree.root_id().cloned().unwrap();

        let new_id;
        let new_id_2;
        {
            let mut root_mut = tree.get_mut(&root_id).ok().unwrap();
            new_id = root_mut.append(2);
            new_id_2 = root_mut.append(3);
        }

        let root_node = unsafe { tree.get_node_unchecked(&root_id) };
        assert_eq!(root_node.first_child, Some(new_id.clone()));
        assert_eq!(root_node.last_child, Some(new_id_2.clone()));

        let new_node = unsafe { tree.get_node_unchecked(&new_id) };
        assert_eq!(new_node.parent, Some(root_id.clone()));
        assert_eq!(new_node.prev_sibling, None);
        assert_eq!(new_node.next_sibling, Some(new_id_2.clone()));
        assert_eq!(new_node.first_child, None);
        assert_eq!(new_node.last_child, None);

        let new_node_2 = unsafe { tree.get_node_unchecked(&new_id_2) };
        assert_eq!(new_node_2.parent, Some(root_id.clone()));
        assert_eq!(new_node_2.prev_sibling, Some(new_id.clone()));
        assert_eq!(new_node_2.next_sibling, None);
        assert_eq!(new_node_2.first_child, None);
        assert_eq!(new_node_2.last_child, None);

        let root = tree.get(&root_id).ok().unwrap();
        assert_eq!(root.data(), &1);

        let new_node = root.first_child().unwrap();
        assert_eq!(new_node.data(), &2);

        let new_node_2 = root.last_child().unwrap();
        assert_eq!(new_node_2.data(), &3);
    }

    #[test]
    fn append_two_children_present() {
        let mut tree = TreeBuilder::new().with_root(1).build();
        let root_id = tree.root_id().cloned().unwrap();

        let new_id;
        let new_id_2;
        let new_id_3;
        {
            let mut root_mut = tree.get_mut(&root_id).ok().unwrap();
            new_id = root_mut.append(2);
            new_id_2 = root_mut.append(3);
            new_id_3 = root_mut.append(4);
        }

        let root_node = unsafe { tree.get_node_unchecked(&root_id) };
        assert_eq!(root_node.first_child, Some(new_id.clone()));
        assert_eq!(root_node.last_child, Some(new_id_3.clone()));

        let new_node = unsafe { tree.get_node_unchecked(&new_id) };
        assert_eq!(new_node.parent, Some(root_id.clone()));
        assert_eq!(new_node.prev_sibling, None);
        assert_eq!(new_node.next_sibling, Some(new_id_2.clone()));
        assert_eq!(new_node.first_child, None);
        assert_eq!(new_node.last_child, None);

        let new_node_2 = unsafe { tree.get_node_unchecked(&new_id_2) };
        assert_eq!(new_node_2.parent, Some(root_id.clone()));
        assert_eq!(new_node_2.prev_sibling, Some(new_id.clone()));
        assert_eq!(new_node_2.next_sibling, Some(new_id_3.clone()));
        assert_eq!(new_node_2.first_child, None);
        assert_eq!(new_node_2.last_child, None);

        let new_node_3 = unsafe { tree.get_node_unchecked(&new_id_3) };
        assert_eq!(new_node_3.parent, Some(root_id.clone()));
        assert_eq!(new_node_3.prev_sibling, Some(new_id_2.clone()));
        assert_eq!(new_node_3.next_sibling, None);
        assert_eq!(new_node_3.first_child, None);
        assert_eq!(new_node_3.last_child, None);

        let root = tree.get(&root_id).ok().unwrap();
        assert_eq!(root.data(), &1);

        // left to right
        let new_node = root.first_child().unwrap();
        let new_node_2 = new_node.next_sibling().unwrap();
        let new_node_3 = new_node_2.next_sibling().unwrap();
        assert_eq!(new_node.data(), &2);
        assert_eq!(new_node_2.data(), &3);
        assert_eq!(new_node_3.data(), &4);

        // right to left
        let new_node_3 = root.last_child().unwrap();
        let new_node_2 = new_node_3.prev_sibling().unwrap();
        let new_node = new_node_2.prev_sibling().unwrap();
        assert_eq!(new_node_3.data(), &4);
        assert_eq!(new_node_2.data(), &3);
        assert_eq!(new_node.data(), &2);
    }
}
