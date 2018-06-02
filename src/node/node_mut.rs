use node::Node;
use tree::core::NodeId;
use tree::Tree;

pub struct NodeMut<'a, T: 'a> {
    pub(crate) node_id: NodeId,
    pub(crate) tree: &'a mut Tree<T>,
}

impl<'a, T: 'a> NodeMut<'a, T> {
    pub fn node_id(&self) -> &NodeId {
        &self.node_id
    }

    pub fn data(&mut self) -> &mut T {
        unsafe { &mut self.tree.get_node_unchecked_mut(&self.node_id).data }
    }

    pub fn parent(&mut self) -> Option<NodeMut<T>> {
        self.get_self_as_node()
            .parent
            .clone()
            .map(move |id| self.tree.new_node_mut(id))
    }

    pub fn prev_sibling(&mut self) -> Option<NodeMut<T>> {
        self.get_self_as_node()
            .prev_sibling
            .clone()
            .map(move |id| self.tree.new_node_mut(id))
    }

    pub fn next_sibling(&mut self) -> Option<NodeMut<T>> {
        self.get_self_as_node()
            .next_sibling
            .clone()
            .map(move |id| self.tree.new_node_mut(id))
    }

    pub fn first_child(&mut self) -> Option<NodeMut<T>> {
        self.get_self_as_node()
            .first_child
            .clone()
            .map(move |id| self.tree.new_node_mut(id))
    }

    pub fn last_child(&mut self) -> Option<NodeMut<T>> {
        self.get_self_as_node()
            .last_child
            .clone()
            .map(move |id| self.tree.new_node_mut(id))
    }

    pub fn append(&mut self, data: T) -> NodeMut<T> {
        let new_id = self.tree.core_tree.insert(data);

        let current_id = &self.node_id;
        let relatives = self.tree.get_node_relatives(current_id);

        let prev_sibling = relatives.last_child.clone();
        self.tree.set_parent(&new_id, Some(current_id.clone()));
        self.tree.set_prev_sibling(&new_id, prev_sibling.clone());

        let first_child = relatives.first_child.or_else(|| Some(new_id.clone()));
        self.tree.set_first_child(current_id, first_child);
        self.tree.set_last_child(current_id, Some(new_id.clone()));

        if let Some(node_id) = prev_sibling {
            self.tree.set_next_sibling(&node_id, Some(new_id.clone()));
        }

        self.tree.new_node_mut(new_id)
    }

    pub fn prepend(&mut self, data: T) -> NodeMut<T> {
        let new_id = self.tree.core_tree.insert(data);

        let current_id = &self.node_id;
        let relatives = self.tree.get_node_relatives(&self.node_id);

        let next_sibling = relatives.first_child.clone();
        self.tree.set_parent(&new_id, Some(current_id.clone()));
        self.tree.set_next_sibling(&new_id, next_sibling.clone());

        let last_child = relatives.last_child.or_else(|| Some(new_id.clone()));
        self.tree.set_first_child(current_id, Some(new_id.clone()));
        self.tree.set_last_child(current_id, last_child);

        if let Some(node_id) = next_sibling {
            self.tree.set_prev_sibling(&node_id, Some(new_id.clone()));
        }

        self.tree.new_node_mut(new_id)
    }

    pub fn remove_first(&mut self) -> Option<T> {
        let current_id = &self.node_id;
        let current_relatives = self.tree.get_node_relatives(current_id);

        let first = current_relatives.first_child;
        let last = current_relatives.last_child;

        let first_id;
        if first == last {
            first_id = first?;
            self.tree.set_first_child(current_id, None);
            self.tree.set_last_child(current_id, None);
        } else {
            first_id = first?;
            let first_child = self.tree.get_node_relatives(&first_id).next_sibling;

            self.tree.set_first_child(current_id, first_child);
            self.tree.set_next_siblings_prev_sibling(&first_id, None);
        }

        Some(self.tree.core_tree.remove(first_id))
    }

    pub fn remove_last(&mut self) -> Option<T> {
        let current_id = &self.node_id;
        let current_node_relatives = self.tree.get_node_relatives(current_id);

        let first = current_node_relatives.first_child;
        let last = current_node_relatives.last_child;

        let last_id;
        if first == last {
            last_id = last?;
            self.tree.set_first_child(current_id, None);
            self.tree.set_last_child(current_id, None);
        } else {
            last_id = last?;
            let last_child = self.tree.get_node_relatives(&last_id).prev_sibling;

            self.tree.set_last_child(current_id, last_child);
            self.tree.set_prev_siblings_next_sibling(&last_id, None);
        }

        Some(self.tree.core_tree.remove(last_id))
    }

    fn get_self_as_node(&self) -> &Node<T> {
        unsafe { self.tree.get_node_unchecked(&self.node_id) }
    }
}

#[cfg(test)]
mod node_mut_tests {
    use tree::TreeBuilder;

    #[test]
    fn node_id() {
        let mut tree = TreeBuilder::new().with_root(1).build();
        let root_id = tree.root_id().cloned().unwrap();

        let root_mut = tree.get_mut(&root_id).ok().unwrap();
        assert_eq!(&root_id, root_mut.node_id());
    }

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
            new_id = root_mut.append(2).node_id().clone();
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
            new_id = root_mut.append(2).node_id().clone();
            new_id_2 = root_mut.append(3).node_id().clone();
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
            new_id = root_mut.append(2).node_id().clone();
            new_id_2 = root_mut.append(3).node_id().clone();
            new_id_3 = root_mut.append(4).node_id().clone();
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

    #[test]
    fn prepend_no_children_present() {
        let mut tree = TreeBuilder::new().with_root(1).build();
        let root_id = tree.root_id().cloned().unwrap();

        let new_id;
        {
            let mut root_mut = tree.get_mut(&root_id).ok().unwrap();
            new_id = root_mut.prepend(2).node_id().clone();
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
    fn prepend_single_child_present() {
        let mut tree = TreeBuilder::new().with_root(1).build();
        let root_id = tree.root_id().cloned().unwrap();

        let new_id;
        let new_id_2;
        {
            let mut root_mut = tree.get_mut(&root_id).ok().unwrap();
            new_id = root_mut.prepend(2).node_id().clone();
            new_id_2 = root_mut.prepend(3).node_id().clone();
        }

        let root_node = unsafe { tree.get_node_unchecked(&root_id) };
        assert_eq!(root_node.first_child, Some(new_id_2.clone()));
        assert_eq!(root_node.last_child, Some(new_id.clone()));

        let new_node = unsafe { tree.get_node_unchecked(&new_id) };
        assert_eq!(new_node.parent, Some(root_id.clone()));
        assert_eq!(new_node.prev_sibling, Some(new_id_2.clone()));
        assert_eq!(new_node.next_sibling, None);
        assert_eq!(new_node.first_child, None);
        assert_eq!(new_node.last_child, None);

        let new_node_2 = unsafe { tree.get_node_unchecked(&new_id_2) };
        assert_eq!(new_node_2.parent, Some(root_id.clone()));
        assert_eq!(new_node_2.prev_sibling, None);
        assert_eq!(new_node_2.next_sibling, Some(new_id.clone()));
        assert_eq!(new_node_2.first_child, None);
        assert_eq!(new_node_2.last_child, None);

        let root = tree.get(&root_id).ok().unwrap();
        assert_eq!(root.data(), &1);

        let new_node = root.first_child().unwrap();
        assert_eq!(new_node.data(), &3);

        let new_node_2 = root.last_child().unwrap();
        assert_eq!(new_node_2.data(), &2);
    }

    #[test]
    fn prepend_two_children_present() {
        let mut tree = TreeBuilder::new().with_root(1).build();
        let root_id = tree.root_id().cloned().unwrap();

        let new_id;
        let new_id_2;
        let new_id_3;
        {
            let mut root_mut = tree.get_mut(&root_id).ok().unwrap();
            new_id = root_mut.prepend(2).node_id().clone();
            new_id_2 = root_mut.prepend(3).node_id().clone();
            new_id_3 = root_mut.prepend(4).node_id().clone();
        }

        let root_node = unsafe { tree.get_node_unchecked(&root_id) };
        assert_eq!(root_node.first_child, Some(new_id_3.clone()));
        assert_eq!(root_node.last_child, Some(new_id.clone()));

        let new_node = unsafe { tree.get_node_unchecked(&new_id) };
        assert_eq!(new_node.parent, Some(root_id.clone()));
        assert_eq!(new_node.prev_sibling, Some(new_id_2.clone()));
        assert_eq!(new_node.next_sibling, None);
        assert_eq!(new_node.first_child, None);
        assert_eq!(new_node.last_child, None);

        let new_node_2 = unsafe { tree.get_node_unchecked(&new_id_2) };
        assert_eq!(new_node_2.parent, Some(root_id.clone()));
        assert_eq!(new_node_2.prev_sibling, Some(new_id_3.clone()));
        assert_eq!(new_node_2.next_sibling, Some(new_id.clone()));
        assert_eq!(new_node_2.first_child, None);
        assert_eq!(new_node_2.last_child, None);

        let new_node_3 = unsafe { tree.get_node_unchecked(&new_id_3) };
        assert_eq!(new_node_3.parent, Some(root_id.clone()));
        assert_eq!(new_node_3.prev_sibling, None);
        assert_eq!(new_node_3.next_sibling, Some(new_id_2.clone()));
        assert_eq!(new_node_3.first_child, None);
        assert_eq!(new_node_3.last_child, None);

        let root = tree.get(&root_id).ok().unwrap();
        assert_eq!(root.data(), &1);

        // left to right
        let new_node_3 = root.first_child().unwrap();
        let new_node_2 = new_node_3.next_sibling().unwrap();
        let new_node = new_node_2.next_sibling().unwrap();
        assert_eq!(new_node_3.data(), &4);
        assert_eq!(new_node_2.data(), &3);
        assert_eq!(new_node.data(), &2);

        // right to left
        let new_node = root.last_child().unwrap();
        let new_node_2 = new_node.prev_sibling().unwrap();
        let new_node_3 = new_node_2.prev_sibling().unwrap();
        assert_eq!(new_node.data(), &2);
        assert_eq!(new_node_2.data(), &3);
        assert_eq!(new_node_3.data(), &4);
    }

    #[test]
    fn remove_first_no_children_present() {
        let mut tree = TreeBuilder::new().with_root(1).build();
        let root_id = tree.root_id().cloned().unwrap();

        {
            let mut root_mut = tree.get_mut(&root_id).ok().unwrap();
            let first_child_data = root_mut.remove_first();
            assert_eq!(first_child_data, None);
        }

        let root_node = unsafe { tree.get_node_unchecked(&root_id) };
        assert_eq!(root_node.first_child, None);
        assert_eq!(root_node.last_child, None);
    }

    #[test]
    fn remove_first_single_child_present() {
        let mut tree = TreeBuilder::new().with_root(1).build();
        let root_id = tree.root_id().cloned().unwrap();

        {
            let mut root_mut = tree.get_mut(&root_id).ok().unwrap();
            root_mut.append(2);
            let first_child_data = root_mut.remove_first();
            assert_eq!(first_child_data, Some(2));
        }

        let root_node = unsafe { tree.get_node_unchecked(&root_id) };
        assert_eq!(root_node.first_child, None);
        assert_eq!(root_node.last_child, None);
    }

    #[test]
    fn remove_first_two_children_present() {
        let mut tree = TreeBuilder::new().with_root(1).build();
        let root_id = tree.root_id().cloned().unwrap();

        let node_id;
        {
            let mut root_mut = tree.get_mut(&root_id).ok().unwrap();
            root_mut.append(2);
            node_id = root_mut.append(3).node_id().clone();

            let first_child_data = root_mut.remove_first();
            assert_eq!(first_child_data, Some(2));
        }

        let root_node = unsafe { tree.get_node_unchecked(&root_id) };
        assert_eq!(root_node.first_child, Some(node_id.clone()));
        assert_eq!(root_node.last_child, Some(node_id.clone()));

        let node = unsafe { tree.get_node_unchecked(&node_id) };
        assert_eq!(node.parent, Some(root_id.clone()));
        assert_eq!(node.prev_sibling, None);
        assert_eq!(node.next_sibling, None);
        assert_eq!(node.first_child, None);
        assert_eq!(node.last_child, None);
    }

    #[test]
    fn remove_first_three_children_present() {
        let mut tree = TreeBuilder::new().with_root(1).build();
        let root_id = tree.root_id().cloned().unwrap();

        let node_id;
        let node_id_2;
        {
            let mut root_mut = tree.get_mut(&root_id).ok().unwrap();
            root_mut.append(2);
            node_id = root_mut.append(3).node_id().clone();
            node_id_2 = root_mut.append(4).node_id().clone();

            let first_child_data = root_mut.remove_first();
            assert_eq!(first_child_data, Some(2));
        }

        let root_node = unsafe { tree.get_node_unchecked(&root_id) };
        assert_eq!(root_node.first_child, Some(node_id.clone()));
        assert_eq!(root_node.last_child, Some(node_id_2.clone()));

        let node = unsafe { tree.get_node_unchecked(&node_id) };
        assert_eq!(node.parent, Some(root_id.clone()));
        assert_eq!(node.prev_sibling, None);
        assert_eq!(node.next_sibling, Some(node_id_2.clone()));
        assert_eq!(node.first_child, None);
        assert_eq!(node.last_child, None);

        let node_2 = unsafe { tree.get_node_unchecked(&node_id_2) };
        assert_eq!(node_2.parent, Some(root_id.clone()));
        assert_eq!(node_2.prev_sibling, Some(node_id.clone()));
        assert_eq!(node_2.next_sibling, None);
        assert_eq!(node_2.first_child, None);
        assert_eq!(node_2.last_child, None);
    }

    #[test]
    fn remove_last_no_children_present() {
        let mut tree = TreeBuilder::new().with_root(1).build();
        let root_id = tree.root_id().cloned().unwrap();

        {
            let mut root_mut = tree.get_mut(&root_id).ok().unwrap();
            let last_child_data = root_mut.remove_last();
            assert_eq!(last_child_data, None);
        }

        let root_node = unsafe { tree.get_node_unchecked(&root_id) };
        assert_eq!(root_node.first_child, None);
        assert_eq!(root_node.last_child, None);
    }

    #[test]
    fn remove_last_single_child_present() {
        let mut tree = TreeBuilder::new().with_root(1).build();
        let root_id = tree.root_id().cloned().unwrap();

        {
            let mut root_mut = tree.get_mut(&root_id).ok().unwrap();
            root_mut.append(2);
            let last_child_data = root_mut.remove_last();
            assert_eq!(last_child_data, Some(2));
        }

        let root_node = unsafe { tree.get_node_unchecked(&root_id) };
        assert_eq!(root_node.first_child, None);
        assert_eq!(root_node.last_child, None);
    }

    #[test]
    fn remove_last_two_children_present() {
        let mut tree = TreeBuilder::new().with_root(1).build();
        let root_id = tree.root_id().cloned().unwrap();

        let node_id;
        {
            let mut root_mut = tree.get_mut(&root_id).ok().unwrap();
            node_id = root_mut.append(2).node_id().clone();
            root_mut.append(3);

            let last_child_data = root_mut.remove_last();
            assert_eq!(last_child_data, Some(3));
        }

        let root_node = unsafe { tree.get_node_unchecked(&root_id) };
        assert_eq!(root_node.first_child, Some(node_id.clone()));
        assert_eq!(root_node.last_child, Some(node_id.clone()));

        let node = unsafe { tree.get_node_unchecked(&node_id) };
        assert_eq!(node.parent, Some(root_id.clone()));
        assert_eq!(node.prev_sibling, None);
        assert_eq!(node.next_sibling, None);
        assert_eq!(node.first_child, None);
        assert_eq!(node.last_child, None);
    }

    #[test]
    fn remove_last_three_children_present() {
        let mut tree = TreeBuilder::new().with_root(1).build();
        let root_id = tree.root_id().cloned().unwrap();

        let node_id;
        let node_id_2;
        {
            let mut root_mut = tree.get_mut(&root_id).ok().unwrap();
            node_id = root_mut.append(2).node_id().clone();
            node_id_2 = root_mut.append(3).node_id().clone();
            root_mut.append(4);

            let last_child_data = root_mut.remove_last();
            assert_eq!(last_child_data, Some(4));
        }

        let root_node = unsafe { tree.get_node_unchecked(&root_id) };
        assert_eq!(root_node.first_child, Some(node_id.clone()));
        assert_eq!(root_node.last_child, Some(node_id_2.clone()));

        let node = unsafe { tree.get_node_unchecked(&node_id) };
        assert_eq!(node.parent, Some(root_id.clone()));
        assert_eq!(node.prev_sibling, None);
        assert_eq!(node.next_sibling, Some(node_id_2.clone()));
        assert_eq!(node.first_child, None);
        assert_eq!(node.last_child, None);

        let node_2 = unsafe { tree.get_node_unchecked(&node_id_2) };
        assert_eq!(node_2.parent, Some(root_id.clone()));
        assert_eq!(node_2.prev_sibling, Some(node_id.clone()));
        assert_eq!(node_2.next_sibling, None);
        assert_eq!(node_2.first_child, None);
        assert_eq!(node_2.last_child, None);
    }
}
