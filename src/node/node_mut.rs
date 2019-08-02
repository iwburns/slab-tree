use crate::node::Node;
use crate::node::NodeRef;
use crate::tree::Tree;
use crate::NodeId;

///
/// A mutable reference to a given `Node`'s data and its relatives.
///
#[derive(Debug, PartialEq)]
pub struct NodeMut<'a, T> {
    node_id: NodeId,
    tree: &'a mut Tree<T>,
}

impl<'a, T> NodeMut<'a, T> {
    pub(crate) fn new(node_id: NodeId, tree: &mut Tree<T>) -> NodeMut<T> {
        NodeMut { node_id, tree }
    }

    ///
    /// Returns the `NodeId` that identifies this `Node` in the tree.
    ///
    /// ```
    /// use slab_tree::tree::TreeBuilder;
    ///
    /// let mut tree = TreeBuilder::new().with_root(1).build();
    /// let root_id = tree.root_id().expect("root doesn't exist?");
    /// let root = tree.root_mut().expect("root doesn't exist?");
    ///
    /// let root_id_again = root.node_id();
    ///
    /// assert_eq!(root_id_again, root_id);
    /// ```
    ///
    pub fn node_id(&self) -> NodeId {
        self.node_id
    }

    ///
    /// Returns a mutable reference to the data contained by the given `Node`.
    ///
    /// ```
    /// use slab_tree::tree::TreeBuilder;
    ///
    /// let mut tree = TreeBuilder::new().with_root(1).build();
    /// let mut root = tree.root_mut().expect("root doesn't exist?");
    ///
    /// let data = root.data();
    ///
    /// assert_eq!(data, &mut 1);
    ///
    /// *data = 3;
    ///
    /// assert_eq!(data, &mut 3);
    /// ```
    ///
    pub fn data(&mut self) -> &mut T {
        if let Some(node) = self.tree.get_node_mut(self.node_id) {
            &mut node.data
        } else {
            unreachable!()
        }
    }

    ///
    /// Returns a `NodeMut` pointing to this `Node`'s parent.  Returns a `Some`-value containing
    /// the `NodeMut` if this `Node` has a parent; otherwise returns a `None`.
    ///
    /// ```
    /// use slab_tree::tree::TreeBuilder;
    ///
    /// let mut tree = TreeBuilder::new().with_root(1).build();
    /// let mut root = tree.root_mut().expect("root doesn't exist?");
    ///
    /// assert!(root.parent().is_none());
    /// ```
    ///
    pub fn parent(&mut self) -> Option<NodeMut<T>> {
        self.get_self_as_node()
            .relatives
            .parent
            .map(move |id| NodeMut::new(id, self.tree))
    }

    ///
    /// Returns a `NodeMut` pointing to this `Node`'s previous sibling.  Returns a `Some`-value
    /// containing the `NodeMut` if this `Node` has a previous sibling; otherwise returns a `None`.
    ///
    /// ```
    /// use slab_tree::tree::TreeBuilder;
    ///
    /// let mut tree = TreeBuilder::new().with_root(1).build();
    /// let mut root = tree.root_mut().expect("root doesn't exist?");
    ///
    /// assert!(root.prev_sibling().is_none());
    /// ```
    ///
    pub fn prev_sibling(&mut self) -> Option<NodeMut<T>> {
        self.get_self_as_node()
            .relatives
            .prev_sibling
            .map(move |id| NodeMut::new(id, self.tree))
    }

    ///
    /// Returns a `NodeMut` pointing to this `Node`'s next sibling.  Returns a `Some`-value
    /// containing the `NodeMut` if this `Node` has a next sibling; otherwise returns a `None`.
    ///
    /// ```
    /// use slab_tree::tree::TreeBuilder;
    ///
    /// let mut tree = TreeBuilder::new().with_root(1).build();
    /// let mut root = tree.root_mut().expect("root doesn't exist?");
    ///
    /// assert!(root.next_sibling().is_none());
    /// ```
    ///
    pub fn next_sibling(&mut self) -> Option<NodeMut<T>> {
        self.get_self_as_node()
            .relatives
            .next_sibling
            .map(move |id| NodeMut::new(id, self.tree))
    }

    ///
    /// Returns a `NodeMut` pointing to this `Node`'s first child.  Returns a `Some`-value
    /// containing the `NodeMut` if this `Node` has a first child; otherwise returns a `None`.
    ///
    /// ```
    /// use slab_tree::tree::TreeBuilder;
    ///
    /// let mut tree = TreeBuilder::new().with_root(1).build();
    /// let mut root = tree.root_mut().expect("root doesn't exist?");
    ///
    /// assert!(root.first_child().is_none());
    /// ```
    ///
    pub fn first_child(&mut self) -> Option<NodeMut<T>> {
        self.get_self_as_node()
            .relatives
            .first_child
            .map(move |id| NodeMut::new(id, self.tree))
    }

    ///
    /// Returns a `NodeMut` pointing to this `Node`'s last child.  Returns a `Some`-value
    /// containing the `NodeMut` if this `Node` has a last child; otherwise returns a `None`.
    ///
    /// ```
    /// use slab_tree::tree::TreeBuilder;
    ///
    /// let mut tree = TreeBuilder::new().with_root(1).build();
    /// let mut root = tree.root_mut().expect("root doesn't exist?");
    ///
    /// assert!(root.last_child().is_none());
    /// ```
    ///
    pub fn last_child(&mut self) -> Option<NodeMut<T>> {
        self.get_self_as_node()
            .relatives
            .last_child
            .map(move |id| NodeMut::new(id, self.tree))
    }

    ///
    /// Appends a new `Node` as this `Node`'s last child (and first child if it has none).
    /// Returns a `NodeMut` pointing to the newly added `Node`.
    ///
    /// ```
    /// use slab_tree::tree::TreeBuilder;
    ///
    /// let mut tree = TreeBuilder::new().with_root(1).build();
    /// let mut root = tree.root_mut().expect("root doesn't exist?");
    ///
    /// root.append(2);
    ///
    /// assert!(root.first_child().is_some());
    /// assert_eq!(root.first_child().unwrap().data(), &mut 2);
    ///
    /// assert!(root.last_child().is_some());
    /// assert_eq!(root.last_child().unwrap().data(), &mut 2);
    ///
    /// let mut child = root.first_child().unwrap();
    ///
    /// assert!(child.parent().is_some());
    /// assert_eq!(child.parent().unwrap().data(), &mut 1);
    /// ```
    ///
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

        NodeMut::new(new_id, self.tree)
    }

    ///
    /// Prepends a new `Node` as this `Node`'s first child (and last child if it has none).
    /// Returns a `NodeMut` pointing to the newly added `Node`.
    ///
    /// ```
    /// use slab_tree::tree::TreeBuilder;
    ///
    /// let mut tree = TreeBuilder::new().with_root(1).build();
    /// let mut root = tree.root_mut().expect("root doesn't exist?");
    ///
    /// root.prepend(2);
    ///
    /// assert!(root.first_child().is_some());
    /// assert_eq!(root.first_child().unwrap().data(), &mut 2);
    ///
    /// assert!(root.last_child().is_some());
    /// assert_eq!(root.last_child().unwrap().data(), &mut 2);
    ///
    /// let mut child = root.first_child().unwrap();
    ///
    /// assert!(child.parent().is_some());
    /// assert_eq!(child.parent().unwrap().data(), &mut 1);
    /// ```
    ///
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

        NodeMut::new(new_id, self.tree)
    }

    ///
    /// Remove the first child of this `Node` and return the data that child contained.
    /// Returns a `Some`-value if this `Node` has a child to remove; returns a `None`-value
    /// otherwise.
    ///
    /// ```
    /// use slab_tree::tree::TreeBuilder;
    ///
    /// let mut tree = TreeBuilder::new().with_root(1).build();
    /// let mut root = tree.root_mut().expect("root doesn't exist?");
    /// root.append(2);
    /// root.append(3);
    ///
    /// let two = root.remove_first();
    ///
    /// assert!(two.is_some());
    /// assert_eq!(two.unwrap(), 2);
    ///
    /// assert!(root.first_child().is_some());
    /// assert_eq!(root.first_child().unwrap().data(), &mut 3);
    ///
    /// assert!(root.last_child().is_some());
    /// assert_eq!(root.last_child().unwrap().data(), &mut 3);
    /// ```
    ///
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

        self.tree.core_tree.remove(first_id)
    }

    ///
    /// Remove the first child of this `Node` and return the data that child contained.
    /// Returns a `Some`-value if this `Node` has a child to remove; returns a `None`-value
    /// otherwise.
    ///
    /// ```
    /// use slab_tree::tree::TreeBuilder;
    ///
    /// let mut tree = TreeBuilder::new().with_root(1).build();
    /// let mut root = tree.root_mut().expect("root doesn't exist?");
    /// root.append(2);
    /// root.append(3);
    ///
    /// let three = root.remove_last();
    ///
    /// assert!(three.is_some());
    /// assert_eq!(three.unwrap(), 3);
    ///
    /// assert!(root.first_child().is_some());
    /// assert_eq!(root.first_child().unwrap().data(), &mut 2);
    ///
    /// assert!(root.last_child().is_some());
    /// assert_eq!(root.last_child().unwrap().data(), &mut 2);
    /// ```
    ///
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

        self.tree.core_tree.remove(last_id)
    }

    ///
    /// Returns a `NodeRef` pointing to this `NodeMut`.
    ///
    /// ```
    /// use slab_tree::tree::TreeBuilder;
    ///
    /// let mut tree = TreeBuilder::new().with_root(1).build();
    /// let mut root = tree.root_mut().expect("root doesn't exist?");
    /// root.append(2);
    ///
    /// let root = root.as_ref();
    ///
    /// assert_eq!(root.data(), &1);
    /// ```
    ///
    pub fn as_ref(&self) -> NodeRef<T> {
        NodeRef::new(self.node_id, self.tree)
    }

    /// Exchange positions with the next sibling.
    ///
    /// Returns true if swapped with a next sibling, returns false if this was
    /// already the last sibling.
    ///
    /// ```
    /// use slab_tree::tree::TreeBuilder;
    ///
    /// let mut tree = TreeBuilder::new().with_root(1).build();
    /// let two_id = {
    ///     let mut root = tree.root_mut().expect("root doesn't exist?");
    ///     let two_id = root.append(2).node_id();
    ///     root.append(3);
    ///     root.append(4);
    ///     two_id
    /// };
    /// assert_eq!(
    ///     tree.root().unwrap().children().map(|child_ref| *child_ref.data())
    ///         .collect::<Vec<i32>>(),
    ///     vec![2, 3, 4]);
    /// assert!(tree.get_mut(two_id).unwrap().swap_next_sibling());
    /// assert_eq!(
    ///     tree.root().unwrap().children().map(|child_ref| *child_ref.data())
    ///         .collect::<Vec<i32>>(),
    ///     vec![3, 2, 4]);
    /// assert!(tree.get_mut(two_id).unwrap().swap_next_sibling());
    /// assert_eq!(
    ///   tree.root().unwrap().children().map(|child_ref| *child_ref.data())
    ///         .collect::<Vec<i32>>(),
    ///     vec![3, 4, 2]);
    /// assert!(!tree.get_mut(two_id).unwrap().swap_next_sibling());
    /// assert_eq!(
    ///     tree.root().unwrap().children().map(|child_ref| *child_ref.data())
    ///         .collect::<Vec<i32>>(),
    ///     vec![3, 4, 2]);
    /// ```
    pub fn swap_next_sibling(&mut self) -> bool {
        let node_id = self.node_id();
        let prev_id = self.tree.get_node_prev_sibling_id(node_id);
        let next_id = self.tree.get_node_next_sibling_id(node_id);
        if let Some(next_id) = next_id {
            if let Some(parent_id) = self.parent().map(|parent| parent.node_id()) {
                let (set_first, set_last) = {
                    let parent = self.tree.get(parent_id).unwrap();
                    (
                        node_id == parent.first_child().unwrap().node_id(),
                        next_id == parent.last_child().unwrap().node_id(),
                    )
                };
                if set_first {
                    self.tree.set_first_child(parent_id, Some(next_id));
                }
                if set_last {
                    self.tree.set_last_child(parent_id, Some(node_id));
                }
            }
            let new_next_id = self.tree.get_node_next_sibling_id(next_id);
            self.tree
                .set_prev_siblings_next_sibling(node_id, Some(next_id));
            self.tree.set_next_siblings_prev_sibling(node_id, prev_id);
            self.tree.set_prev_sibling(node_id, Some(next_id));
            self.tree.set_next_sibling(node_id, new_next_id);
            self.tree
                .set_prev_siblings_next_sibling(node_id, Some(node_id));
            self.tree
                .set_next_siblings_prev_sibling(node_id, Some(node_id));
            true
        } else {
            false
        }
    }

    /// Exchange positions with the previous sibling.
    ///
    /// Returns true if swapped with a previous sibling, returns false if this
    /// was already the first sibling.
    ///
    /// ```
    /// use slab_tree::tree::TreeBuilder;
    ///
    /// let mut tree = TreeBuilder::new().with_root(1).build();
    /// let four_id = {
    ///     let mut root = tree.root_mut().expect("root doesn't exist?");
    ///     root.append(2);
    ///     root.append(3);
    ///     let four_id = root.append(4).node_id();
    ///     four_id
    /// };
    /// assert_eq!(
    ///     tree.root().unwrap().children().map(|child_ref| *child_ref.data())
    ///         .collect::<Vec<i32>>(),
    ///     vec![2, 3, 4]);
    /// assert!(tree.get_mut(four_id).unwrap().swap_prev_sibling());
    /// assert_eq!(
    ///     tree.root().unwrap().children().map(|child_ref| *child_ref.data())
    ///         .collect::<Vec<i32>>(),
    ///     vec![2, 4, 3]);
    /// assert!(tree.get_mut(four_id).unwrap().swap_prev_sibling());
    /// assert_eq!(
    ///     tree.root().unwrap().children().map(|child_ref| *child_ref.data())
    ///         .collect::<Vec<i32>>(),
    ///     vec![4, 2, 3]);
    /// assert!(!tree.get_mut(four_id).unwrap().swap_prev_sibling());
    /// assert_eq!(
    ///     tree.root().unwrap().children().map(|child_ref| *child_ref.data())
    ///         .collect::<Vec<i32>>(),
    ///     vec![4, 2, 3]);
    /// ```
    pub fn swap_prev_sibling(&mut self) -> bool {
        let node_id = self.node_id();
        let prev_id = self.tree.get_node_prev_sibling_id(node_id);
        let next_id = self.tree.get_node_next_sibling_id(node_id);
        if let Some(prev_id) = prev_id {
            if let Some(parent_id) = self.parent().map(|parent| parent.node_id()) {
                let (set_first, set_last) = {
                    let parent = self.tree.get(parent_id).unwrap();
                    (
                        prev_id == parent.first_child().unwrap().node_id(),
                        node_id == parent.last_child().unwrap().node_id(),
                    )
                };
                if set_first {
                    self.tree.set_first_child(parent_id, Some(node_id));
                }
                if set_last {
                    self.tree.set_last_child(parent_id, Some(prev_id));
                }
            }
            let new_prev_id = self.tree.get_node_prev_sibling_id(prev_id);
            self.tree.set_prev_siblings_next_sibling(node_id, next_id);
            self.tree
                .set_next_siblings_prev_sibling(node_id, Some(prev_id));
            self.tree.set_prev_sibling(node_id, new_prev_id);
            self.tree.set_next_sibling(node_id, Some(prev_id));
            self.tree
                .set_prev_siblings_next_sibling(node_id, Some(node_id));
            self.tree
                .set_next_siblings_prev_sibling(node_id, Some(node_id));
            true
        } else {
            false
        }
    }

    /// Moves this node to the last sibling position.
    ///
    /// Returns false if the node was already the last sibling.
    ///
    /// ```
    /// use slab_tree::tree::TreeBuilder;
    ///
    /// let mut tree = TreeBuilder::new().with_root(1).build();
    /// let two_id = {
    ///     let mut root = tree.root_mut().expect("root doesn't exist?");
    ///     let two_id = root.append(2).node_id();
    ///     root.append(3);
    ///     root.append(4);
    ///     two_id
    /// };
    /// assert_eq!(
    ///     tree.root().unwrap().children().map(|child_ref| *child_ref.data())
    ///         .collect::<Vec<i32>>(),
    ///     vec![2, 3, 4]);
    /// assert!(tree.get_mut(two_id).unwrap().to_last_sibling());
    /// assert_eq!(
    ///     tree.root().unwrap().children().map(|child_ref| *child_ref.data())
    ///         .collect::<Vec<i32>>(),
    ///     vec![3, 4, 2]);
    /// assert!(!tree.get_mut(two_id).unwrap().to_last_sibling());
    /// assert_eq!(
    ///     tree.root().unwrap().children().map(|child_ref| *child_ref.data())
    ///         .collect::<Vec<i32>>(),
    ///     vec![3, 4, 2]);
    /// ```
    pub fn to_last_sibling(&mut self) -> bool {
        if let Some(parent_id) = self.parent().map(|parent| parent.node_id()) {
            let node_id = self.node_id();
            let prev_id = self.tree.get_node_prev_sibling_id(node_id);
            let next_id = self.tree.get_node_next_sibling_id(node_id);
            let last_id = self
                .tree
                .get(parent_id)
                .unwrap()
                .last_child()
                .unwrap()
                .node_id();
            let first_id = self
                .tree
                .get(parent_id)
                .unwrap()
                .first_child()
                .unwrap()
                .node_id();
            if node_id != last_id {
                self.tree.set_last_child(parent_id, Some(node_id));
                if node_id == first_id {
                    self.tree.set_first_child(parent_id, next_id);
                }
                self.tree.set_next_sibling(last_id, Some(node_id));
                self.tree.set_prev_siblings_next_sibling(node_id, next_id);
                self.tree.set_next_siblings_prev_sibling(node_id, prev_id);
                self.tree.set_prev_sibling(node_id, Some(last_id));
                self.tree.set_next_sibling(node_id, None);
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    /// Moves this node to the first sibling position.
    ///
    /// Returns false if the node was already the first sibling.
    ///
    /// ```
    /// use slab_tree::tree::TreeBuilder;
    ///
    /// let mut tree = TreeBuilder::new().with_root(1).build();
    /// let four_id = {
    ///     let mut root = tree.root_mut().expect("root doesn't exist?");
    ///     root.append(2);
    ///     root.append(3);
    ///     root.append(4).node_id()
    /// };
    /// assert_eq!(
    ///     tree.root().unwrap().children().map(|child_ref| *child_ref.data())
    ///         .collect::<Vec<i32>>(),
    ///     vec![2, 3, 4]);
    /// assert!(tree.get_mut(four_id).unwrap().to_first_sibling());
    /// assert_eq!(
    ///     tree.root().unwrap().children().map(|child_ref| *child_ref.data())
    ///         .collect::<Vec<i32>>(),
    ///     vec![4, 2, 3]);
    /// assert!(!tree.get_mut(four_id).unwrap().to_first_sibling());
    /// assert_eq!(
    ///     tree.root().unwrap().children().map(|child_ref| *child_ref.data())
    ///         .collect::<Vec<i32>>(),
    ///     vec![4, 2, 3]);
    /// ```
    pub fn to_first_sibling(&mut self) -> bool {
        if let Some(parent_id) = self.parent().map(|parent| parent.node_id()) {
            let node_id = self.node_id();
            let prev_id = self.tree.get_node_prev_sibling_id(node_id);
            let next_id = self.tree.get_node_next_sibling_id(node_id);
            let first_id = self
                .tree
                .get(parent_id)
                .unwrap()
                .first_child()
                .unwrap()
                .node_id();
            let last_id = self
                .tree
                .get(parent_id)
                .unwrap()
                .last_child()
                .unwrap()
                .node_id();
            if node_id != first_id {
                self.tree.set_first_child(parent_id, Some(node_id));
                if node_id == last_id {
                    self.tree.set_last_child(parent_id, prev_id);
                }
                self.tree.set_prev_sibling(first_id, Some(node_id));
                self.tree.set_prev_siblings_next_sibling(node_id, next_id);
                self.tree.set_next_siblings_prev_sibling(node_id, prev_id);
                self.tree.set_next_sibling(node_id, Some(first_id));
                self.tree.set_prev_sibling(node_id, None);
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    fn get_self_as_node(&self) -> &Node<T> {
        if let Some(node) = self.tree.get_node(self.node_id) {
            &node
        } else {
            unreachable!()
        }
    }
}

#[cfg_attr(tarpaulin, skip)]
#[cfg(test)]
mod node_mut_tests {
    use crate::tree::Tree;

    #[test]
    fn node_id() {
        let mut tree = Tree::new();
        tree.set_root(1);
        let root_id = tree.root_id().expect("root doesn't exist?");

        let root_mut = tree.get_mut(root_id).unwrap();
        assert_eq!(root_id, root_mut.node_id());
    }

    #[test]
    fn data() {
        let mut tree = Tree::new();
        tree.set_root(1);
        let root_id = tree.root_id().expect("root doesn't exist?");

        let mut root_mut = tree.get_mut(root_id).unwrap();
        assert_eq!(root_mut.data(), &mut 1);

        *root_mut.data() = 2;
        assert_eq!(root_mut.data(), &mut 2);
    }

    #[test]
    fn parent() {
        let mut tree = Tree::new();
        tree.set_root(1);
        let root_id = tree.root_id().expect("root doesn't exist?");
        let mut root_mut = tree.get_mut(root_id).unwrap();
        assert!(root_mut.parent().is_none());
    }

    #[test]
    fn prev_sibling() {
        let mut tree = Tree::new();
        tree.set_root(1);
        let root_id = tree.root_id().expect("root doesn't exist?");
        let mut root_mut = tree.get_mut(root_id).unwrap();
        assert!(root_mut.prev_sibling().is_none());
    }

    #[test]
    fn next_sibling() {
        let mut tree = Tree::new();
        tree.set_root(1);
        let root_id = tree.root_id().expect("root doesn't exist?");
        let mut root_mut = tree.get_mut(root_id).unwrap();
        assert!(root_mut.next_sibling().is_none());
    }

    #[test]
    fn first_child() {
        let mut tree = Tree::new();
        tree.set_root(1);
        let root_id = tree.root_id().expect("root doesn't exist?");
        let mut root_mut = tree.get_mut(root_id).unwrap();
        assert!(root_mut.first_child().is_none());
    }

    #[test]
    fn last_child() {
        let mut tree = Tree::new();
        tree.set_root(1);
        let root_id = tree.root_id().expect("root doesn't exist?");
        let mut root_mut = tree.get_mut(root_id).unwrap();
        assert!(root_mut.last_child().is_none());
    }

    #[test]
    fn append_no_children_present() {
        let mut tree = Tree::new();
        tree.set_root(1);
        let root_id = tree.root_id().expect("root doesn't exist?");

        let mut root_mut = tree.get_mut(root_id).unwrap();
        let new_id = root_mut.append(2).node_id();

        let root_node = tree.get_node(root_id);
        assert!(root_node.is_some());

        let root_node = root_node.unwrap();
        assert_eq!(root_node.relatives.first_child, Some(new_id));
        assert_eq!(root_node.relatives.last_child, Some(new_id));

        let new_node = tree.get_node(new_id);
        assert!(new_node.is_some());

        let new_node = new_node.unwrap();
        assert_eq!(new_node.relatives.parent, Some(root_id));
        assert_eq!(new_node.relatives.prev_sibling, None);
        assert_eq!(new_node.relatives.next_sibling, None);
        assert_eq!(new_node.relatives.first_child, None);
        assert_eq!(new_node.relatives.last_child, None);

        let root = tree.get(root_id).unwrap();
        assert_eq!(root.data(), &1);

        let new_node = root.first_child().unwrap();
        assert_eq!(new_node.data(), &2);
    }

    #[test]
    fn append_single_child_present() {
        let mut tree = Tree::new();
        tree.set_root(1);
        let root_id = tree.root_id().expect("root doesn't exist?");

        let mut root_mut = tree.get_mut(root_id).unwrap();
        let new_id = root_mut.append(2).node_id();
        let new_id_2 = root_mut.append(3).node_id();

        let root_node = tree.get_node(root_id);
        assert!(root_node.is_some());

        let root_node = root_node.unwrap();
        assert_eq!(root_node.relatives.first_child, Some(new_id));
        assert_eq!(root_node.relatives.last_child, Some(new_id_2));

        let new_node = tree.get_node(new_id);
        assert!(new_node.is_some());

        let new_node = new_node.unwrap();
        assert_eq!(new_node.relatives.parent, Some(root_id));
        assert_eq!(new_node.relatives.prev_sibling, None);
        assert_eq!(new_node.relatives.next_sibling, Some(new_id_2));
        assert_eq!(new_node.relatives.first_child, None);
        assert_eq!(new_node.relatives.last_child, None);

        let new_node_2 = tree.get_node(new_id_2);
        assert!(new_node_2.is_some());

        let new_node_2 = new_node_2.unwrap();
        assert_eq!(new_node_2.relatives.parent, Some(root_id));
        assert_eq!(new_node_2.relatives.prev_sibling, Some(new_id));
        assert_eq!(new_node_2.relatives.next_sibling, None);
        assert_eq!(new_node_2.relatives.first_child, None);
        assert_eq!(new_node_2.relatives.last_child, None);

        let root = tree.get(root_id).unwrap();
        assert_eq!(root.data(), &1);

        let new_node = root.first_child().unwrap();
        assert_eq!(new_node.data(), &2);

        let new_node_2 = root.last_child().unwrap();
        assert_eq!(new_node_2.data(), &3);
    }

    #[test]
    fn append_two_children_present() {
        let mut tree = Tree::new();
        tree.set_root(1);
        let root_id = tree.root_id().expect("root doesn't exist?");

        let mut root_mut = tree.get_mut(root_id).unwrap();
        let new_id = root_mut.append(2).node_id();
        let new_id_2 = root_mut.append(3).node_id();
        let new_id_3 = root_mut.append(4).node_id();

        let root_node = tree.get_node(root_id);
        assert!(root_node.is_some());

        let root_node = root_node.unwrap();
        assert_eq!(root_node.relatives.first_child, Some(new_id));
        assert_eq!(root_node.relatives.last_child, Some(new_id_3));

        let new_node = tree.get_node(new_id);
        assert!(new_node.is_some());

        let new_node = new_node.unwrap();
        assert_eq!(new_node.relatives.parent, Some(root_id));
        assert_eq!(new_node.relatives.prev_sibling, None);
        assert_eq!(new_node.relatives.next_sibling, Some(new_id_2));
        assert_eq!(new_node.relatives.first_child, None);
        assert_eq!(new_node.relatives.last_child, None);

        let new_node_2 = tree.get_node(new_id_2);
        assert!(new_node_2.is_some());

        let new_node_2 = new_node_2.unwrap();
        assert_eq!(new_node_2.relatives.parent, Some(root_id));
        assert_eq!(new_node_2.relatives.prev_sibling, Some(new_id));
        assert_eq!(new_node_2.relatives.next_sibling, Some(new_id_3));
        assert_eq!(new_node_2.relatives.first_child, None);
        assert_eq!(new_node_2.relatives.last_child, None);

        let new_node_3 = tree.get_node(new_id_3);
        assert!(new_node_3.is_some());

        let new_node_3 = new_node_3.unwrap();
        assert_eq!(new_node_3.relatives.parent, Some(root_id));
        assert_eq!(new_node_3.relatives.prev_sibling, Some(new_id_2));
        assert_eq!(new_node_3.relatives.next_sibling, None);
        assert_eq!(new_node_3.relatives.first_child, None);
        assert_eq!(new_node_3.relatives.last_child, None);

        let root = tree.get(root_id).unwrap();
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
        let mut tree = Tree::new();
        tree.set_root(1);
        let root_id = tree.root_id().expect("root doesn't exist?");

        let mut root_mut = tree.get_mut(root_id).unwrap();
        let new_id = root_mut.prepend(2).node_id();

        let root_node = tree.get_node(root_id);
        assert!(root_node.is_some());

        let root_node = root_node.unwrap();
        assert_eq!(root_node.relatives.first_child, Some(new_id));
        assert_eq!(root_node.relatives.last_child, Some(new_id));

        let new_node = tree.get_node(new_id);
        assert!(new_node.is_some());

        let new_node = new_node.unwrap();
        assert_eq!(new_node.relatives.parent, Some(root_id));
        assert_eq!(new_node.relatives.prev_sibling, None);
        assert_eq!(new_node.relatives.next_sibling, None);
        assert_eq!(new_node.relatives.first_child, None);
        assert_eq!(new_node.relatives.last_child, None);

        let root = tree.get(root_id).unwrap();
        assert_eq!(root.data(), &1);

        let new_node = root.first_child().unwrap();
        assert_eq!(new_node.data(), &2);
    }

    #[test]
    fn prepend_single_child_present() {
        let mut tree = Tree::new();
        tree.set_root(1);
        let root_id = tree.root_id().expect("root doesn't exist?");

        let mut root_mut = tree.get_mut(root_id).unwrap();
        let new_id = root_mut.prepend(2).node_id();
        let new_id_2 = root_mut.prepend(3).node_id();

        let root_node = tree.get_node(root_id);
        assert!(root_node.is_some());

        let root_node = root_node.unwrap();
        assert_eq!(root_node.relatives.first_child, Some(new_id_2));
        assert_eq!(root_node.relatives.last_child, Some(new_id));

        let new_node = tree.get_node(new_id);
        assert!(new_node.is_some());

        let new_node = new_node.unwrap();
        assert_eq!(new_node.relatives.parent, Some(root_id));
        assert_eq!(new_node.relatives.prev_sibling, Some(new_id_2));
        assert_eq!(new_node.relatives.next_sibling, None);
        assert_eq!(new_node.relatives.first_child, None);
        assert_eq!(new_node.relatives.last_child, None);

        let new_node_2 = tree.get_node(new_id_2);
        assert!(new_node_2.is_some());

        let new_node_2 = new_node_2.unwrap();
        assert_eq!(new_node_2.relatives.parent, Some(root_id));
        assert_eq!(new_node_2.relatives.prev_sibling, None);
        assert_eq!(new_node_2.relatives.next_sibling, Some(new_id));
        assert_eq!(new_node_2.relatives.first_child, None);
        assert_eq!(new_node_2.relatives.last_child, None);

        let root = tree.get(root_id).unwrap();
        assert_eq!(root.data(), &1);

        let new_node = root.first_child().unwrap();
        assert_eq!(new_node.data(), &3);

        let new_node_2 = root.last_child().unwrap();
        assert_eq!(new_node_2.data(), &2);
    }

    #[test]
    fn prepend_two_children_present() {
        let mut tree = Tree::new();
        tree.set_root(1);
        let root_id = tree.root_id().expect("root doesn't exist?");

        let mut root_mut = tree.get_mut(root_id).unwrap();
        let new_id = root_mut.prepend(2).node_id();
        let new_id_2 = root_mut.prepend(3).node_id();
        let new_id_3 = root_mut.prepend(4).node_id();

        let root_node = tree.get_node(root_id);
        assert!(root_node.is_some());

        let root_node = root_node.unwrap();
        assert_eq!(root_node.relatives.first_child, Some(new_id_3));
        assert_eq!(root_node.relatives.last_child, Some(new_id));

        let new_node = tree.get_node(new_id);
        assert!(new_node.is_some());

        let new_node = new_node.unwrap();
        assert_eq!(new_node.relatives.parent, Some(root_id));
        assert_eq!(new_node.relatives.prev_sibling, Some(new_id_2));
        assert_eq!(new_node.relatives.next_sibling, None);
        assert_eq!(new_node.relatives.first_child, None);
        assert_eq!(new_node.relatives.last_child, None);

        let new_node_2 = tree.get_node(new_id_2);
        assert!(new_node_2.is_some());

        let new_node_2 = new_node_2.unwrap();
        assert_eq!(new_node_2.relatives.parent, Some(root_id));
        assert_eq!(new_node_2.relatives.prev_sibling, Some(new_id_3));
        assert_eq!(new_node_2.relatives.next_sibling, Some(new_id));
        assert_eq!(new_node_2.relatives.first_child, None);
        assert_eq!(new_node_2.relatives.last_child, None);

        let new_node_3 = tree.get_node(new_id_3);
        assert!(new_node_3.is_some());

        let new_node_3 = new_node_3.unwrap();
        assert_eq!(new_node_3.relatives.parent, Some(root_id));
        assert_eq!(new_node_3.relatives.prev_sibling, None);
        assert_eq!(new_node_3.relatives.next_sibling, Some(new_id_2));
        assert_eq!(new_node_3.relatives.first_child, None);
        assert_eq!(new_node_3.relatives.last_child, None);

        let root = tree.get(root_id).unwrap();
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
        let mut tree = Tree::new();
        tree.set_root(1);
        let root_id = tree.root_id().expect("root doesn't exist?");

        let mut root_mut = tree.get_mut(root_id).unwrap();
        let first_child_data = root_mut.remove_first();
        assert_eq!(first_child_data, None);

        let root_node = tree.get_node(root_id);
        assert!(root_node.is_some());

        let root_node = root_node.unwrap();
        assert_eq!(root_node.relatives.first_child, None);
        assert_eq!(root_node.relatives.last_child, None);
    }

    #[test]
    fn remove_first_single_child_present() {
        let mut tree = Tree::new();
        tree.set_root(1);
        let root_id = tree.root_id().expect("root doesn't exist?");

        let mut root_mut = tree.get_mut(root_id).unwrap();
        root_mut.append(2);
        let first_child_data = root_mut.remove_first();
        assert_eq!(first_child_data, Some(2));

        let root_node = tree.get_node(root_id);
        assert!(root_node.is_some());

        let root_node = root_node.unwrap();
        assert_eq!(root_node.relatives.first_child, None);
        assert_eq!(root_node.relatives.last_child, None);
    }

    #[test]
    fn remove_first_two_children_present() {
        let mut tree = Tree::new();
        tree.set_root(1);
        let root_id = tree.root_id().expect("root doesn't exist?");

        let mut root_mut = tree.get_mut(root_id).unwrap();
        root_mut.append(2);
        let node_id = root_mut.append(3).node_id();

        let first_child_data = root_mut.remove_first();
        assert_eq!(first_child_data, Some(2));

        let root_node = tree.get_node(root_id);
        assert!(root_node.is_some());

        let root_node = root_node.unwrap();
        assert_eq!(root_node.relatives.first_child, Some(node_id));
        assert_eq!(root_node.relatives.last_child, Some(node_id));

        let node = tree.get_node(node_id);
        assert!(node.is_some());

        let node = node.unwrap();
        assert_eq!(node.relatives.parent, Some(root_id));
        assert_eq!(node.relatives.prev_sibling, None);
        assert_eq!(node.relatives.next_sibling, None);
        assert_eq!(node.relatives.first_child, None);
        assert_eq!(node.relatives.last_child, None);
    }

    #[test]
    fn remove_first_three_children_present() {
        let mut tree = Tree::new();
        tree.set_root(1);
        let root_id = tree.root_id().expect("root doesn't exist?");

        let mut root_mut = tree.get_mut(root_id).unwrap();
        root_mut.append(2);
        let node_id = root_mut.append(3).node_id();
        let node_id_2 = root_mut.append(4).node_id();

        let first_child_data = root_mut.remove_first();
        assert_eq!(first_child_data, Some(2));

        let root_node = tree.get_node(root_id);
        assert!(root_node.is_some());

        let root_node = root_node.unwrap();
        assert_eq!(root_node.relatives.first_child, Some(node_id));
        assert_eq!(root_node.relatives.last_child, Some(node_id_2));

        let node = tree.get_node(node_id);
        assert!(node.is_some());

        let node = node.unwrap();
        assert_eq!(node.relatives.parent, Some(root_id));
        assert_eq!(node.relatives.prev_sibling, None);
        assert_eq!(node.relatives.next_sibling, Some(node_id_2));
        assert_eq!(node.relatives.first_child, None);
        assert_eq!(node.relatives.last_child, None);

        let node_2 = tree.get_node(node_id_2);
        assert!(node_2.is_some());

        let node_2 = node_2.unwrap();
        assert_eq!(node_2.relatives.parent, Some(root_id));
        assert_eq!(node_2.relatives.prev_sibling, Some(node_id));
        assert_eq!(node_2.relatives.next_sibling, None);
        assert_eq!(node_2.relatives.first_child, None);
        assert_eq!(node_2.relatives.last_child, None);
    }

    #[test]
    fn remove_last_no_children_present() {
        let mut tree = Tree::new();
        tree.set_root(1);
        let root_id = tree.root_id().expect("root doesn't exist?");

        let mut root_mut = tree.get_mut(root_id).unwrap();
        let last_child_data = root_mut.remove_last();
        assert_eq!(last_child_data, None);

        let root_node = tree.get_node(root_id);
        assert!(root_node.is_some());

        let root_node = root_node.unwrap();
        assert_eq!(root_node.relatives.first_child, None);
        assert_eq!(root_node.relatives.last_child, None);
    }

    #[test]
    fn remove_last_single_child_present() {
        let mut tree = Tree::new();
        tree.set_root(1);
        let root_id = tree.root_id().expect("root doesn't exist?");

        let mut root_mut = tree.get_mut(root_id).unwrap();
        root_mut.append(2);
        let last_child_data = root_mut.remove_last();
        assert_eq!(last_child_data, Some(2));

        let root_node = tree.get_node(root_id);
        assert!(root_node.is_some());

        let root_node = root_node.unwrap();
        assert_eq!(root_node.relatives.first_child, None);
        assert_eq!(root_node.relatives.last_child, None);
    }

    #[test]
    fn remove_last_two_children_present() {
        let mut tree = Tree::new();
        tree.set_root(1);
        let root_id = tree.root_id().expect("root doesn't exist?");

        let mut root_mut = tree.get_mut(root_id).unwrap();
        let node_id = root_mut.append(2).node_id();
        root_mut.append(3);

        let last_child_data = root_mut.remove_last();
        assert_eq!(last_child_data, Some(3));

        let root_node = tree.get_node(root_id);
        assert!(root_node.is_some());

        let root_node = root_node.unwrap();
        assert_eq!(root_node.relatives.first_child, Some(node_id));
        assert_eq!(root_node.relatives.last_child, Some(node_id));

        let node = tree.get_node(node_id);
        assert!(node.is_some());

        let node = node.unwrap();
        assert_eq!(node.relatives.parent, Some(root_id));
        assert_eq!(node.relatives.prev_sibling, None);
        assert_eq!(node.relatives.next_sibling, None);
        assert_eq!(node.relatives.first_child, None);
        assert_eq!(node.relatives.last_child, None);
    }

    #[test]
    fn remove_last_three_children_present() {
        let mut tree = Tree::new();
        tree.set_root(1);
        let root_id = tree.root_id().expect("root doesn't exist?");

        let mut root_mut = tree.get_mut(root_id).unwrap();
        let node_id = root_mut.append(2).node_id();
        let node_id_2 = root_mut.append(3).node_id();
        root_mut.append(4);

        let last_child_data = root_mut.remove_last();
        assert_eq!(last_child_data, Some(4));

        let root_node = tree.get_node(root_id);
        assert!(root_node.is_some());

        let root_node = root_node.unwrap();
        assert_eq!(root_node.relatives.first_child, Some(node_id));
        assert_eq!(root_node.relatives.last_child, Some(node_id_2));

        let node = tree.get_node(node_id);
        assert!(node.is_some());

        let node = node.unwrap();
        assert_eq!(node.relatives.parent, Some(root_id));
        assert_eq!(node.relatives.prev_sibling, None);
        assert_eq!(node.relatives.next_sibling, Some(node_id_2));
        assert_eq!(node.relatives.first_child, None);
        assert_eq!(node.relatives.last_child, None);

        let node_2 = tree.get_node(node_id_2);
        assert!(node_2.is_some());

        let node_2 = node_2.unwrap();
        assert_eq!(node_2.relatives.parent, Some(root_id));
        assert_eq!(node_2.relatives.prev_sibling, Some(node_id));
        assert_eq!(node_2.relatives.next_sibling, None);
        assert_eq!(node_2.relatives.first_child, None);
        assert_eq!(node_2.relatives.last_child, None);
    }
}
