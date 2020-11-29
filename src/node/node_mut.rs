use crate::behaviors::RemoveBehavior;
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
    ///
    ///
    /// let mut tree = TreeBuilder::new().with_root(1).build();
    /// tree.set_root(0);
    /// tree
    ///     .root_mut()
    ///     .unwrap()
    ///     .append(2);
    /// assert_eq!(tree.root().unwrap().last_child().unwrap().data(), &2);
    /// let mut s = String::new();
    /// tree.write_formatted(&mut s).unwrap();
    /// assert_eq!(&s, "\
    /// 0
    /// ├── 1
    /// └── 2
    /// ");
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
    /// Children of the removed `Node` can either be dropped with `DropChildren` or orphaned with
    /// `OrphanChildren`.
    ///
    /// ```
    /// use slab_tree::tree::TreeBuilder;
    /// use slab_tree::behaviors::RemoveBehavior::*;
    ///
    /// let mut tree = TreeBuilder::new().with_root(1).build();
    /// let mut root = tree.root_mut().expect("root doesn't exist?");
    /// root.append(2);
    /// root.append(3);
    ///
    /// let two = root.remove_first(DropChildren);
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
    pub fn remove_first(&mut self, behavior: RemoveBehavior) -> Option<T> {
        // todo: can probably simplify this
        let relatives = self.tree.get_node_relatives(self.node_id);
        let first = relatives.first_child;
        let first_id = first?;
        self.tree.remove(first_id, behavior)
    }

    ///
    /// Remove the first child of this `Node` and return the data that child contained.
    /// Returns a `Some`-value if this `Node` has a child to remove; returns a `None`-value
    /// otherwise.
    ///
    /// Children of the removed `Node` can either be dropped with `DropChildren` or orphaned with
    /// `OrphanChildren`.
    ///
    /// ```
    /// use slab_tree::tree::TreeBuilder;
    /// use slab_tree::behaviors::RemoveBehavior::*;
    ///
    /// let mut tree = TreeBuilder::new().with_root(1).build();
    /// let mut root = tree.root_mut().expect("root doesn't exist?");
    /// root.append(2);
    /// root.append(3);
    ///
    /// let three = root.remove_last(DropChildren);
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
    pub fn remove_last(&mut self, behavior: RemoveBehavior) -> Option<T> {
        // todo: can probably simplify this
        let relatives = self.tree.get_node_relatives(self.node_id);
        let last = relatives.last_child;
        let last_id = last?;
        self.tree.remove(last_id, behavior)
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
    /// assert_eq!(
    ///     *tree.get(two_id).unwrap().parent().unwrap().first_child().unwrap()
    ///         .data(),
    ///     3);
    /// assert_eq!(
    ///     *tree.get(two_id).unwrap().parent().unwrap().last_child().unwrap()
    ///         .data(),
    ///     4);
    /// assert!(tree.get_mut(two_id).unwrap().swap_next_sibling());
    /// assert_eq!(
    ///   tree.root().unwrap().children().map(|child_ref| *child_ref.data())
    ///         .collect::<Vec<i32>>(),
    ///     vec![3, 4, 2]);
    /// assert_eq!(
    ///     *tree.get(two_id).unwrap().parent().unwrap().first_child().unwrap()
    ///         .data(),
    ///     3);
    /// assert_eq!(
    ///     *tree.get(two_id).unwrap().parent().unwrap().last_child().unwrap()
    ///         .data(),
    ///     2);
    /// assert!(!tree.get_mut(two_id).unwrap().swap_next_sibling());
    /// assert_eq!(
    ///     tree.root().unwrap().children().map(|child_ref| *child_ref.data())
    ///         .collect::<Vec<i32>>(),
    ///     vec![3, 4, 2]);
    /// assert_eq!(
    ///     *tree.get(two_id).unwrap().parent().unwrap().first_child().unwrap()
    ///         .data(),
    ///     3);
    /// assert_eq!(
    ///     *tree.get(two_id).unwrap().parent().unwrap().last_child().unwrap()
    ///         .data(),
    ///     2);
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
    /// assert_eq!(
    ///     *tree.get(four_id).unwrap().parent().unwrap().first_child().unwrap()
    ///         .data(),
    ///     2);
    /// assert_eq!(
    ///     *tree.get(four_id).unwrap().parent().unwrap().last_child().unwrap()
    ///         .data(),
    ///     3);
    /// assert!(tree.get_mut(four_id).unwrap().swap_prev_sibling());
    /// assert_eq!(
    ///     tree.root().unwrap().children().map(|child_ref| *child_ref.data())
    ///         .collect::<Vec<i32>>(),
    ///     vec![4, 2, 3]);
    /// assert_eq!(
    ///     *tree.get(four_id).unwrap().parent().unwrap().first_child().unwrap()
    ///         .data(),
    ///     4);
    /// assert_eq!(
    ///     *tree.get(four_id).unwrap().parent().unwrap().last_child().unwrap()
    ///         .data(),
    ///     3);
    /// assert!(!tree.get_mut(four_id).unwrap().swap_prev_sibling());
    /// assert_eq!(
    ///     tree.root().unwrap().children().map(|child_ref| *child_ref.data())
    ///         .collect::<Vec<i32>>(),
    ///     vec![4, 2, 3]);
    /// assert_eq!(
    ///     *tree.get(four_id).unwrap().parent().unwrap().first_child().unwrap()
    ///         .data(),
    ///     4);
    /// assert_eq!(
    ///     *tree.get(four_id).unwrap().parent().unwrap().last_child().unwrap()
    ///         .data(),
    ///     3);
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
    /// assert!(tree.get_mut(two_id).unwrap().make_last_sibling());
    /// assert_eq!(
    ///     tree.root().unwrap().children().map(|child_ref| *child_ref.data())
    ///         .collect::<Vec<i32>>(),
    ///     vec![3, 4, 2]);
    /// assert_eq!(
    ///     *tree.get(two_id).unwrap().parent().unwrap().first_child().unwrap()
    ///         .data(),
    ///     3);
    /// assert_eq!(
    ///     *tree.get(two_id).unwrap().parent().unwrap().last_child().unwrap()
    ///         .data(),
    ///     2);
    /// assert!(!tree.get_mut(two_id).unwrap().make_last_sibling());
    /// assert_eq!(
    ///     tree.root().unwrap().children().map(|child_ref| *child_ref.data())
    ///         .collect::<Vec<i32>>(),
    ///     vec![3, 4, 2]);
    /// assert_eq!(
    ///     *tree.get(two_id).unwrap().parent().unwrap().first_child().unwrap()
    ///         .data(),
    ///     3);
    /// assert_eq!(
    ///     *tree.get(two_id).unwrap().parent().unwrap().last_child().unwrap()
    ///         .data(),
    ///     2);
    /// ```
    pub fn make_last_sibling(&mut self) -> bool {
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
    /// assert!(tree.get_mut(four_id).unwrap().make_first_sibling());
    /// assert_eq!(
    ///     tree.root().unwrap().children().map(|child_ref| *child_ref.data())
    ///         .collect::<Vec<i32>>(),
    ///     vec![4, 2, 3]);
    /// assert_eq!(
    ///     *tree.get(four_id).unwrap().parent().unwrap().first_child().unwrap()
    ///         .data(),
    ///     4);
    /// assert_eq!(
    ///     *tree.get(four_id).unwrap().parent().unwrap().last_child().unwrap()
    ///         .data(),
    ///     3);
    /// assert!(!tree.get_mut(four_id).unwrap().make_first_sibling());
    /// assert_eq!(
    ///     tree.root().unwrap().children().map(|child_ref| *child_ref.data())
    ///         .collect::<Vec<i32>>(),
    ///     vec![4, 2, 3]);
    /// assert_eq!(
    ///     *tree.get(four_id).unwrap().parent().unwrap().first_child().unwrap()
    ///         .data(),
    ///     4);
    /// assert_eq!(
    ///     *tree.get(four_id).unwrap().parent().unwrap().last_child().unwrap()
    ///         .data(),
    ///     3);
    /// ```
    pub fn make_first_sibling(&mut self) -> bool {
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
    use crate::behaviors::RemoveBehavior::{DropChildren, OrphanChildren};
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
        let first_child_data = root_mut.remove_first(DropChildren);
        assert_eq!(first_child_data, None);

        let root_node = tree.get_node(root_id);
        assert!(root_node.is_some());

        let root_node = root_node.unwrap();
        assert_eq!(root_node.relatives.first_child, None);
        assert_eq!(root_node.relatives.last_child, None);
    }

    #[test]
    fn remove_first_drop_single_child_present() {
        let mut tree = Tree::new();
        tree.set_root(1);
        let root_id = tree.root_id().expect("root doesn't exist?");

        let mut root_mut = tree.get_mut(root_id).unwrap();
        let two_id = root_mut.append(2).node_id();

        let removed = root_mut.remove_first(DropChildren);
        assert_eq!(removed, Some(2));

        let root_node = tree.get_node(root_id);
        assert!(root_node.is_some());

        let root_node = root_node.unwrap();
        assert_eq!(root_node.relatives.first_child, None);
        assert_eq!(root_node.relatives.last_child, None);

        let two = tree.get_node(two_id);
        assert!(two.is_none());
    }

    #[test]
    fn remove_first_drop_two_children_present() {
        let mut tree = Tree::new();
        tree.set_root(1);
        let root_id = tree.root_id().expect("root doesn't exist?");

        let mut root_mut = tree.get_mut(root_id).unwrap();
        root_mut.append(2);
        let three_id = root_mut.append(3).node_id();

        let removed = root_mut.remove_first(DropChildren);
        assert_eq!(removed, Some(2));

        let root_node = tree.get_node(root_id);
        assert!(root_node.is_some());

        let root_node = root_node.unwrap();
        assert_eq!(root_node.relatives.first_child, Some(three_id));
        assert_eq!(root_node.relatives.last_child, Some(three_id));

        let three = tree.get_node(three_id);
        assert!(three.is_some());

        let three = three.unwrap();
        assert_eq!(three.relatives.parent, Some(root_id));
        assert_eq!(three.relatives.prev_sibling, None);
        assert_eq!(three.relatives.next_sibling, None);
        assert_eq!(three.relatives.first_child, None);
        assert_eq!(three.relatives.last_child, None);
    }

    #[test]
    fn remove_first_drop_three_children_present() {
        let mut tree = Tree::new();
        tree.set_root(1);
        let root_id = tree.root_id().expect("root doesn't exist?");

        let mut root_mut = tree.get_mut(root_id).unwrap();
        root_mut.append(2);
        let three_id = root_mut.append(3).node_id();
        let four_id = root_mut.append(4).node_id();

        let removed = root_mut.remove_first(DropChildren);
        assert_eq!(removed, Some(2));

        let root_node = tree.get_node(root_id);
        assert!(root_node.is_some());

        let root_node = root_node.unwrap();
        assert_eq!(root_node.relatives.first_child, Some(three_id));
        assert_eq!(root_node.relatives.last_child, Some(four_id));

        let three = tree.get_node(three_id);
        assert!(three.is_some());

        let three = three.unwrap();
        assert_eq!(three.relatives.parent, Some(root_id));
        assert_eq!(three.relatives.prev_sibling, None);
        assert_eq!(three.relatives.next_sibling, Some(four_id));
        assert_eq!(three.relatives.first_child, None);
        assert_eq!(three.relatives.last_child, None);

        let four = tree.get_node(four_id);
        assert!(four.is_some());

        let four = four.unwrap();
        assert_eq!(four.relatives.parent, Some(root_id));
        assert_eq!(four.relatives.prev_sibling, Some(three_id));
        assert_eq!(four.relatives.next_sibling, None);
        assert_eq!(four.relatives.first_child, None);
        assert_eq!(four.relatives.last_child, None);
    }

    #[test]
    fn remove_first_drop_grandchild_present() {
        let mut tree = Tree::new();
        tree.set_root(1);
        let root_id = tree.root_id().expect("root doesn't exist?");

        let mut root_mut = tree.get_mut(root_id).unwrap();
        let three_id = root_mut.append(2).append(3).node_id();

        let removed = root_mut.remove_first(DropChildren);
        assert_eq!(removed, Some(2));

        let root_node = tree.get_node(root_id);
        assert!(root_node.is_some());

        let root_node = root_node.unwrap();
        assert_eq!(root_node.relatives.first_child, None);
        assert_eq!(root_node.relatives.last_child, None);

        let three = tree.get_node(three_id);
        assert!(three.is_none());
    }

    #[test]
    fn remove_first_orphan_grandchild_present() {
        let mut tree = Tree::new();
        tree.set_root(1);
        let root_id = tree.root_id().expect("root doesn't exist?");

        let mut root_mut = tree.get_mut(root_id).unwrap();
        let three_id = root_mut.append(2).append(3).node_id();

        let removed = root_mut.remove_first(OrphanChildren);
        assert_eq!(removed, Some(2));

        let root_node = tree.get_node(root_id);
        assert!(root_node.is_some());

        let root_node = root_node.unwrap();
        assert_eq!(root_node.relatives.first_child, None);
        assert_eq!(root_node.relatives.last_child, None);

        let three = tree.get_node(three_id);
        assert!(three.is_some());

        let three = three.unwrap();
        assert_eq!(three.relatives.parent, None);
    }

    #[test]
    fn remove_last_no_children_present() {
        let mut tree = Tree::new();
        tree.set_root(1);
        let root_id = tree.root_id().expect("root doesn't exist?");

        let mut root_mut = tree.get_mut(root_id).unwrap();
        let removed = root_mut.remove_last(DropChildren);
        assert_eq!(removed, None);

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
        let removed = root_mut.remove_last(DropChildren);
        assert_eq!(removed, Some(2));

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
        let two_id = root_mut.append(2).node_id();
        root_mut.append(3);

        let removed = root_mut.remove_last(DropChildren);
        assert_eq!(removed, Some(3));

        let root_node = tree.get_node(root_id);
        assert!(root_node.is_some());

        let root_node = root_node.unwrap();
        assert_eq!(root_node.relatives.first_child, Some(two_id));
        assert_eq!(root_node.relatives.last_child, Some(two_id));

        let two = tree.get_node(two_id);
        assert!(two.is_some());

        let two = two.unwrap();
        assert_eq!(two.relatives.parent, Some(root_id));
        assert_eq!(two.relatives.prev_sibling, None);
        assert_eq!(two.relatives.next_sibling, None);
        assert_eq!(two.relatives.first_child, None);
        assert_eq!(two.relatives.last_child, None);
    }

    #[test]
    fn remove_last_three_children_present() {
        let mut tree = Tree::new();
        tree.set_root(1);
        let root_id = tree.root_id().expect("root doesn't exist?");

        let mut root_mut = tree.get_mut(root_id).unwrap();
        let two_id = root_mut.append(2).node_id();
        let three_id = root_mut.append(3).node_id();
        root_mut.append(4);

        let removed = root_mut.remove_last(DropChildren);
        assert_eq!(removed, Some(4));

        let root_node = tree.get_node(root_id);
        assert!(root_node.is_some());

        let root_node = root_node.unwrap();
        assert_eq!(root_node.relatives.first_child, Some(two_id));
        assert_eq!(root_node.relatives.last_child, Some(three_id));

        let two = tree.get_node(two_id);
        assert!(two.is_some());

        let two = two.unwrap();
        assert_eq!(two.relatives.parent, Some(root_id));
        assert_eq!(two.relatives.prev_sibling, None);
        assert_eq!(two.relatives.next_sibling, Some(three_id));
        assert_eq!(two.relatives.first_child, None);
        assert_eq!(two.relatives.last_child, None);

        let three = tree.get_node(three_id);
        assert!(three.is_some());

        let three = three.unwrap();
        assert_eq!(three.relatives.parent, Some(root_id));
        assert_eq!(three.relatives.prev_sibling, Some(two_id));
        assert_eq!(three.relatives.next_sibling, None);
        assert_eq!(three.relatives.first_child, None);
        assert_eq!(three.relatives.last_child, None);
    }

    #[test]
    fn remove_last_orphan_grandchild_present() {
        let mut tree = Tree::new();
        tree.set_root(1);
        let root_id = tree.root_id().expect("root doesn't exist?");

        let mut root_mut = tree.get_mut(root_id).unwrap();
        let three_id = root_mut.append(2).append(3).node_id();

        let removed = root_mut.remove_last(OrphanChildren);
        assert_eq!(removed, Some(2));

        let root_node = tree.get_node(root_id);
        assert!(root_node.is_some());

        let root_node = root_node.unwrap();
        assert_eq!(root_node.relatives.first_child, None);
        assert_eq!(root_node.relatives.last_child, None);

        let three = tree.get_node(three_id);
        assert!(three.is_some());

        let three = three.unwrap();
        assert_eq!(three.relatives.parent, None);
    }
}
