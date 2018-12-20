use crate::node::Node;
use crate::tree::Tree;
use crate::NodeId;

///
/// A mutable reference to a given `Node`'s data and its relatives.
///
#[derive(Debug, PartialEq)]
pub struct NodeMut<'a, T: 'a> {
    pub(crate) node_id: NodeId,
    pub(crate) tree: &'a mut Tree<T>,
}

impl<'a, T: 'a> NodeMut<'a, T> {
    pub fn node_id(&self) -> NodeId {
        self.node_id
    }

    pub fn data(&mut self) -> &mut T {
        if let Ok(node) = self.tree.get_node_mut(self.node_id) {
            &mut node.data
        } else {
            unreachable!()
        }
    }

    pub fn parent(&mut self) -> Option<NodeMut<T>> {
        self.get_self_as_node()
            .relatives
            .parent
            .map(move |id| self.tree.new_node_mut(id))
    }

    pub fn prev_sibling(&mut self) -> Option<NodeMut<T>> {
        self.get_self_as_node()
            .relatives
            .prev_sibling
            .map(move |id| self.tree.new_node_mut(id))
    }

    pub fn next_sibling(&mut self) -> Option<NodeMut<T>> {
        self.get_self_as_node()
            .relatives
            .next_sibling
            .map(move |id| self.tree.new_node_mut(id))
    }

    pub fn first_child(&mut self) -> Option<NodeMut<T>> {
        self.get_self_as_node()
            .relatives
            .first_child
            .map(move |id| self.tree.new_node_mut(id))
    }

    pub fn last_child(&mut self) -> Option<NodeMut<T>> {
        self.get_self_as_node()
            .relatives
            .last_child
            .map(move |id| self.tree.new_node_mut(id))
    }

    pub fn append(&mut self, data: T) -> NodeMut<T> {
        let new_id = self.tree.core_tree.insert(data);

        let relatives = self.tree.get_node_relatives(self.node_id);

        let prev_sibling = relatives.last_child;
        self.tree.set_parent(new_id, Some(self.node_id));
        self.tree.set_prev_sibling(new_id, prev_sibling);

        let first_child = relatives.first_child.or_else(|| Some(new_id));
        self.tree.set_first_child(self.node_id, first_child);
        self.tree.set_last_child(self.node_id, Some(new_id));

        if let Some(node_id) = prev_sibling {
            self.tree.set_next_sibling(node_id, Some(new_id));
        }

        self.tree.new_node_mut(new_id)
    }

    pub fn prepend(&mut self, data: T) -> NodeMut<T> {
        let new_id = self.tree.core_tree.insert(data);

        let relatives = self.tree.get_node_relatives(self.node_id);

        let next_sibling = relatives.first_child;
        self.tree.set_parent(new_id, Some(self.node_id));
        self.tree.set_next_sibling(new_id, next_sibling);

        let last_child = relatives.last_child.or_else(|| Some(new_id));
        self.tree.set_first_child(self.node_id, Some(new_id));
        self.tree.set_last_child(self.node_id, last_child);

        if let Some(node_id) = next_sibling {
            self.tree.set_prev_sibling(node_id, Some(new_id));
        }

        self.tree.new_node_mut(new_id)
    }

    pub fn remove_first(&mut self) -> Option<T> {
        let relatives = self.tree.get_node_relatives(self.node_id);

        let first = relatives.first_child;
        let last = relatives.last_child;

        let first_id = first?;
        if first == last {
            self.tree.set_first_child(self.node_id, None);
            self.tree.set_last_child(self.node_id, None);
        } else {
            let first_child = self.tree.get_node_relatives(first_id).next_sibling;
            self.tree.set_first_child(self.node_id, first_child);
            self.tree.set_next_siblings_prev_sibling(first_id, None);
        }

        Some(self.tree.core_tree.remove(first_id))
    }

    pub fn remove_last(&mut self) -> Option<T> {
        let relatives = self.tree.get_node_relatives(self.node_id);

        let first = relatives.first_child;
        let last = relatives.last_child;

        let last_id = last?;
        if first == last {
            self.tree.set_first_child(self.node_id, None);
            self.tree.set_last_child(self.node_id, None);
        } else {
            let last_child = self.tree.get_node_relatives(last_id).prev_sibling;
            self.tree.set_last_child(self.node_id, last_child);
            self.tree.set_prev_siblings_next_sibling(last_id, None);
        }

        Some(self.tree.core_tree.remove(last_id))
    }

    fn get_self_as_node(&self) -> &Node<T> {
        if let Ok(node) = self.tree.get_node(self.node_id) {
            &node
        } else {
            unreachable!()
        }
    }
}

#[cfg_attr(tarpaulin, skip)]
#[cfg(test)]
mod node_mut_tests {
    use tree::Tree;

    #[test]
    fn node_id() {
        let mut tree = Tree::new(1);
        let root_id = tree.root_id();

        let root_mut = tree.get_mut(root_id).ok().unwrap();
        assert_eq!(root_id, root_mut.node_id());
    }

    #[test]
    fn data() {
        let mut tree = Tree::new(1);
        let root_id = tree.root_id();

        let mut root_mut = tree.get_mut(root_id).ok().unwrap();
        assert_eq!(root_mut.data(), &mut 1);

        *root_mut.data() = 2;
        assert_eq!(root_mut.data(), &mut 2);
    }

    #[test]
    fn parent() {
        let mut tree = Tree::new(1);
        let root_id = tree.root_id();
        let mut root_mut = tree.get_mut(root_id).ok().unwrap();
        assert!(root_mut.parent().is_none());
    }

    #[test]
    fn prev_sibling() {
        let mut tree = Tree::new(1);
        let root_id = tree.root_id();
        let mut root_mut = tree.get_mut(root_id).ok().unwrap();
        assert!(root_mut.prev_sibling().is_none());
    }

    #[test]
    fn next_sibling() {
        let mut tree = Tree::new(1);
        let root_id = tree.root_id();
        let mut root_mut = tree.get_mut(root_id).ok().unwrap();
        assert!(root_mut.next_sibling().is_none());
    }

    #[test]
    fn first_child() {
        let mut tree = Tree::new(1);
        let root_id = tree.root_id();
        let mut root_mut = tree.get_mut(root_id).ok().unwrap();
        assert!(root_mut.first_child().is_none());
    }

    #[test]
    fn last_child() {
        let mut tree = Tree::new(1);
        let root_id = tree.root_id();
        let mut root_mut = tree.get_mut(root_id).ok().unwrap();
        assert!(root_mut.last_child().is_none());
    }

    #[test]
    fn append_no_children_present() {
        let mut tree = Tree::new(1);
        let root_id = tree.root_id();

        let new_id;
        {
            let mut root_mut = tree.get_mut(root_id).ok().unwrap();
            new_id = root_mut.append(2).node_id();
        }

        let root_node = tree.get_node(root_id);
        assert!(root_node.is_ok());

        let root_node = root_node.ok().unwrap();
        assert_eq!(root_node.relatives.first_child, Some(new_id));
        assert_eq!(root_node.relatives.last_child, Some(new_id));

        let new_node = tree.get_node(new_id);
        assert!(new_node.is_ok());

        let new_node = new_node.ok().unwrap();
        assert_eq!(new_node.relatives.parent, Some(root_id));
        assert_eq!(new_node.relatives.prev_sibling, None);
        assert_eq!(new_node.relatives.next_sibling, None);
        assert_eq!(new_node.relatives.first_child, None);
        assert_eq!(new_node.relatives.last_child, None);

        let root = tree.get(root_id).ok().unwrap();
        assert_eq!(root.data(), &1);

        let new_node = root.first_child().unwrap();
        assert_eq!(new_node.data(), &2);
    }

    #[test]
    fn append_single_child_present() {
        let mut tree = Tree::new(1);
        let root_id = tree.root_id();

        let new_id;
        let new_id_2;
        {
            let mut root_mut = tree.get_mut(root_id).ok().unwrap();
            new_id = root_mut.append(2).node_id();
            new_id_2 = root_mut.append(3).node_id();
        }

        let root_node = tree.get_node(root_id);
        assert!(root_node.is_ok());

        let root_node = root_node.ok().unwrap();
        assert_eq!(root_node.relatives.first_child, Some(new_id));
        assert_eq!(root_node.relatives.last_child, Some(new_id_2));

        let new_node = tree.get_node(new_id);
        assert!(new_node.is_ok());

        let new_node = new_node.ok().unwrap();
        assert_eq!(new_node.relatives.parent, Some(root_id));
        assert_eq!(new_node.relatives.prev_sibling, None);
        assert_eq!(new_node.relatives.next_sibling, Some(new_id_2));
        assert_eq!(new_node.relatives.first_child, None);
        assert_eq!(new_node.relatives.last_child, None);

        let new_node_2 = tree.get_node(new_id_2);
        assert!(new_node_2.is_ok());

        let new_node_2 = new_node_2.ok().unwrap();
        assert_eq!(new_node_2.relatives.parent, Some(root_id));
        assert_eq!(new_node_2.relatives.prev_sibling, Some(new_id));
        assert_eq!(new_node_2.relatives.next_sibling, None);
        assert_eq!(new_node_2.relatives.first_child, None);
        assert_eq!(new_node_2.relatives.last_child, None);

        let root = tree.get(root_id).ok().unwrap();
        assert_eq!(root.data(), &1);

        let new_node = root.first_child().unwrap();
        assert_eq!(new_node.data(), &2);

        let new_node_2 = root.last_child().unwrap();
        assert_eq!(new_node_2.data(), &3);
    }

    #[test]
    fn append_two_children_present() {
        let mut tree = Tree::new(1);
        let root_id = tree.root_id();

        let new_id;
        let new_id_2;
        let new_id_3;
        {
            let mut root_mut = tree.get_mut(root_id).ok().unwrap();
            new_id = root_mut.append(2).node_id();
            new_id_2 = root_mut.append(3).node_id();
            new_id_3 = root_mut.append(4).node_id();
        }

        let root_node = tree.get_node(root_id);
        assert!(root_node.is_ok());

        let root_node = root_node.ok().unwrap();
        assert_eq!(root_node.relatives.first_child, Some(new_id));
        assert_eq!(root_node.relatives.last_child, Some(new_id_3));

        let new_node = tree.get_node(new_id);
        assert!(new_node.is_ok());

        let new_node = new_node.ok().unwrap();
        assert_eq!(new_node.relatives.parent, Some(root_id));
        assert_eq!(new_node.relatives.prev_sibling, None);
        assert_eq!(new_node.relatives.next_sibling, Some(new_id_2));
        assert_eq!(new_node.relatives.first_child, None);
        assert_eq!(new_node.relatives.last_child, None);

        let new_node_2 = tree.get_node(new_id_2);
        assert!(new_node_2.is_ok());

        let new_node_2 = new_node_2.ok().unwrap();
        assert_eq!(new_node_2.relatives.parent, Some(root_id));
        assert_eq!(new_node_2.relatives.prev_sibling, Some(new_id));
        assert_eq!(new_node_2.relatives.next_sibling, Some(new_id_3));
        assert_eq!(new_node_2.relatives.first_child, None);
        assert_eq!(new_node_2.relatives.last_child, None);

        let new_node_3 = tree.get_node(new_id_3);
        assert!(new_node_3.is_ok());

        let new_node_3 = new_node_3.ok().unwrap();
        assert_eq!(new_node_3.relatives.parent, Some(root_id));
        assert_eq!(new_node_3.relatives.prev_sibling, Some(new_id_2));
        assert_eq!(new_node_3.relatives.next_sibling, None);
        assert_eq!(new_node_3.relatives.first_child, None);
        assert_eq!(new_node_3.relatives.last_child, None);

        let root = tree.get(root_id).ok().unwrap();
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
        let mut tree = Tree::new(1);
        let root_id = tree.root_id();

        let new_id;
        {
            let mut root_mut = tree.get_mut(root_id).ok().unwrap();
            new_id = root_mut.prepend(2).node_id();
        }

        let root_node = tree.get_node(root_id);
        assert!(root_node.is_ok());

        let root_node = root_node.ok().unwrap();
        assert_eq!(root_node.relatives.first_child, Some(new_id));
        assert_eq!(root_node.relatives.last_child, Some(new_id));

        let new_node = tree.get_node(new_id);
        assert!(new_node.is_ok());

        let new_node = new_node.ok().unwrap();
        assert_eq!(new_node.relatives.parent, Some(root_id));
        assert_eq!(new_node.relatives.prev_sibling, None);
        assert_eq!(new_node.relatives.next_sibling, None);
        assert_eq!(new_node.relatives.first_child, None);
        assert_eq!(new_node.relatives.last_child, None);

        let root = tree.get(root_id).ok().unwrap();
        assert_eq!(root.data(), &1);

        let new_node = root.first_child().unwrap();
        assert_eq!(new_node.data(), &2);
    }

    #[test]
    fn prepend_single_child_present() {
        let mut tree = Tree::new(1);
        let root_id = tree.root_id();

        let new_id;
        let new_id_2;
        {
            let mut root_mut = tree.get_mut(root_id).ok().unwrap();
            new_id = root_mut.prepend(2).node_id();
            new_id_2 = root_mut.prepend(3).node_id();
        }

        let root_node = tree.get_node(root_id);
        assert!(root_node.is_ok());

        let root_node = root_node.ok().unwrap();
        assert_eq!(root_node.relatives.first_child, Some(new_id_2));
        assert_eq!(root_node.relatives.last_child, Some(new_id));

        let new_node = tree.get_node(new_id);
        assert!(new_node.is_ok());

        let new_node = new_node.ok().unwrap();
        assert_eq!(new_node.relatives.parent, Some(root_id));
        assert_eq!(new_node.relatives.prev_sibling, Some(new_id_2));
        assert_eq!(new_node.relatives.next_sibling, None);
        assert_eq!(new_node.relatives.first_child, None);
        assert_eq!(new_node.relatives.last_child, None);

        let new_node_2 = tree.get_node(new_id_2);
        assert!(new_node_2.is_ok());

        let new_node_2 = new_node_2.ok().unwrap();
        assert_eq!(new_node_2.relatives.parent, Some(root_id));
        assert_eq!(new_node_2.relatives.prev_sibling, None);
        assert_eq!(new_node_2.relatives.next_sibling, Some(new_id));
        assert_eq!(new_node_2.relatives.first_child, None);
        assert_eq!(new_node_2.relatives.last_child, None);

        let root = tree.get(root_id).ok().unwrap();
        assert_eq!(root.data(), &1);

        let new_node = root.first_child().unwrap();
        assert_eq!(new_node.data(), &3);

        let new_node_2 = root.last_child().unwrap();
        assert_eq!(new_node_2.data(), &2);
    }

    #[test]
    fn prepend_two_children_present() {
        let mut tree = Tree::new(1);
        let root_id = tree.root_id();

        let new_id;
        let new_id_2;
        let new_id_3;
        {
            let mut root_mut = tree.get_mut(root_id).ok().unwrap();
            new_id = root_mut.prepend(2).node_id();
            new_id_2 = root_mut.prepend(3).node_id();
            new_id_3 = root_mut.prepend(4).node_id();
        }

        let root_node = tree.get_node(root_id);
        assert!(root_node.is_ok());

        let root_node = root_node.ok().unwrap();
        assert_eq!(root_node.relatives.first_child, Some(new_id_3));
        assert_eq!(root_node.relatives.last_child, Some(new_id));

        let new_node = tree.get_node(new_id);
        assert!(new_node.is_ok());

        let new_node = new_node.ok().unwrap();
        assert_eq!(new_node.relatives.parent, Some(root_id));
        assert_eq!(new_node.relatives.prev_sibling, Some(new_id_2));
        assert_eq!(new_node.relatives.next_sibling, None);
        assert_eq!(new_node.relatives.first_child, None);
        assert_eq!(new_node.relatives.last_child, None);

        let new_node_2 = tree.get_node(new_id_2);
        assert!(new_node_2.is_ok());

        let new_node_2 = new_node_2.ok().unwrap();
        assert_eq!(new_node_2.relatives.parent, Some(root_id));
        assert_eq!(new_node_2.relatives.prev_sibling, Some(new_id_3));
        assert_eq!(new_node_2.relatives.next_sibling, Some(new_id));
        assert_eq!(new_node_2.relatives.first_child, None);
        assert_eq!(new_node_2.relatives.last_child, None);

        let new_node_3 = tree.get_node(new_id_3);
        assert!(new_node_3.is_ok());

        let new_node_3 = new_node_3.ok().unwrap();
        assert_eq!(new_node_3.relatives.parent, Some(root_id));
        assert_eq!(new_node_3.relatives.prev_sibling, None);
        assert_eq!(new_node_3.relatives.next_sibling, Some(new_id_2));
        assert_eq!(new_node_3.relatives.first_child, None);
        assert_eq!(new_node_3.relatives.last_child, None);

        let root = tree.get(root_id).ok().unwrap();
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
        let mut tree = Tree::new(1);
        let root_id = tree.root_id();

        {
            let mut root_mut = tree.get_mut(root_id).ok().unwrap();
            let first_child_data = root_mut.remove_first();
            assert_eq!(first_child_data, None);
        }

        let root_node = tree.get_node(root_id);
        assert!(root_node.is_ok());

        let root_node = root_node.ok().unwrap();
        assert_eq!(root_node.relatives.first_child, None);
        assert_eq!(root_node.relatives.last_child, None);
    }

    #[test]
    fn remove_first_single_child_present() {
        let mut tree = Tree::new(1);
        let root_id = tree.root_id();

        {
            let mut root_mut = tree.get_mut(root_id).ok().unwrap();
            root_mut.append(2);
            let first_child_data = root_mut.remove_first();
            assert_eq!(first_child_data, Some(2));
        }

        let root_node = tree.get_node(root_id);
        assert!(root_node.is_ok());

        let root_node = root_node.ok().unwrap();
        assert_eq!(root_node.relatives.first_child, None);
        assert_eq!(root_node.relatives.last_child, None);
    }

    #[test]
    fn remove_first_two_children_present() {
        let mut tree = Tree::new(1);
        let root_id = tree.root_id();

        let node_id;
        {
            let mut root_mut = tree.get_mut(root_id).ok().unwrap();
            root_mut.append(2);
            node_id = root_mut.append(3).node_id();

            let first_child_data = root_mut.remove_first();
            assert_eq!(first_child_data, Some(2));
        }

        let root_node = tree.get_node(root_id);
        assert!(root_node.is_ok());

        let root_node = root_node.ok().unwrap();
        assert_eq!(root_node.relatives.first_child, Some(node_id));
        assert_eq!(root_node.relatives.last_child, Some(node_id));

        let node = tree.get_node(node_id);
        assert!(node.is_ok());

        let node = node.ok().unwrap();
        assert_eq!(node.relatives.parent, Some(root_id));
        assert_eq!(node.relatives.prev_sibling, None);
        assert_eq!(node.relatives.next_sibling, None);
        assert_eq!(node.relatives.first_child, None);
        assert_eq!(node.relatives.last_child, None);
    }

    #[test]
    fn remove_first_three_children_present() {
        let mut tree = Tree::new(1);
        let root_id = tree.root_id();

        let node_id;
        let node_id_2;
        {
            let mut root_mut = tree.get_mut(root_id).ok().unwrap();
            root_mut.append(2);
            node_id = root_mut.append(3).node_id();
            node_id_2 = root_mut.append(4).node_id();

            let first_child_data = root_mut.remove_first();
            assert_eq!(first_child_data, Some(2));
        }

        let root_node = tree.get_node(root_id);
        assert!(root_node.is_ok());

        let root_node = root_node.ok().unwrap();
        assert_eq!(root_node.relatives.first_child, Some(node_id));
        assert_eq!(root_node.relatives.last_child, Some(node_id_2));

        let node = tree.get_node(node_id);
        assert!(node.is_ok());

        let node = node.ok().unwrap();
        assert_eq!(node.relatives.parent, Some(root_id));
        assert_eq!(node.relatives.prev_sibling, None);
        assert_eq!(node.relatives.next_sibling, Some(node_id_2));
        assert_eq!(node.relatives.first_child, None);
        assert_eq!(node.relatives.last_child, None);

        let node_2 = tree.get_node(node_id_2);
        assert!(node_2.is_ok());

        let node_2 = node_2.ok().unwrap();
        assert_eq!(node_2.relatives.parent, Some(root_id));
        assert_eq!(node_2.relatives.prev_sibling, Some(node_id));
        assert_eq!(node_2.relatives.next_sibling, None);
        assert_eq!(node_2.relatives.first_child, None);
        assert_eq!(node_2.relatives.last_child, None);
    }

    #[test]
    fn remove_last_no_children_present() {
        let mut tree = Tree::new(1);
        let root_id = tree.root_id();

        {
            let mut root_mut = tree.get_mut(root_id).ok().unwrap();
            let last_child_data = root_mut.remove_last();
            assert_eq!(last_child_data, None);
        }

        let root_node = tree.get_node(root_id);
        assert!(root_node.is_ok());

        let root_node = root_node.ok().unwrap();
        assert_eq!(root_node.relatives.first_child, None);
        assert_eq!(root_node.relatives.last_child, None);
    }

    #[test]
    fn remove_last_single_child_present() {
        let mut tree = Tree::new(1);
        let root_id = tree.root_id();

        {
            let mut root_mut = tree.get_mut(root_id).ok().unwrap();
            root_mut.append(2);
            let last_child_data = root_mut.remove_last();
            assert_eq!(last_child_data, Some(2));
        }

        let root_node = tree.get_node(root_id);
        assert!(root_node.is_ok());

        let root_node = root_node.ok().unwrap();
        assert_eq!(root_node.relatives.first_child, None);
        assert_eq!(root_node.relatives.last_child, None);
    }

    #[test]
    fn remove_last_two_children_present() {
        let mut tree = Tree::new(1);
        let root_id = tree.root_id();

        let node_id;
        {
            let mut root_mut = tree.get_mut(root_id).ok().unwrap();
            node_id = root_mut.append(2).node_id();
            root_mut.append(3);

            let last_child_data = root_mut.remove_last();
            assert_eq!(last_child_data, Some(3));
        }

        let root_node = tree.get_node(root_id);
        assert!(root_node.is_ok());

        let root_node = root_node.ok().unwrap();
        assert_eq!(root_node.relatives.first_child, Some(node_id));
        assert_eq!(root_node.relatives.last_child, Some(node_id));

        let node = tree.get_node(node_id);
        assert!(node.is_ok());

        let node = node.ok().unwrap();
        assert_eq!(node.relatives.parent, Some(root_id));
        assert_eq!(node.relatives.prev_sibling, None);
        assert_eq!(node.relatives.next_sibling, None);
        assert_eq!(node.relatives.first_child, None);
        assert_eq!(node.relatives.last_child, None);
    }

    #[test]
    fn remove_last_three_children_present() {
        let mut tree = Tree::new(1);
        let root_id = tree.root_id();

        let node_id;
        let node_id_2;
        {
            let mut root_mut = tree.get_mut(root_id).ok().unwrap();
            node_id = root_mut.append(2).node_id();
            node_id_2 = root_mut.append(3).node_id();
            root_mut.append(4);

            let last_child_data = root_mut.remove_last();
            assert_eq!(last_child_data, Some(4));
        }

        let root_node = tree.get_node(root_id);
        assert!(root_node.is_ok());

        let root_node = root_node.ok().unwrap();
        assert_eq!(root_node.relatives.first_child, Some(node_id));
        assert_eq!(root_node.relatives.last_child, Some(node_id_2));

        let node = tree.get_node(node_id);
        assert!(node.is_ok());

        let node = node.ok().unwrap();
        assert_eq!(node.relatives.parent, Some(root_id));
        assert_eq!(node.relatives.prev_sibling, None);
        assert_eq!(node.relatives.next_sibling, Some(node_id_2));
        assert_eq!(node.relatives.first_child, None);
        assert_eq!(node.relatives.last_child, None);

        let node_2 = tree.get_node(node_id_2);
        assert!(node_2.is_ok());

        let node_2 = node_2.ok().unwrap();
        assert_eq!(node_2.relatives.parent, Some(root_id));
        assert_eq!(node_2.relatives.prev_sibling, Some(node_id));
        assert_eq!(node_2.relatives.next_sibling, None);
        assert_eq!(node_2.relatives.first_child, None);
        assert_eq!(node_2.relatives.last_child, None);
    }
}
