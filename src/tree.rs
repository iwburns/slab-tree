use crate::behaviors::*;
use crate::core_tree::CoreTree;
use crate::node::*;
use crate::NodeId;

///
/// A `Tree` builder. Provides more control over how a `Tree` is created.
///
pub struct TreeBuilder<T> {
    root: Option<T>,
    capacity: Option<usize>,
}

impl<T> Default for TreeBuilder<T> {
    fn default() -> Self {
        TreeBuilder::new()
    }
}

impl<T> TreeBuilder<T> {
    ///
    /// Creates a new `TreeBuilder` with the default settings.
    ///
    /// ```
    /// use slab_tree::tree::TreeBuilder;
    ///
    /// let _tree_builder = TreeBuilder::new();
    ///
    /// # _tree_builder.with_root(1);
    /// ```
    ///
    pub fn new() -> TreeBuilder<T> {
        TreeBuilder {
            root: None,
            capacity: None,
        }
    }

    ///
    /// Sets the root `Node` of the `TreeBuilder`.
    ///
    /// ```
    /// use slab_tree::tree::TreeBuilder;
    ///
    /// let _tree_builder = TreeBuilder::new().with_root(1);
    /// ```
    ///
    pub fn with_root(self, root: T) -> TreeBuilder<T> {
        TreeBuilder {
            root: Some(root),
            capacity: self.capacity,
        }
    }

    ///
    /// Sets the capacity of the `TreeBuilder`.
    ///
    /// This can be used to pre-allocate space in the `Tree` if you know you'll be adding a large
    /// number of `Node`s to the `Tree`.
    ///
    /// ```
    /// use slab_tree::tree::TreeBuilder;
    ///
    /// let _tree_builder = TreeBuilder::new().with_capacity(10);
    ///
    /// # _tree_builder.with_root(1);
    /// ```
    ///
    pub fn with_capacity(self, capacity: usize) -> TreeBuilder<T> {
        TreeBuilder {
            root: self.root,
            capacity: Some(capacity),
        }
    }

    ///
    /// Build a `Tree` based upon the current settings in the `TreeBuilder`.
    ///
    /// ```
    /// use slab_tree::tree::TreeBuilder;
    ///
    /// let _tree = TreeBuilder::new().with_root(1).with_capacity(10).build();
    /// ```
    ///
    pub fn build(self) -> Tree<T> {
        let capacity = self.capacity.unwrap_or(0);
        let mut core_tree: CoreTree<T> = CoreTree::new(capacity);
        let root_id = self.root.map(|val| core_tree.insert(val));

        Tree { root_id, core_tree }
    }
}

///
/// A tree structure containing `Node`s.
///
#[derive(Debug, PartialEq)]
pub struct Tree<T> {
    pub(crate) root_id: Option<NodeId>,
    pub(crate) core_tree: CoreTree<T>,
}

impl<T> Tree<T> {
    ///
    /// Creates a new `Tree` with a capacity of 0.
    ///
    /// ```
    /// use slab_tree::tree::Tree;
    ///
    /// let tree: Tree<i32> = Tree::new();
    ///
    /// # assert_eq!(tree.capacity(), 0);
    /// ```
    ///
    pub fn new() -> Tree<T> {
        TreeBuilder::new().build()
    }

    //todo: write test for this
    ///
    /// Sets the "root" of the `Tree` to be `root`.
    ///
    /// If there is already a "root" node in the `Tree`, that node is shifted down and the new
    /// one takes its place.
    ///
    /// ```
    /// use slab_tree::tree::Tree;
    ///
    /// let mut tree = Tree::new();
    ///
    /// let root_id = tree.set_root(1);
    ///
    /// assert_eq!(tree.root_id().unwrap(), root_id);
    /// ```
    ///
    pub fn set_root(&mut self, root: T) -> NodeId {
        let old_root_id = self.root_id.take();
        let new_root_id = self.core_tree.insert(root);

        self.root_id = Some(new_root_id);

        self.set_first_child(new_root_id, old_root_id);
        self.set_last_child(new_root_id, old_root_id);

        if let Some(node_id) = old_root_id {
            self.set_parent(node_id, self.root_id);
        }

        new_root_id
    }

    ///
    /// Returns the `Tree`'s current capacity.  Capacity is defined as the number of times new
    /// `Node`s can be added to the `Tree` before it must allocate more memory.
    ///
    /// ```
    /// use slab_tree::tree::Tree;
    ///
    /// let tree: Tree<i32> = Tree::new();
    ///
    /// assert_eq!(tree.capacity(), 0);
    /// ```
    ///
    pub fn capacity(&self) -> usize {
        self.core_tree.capacity()
    }

    ///
    /// Returns the `NodeId` of the root node of the `Tree`.
    ///
    /// ```
    /// use slab_tree::tree::Tree;
    ///
    /// let mut tree = Tree::new();
    /// tree.set_root(1);
    ///
    /// let root_id = tree.root_id().expect("root doesn't exist?");
    ///
    /// assert_eq!(tree.get(root_id).unwrap().data(), &1);
    /// ```
    ///
    pub fn root_id(&self) -> Option<NodeId> {
        self.root_id
    }

    ///
    /// Returns a `NodeRef` pointing to the root `Node` of the `Tree`.
    ///
    /// ```
    /// use slab_tree::tree::Tree;
    ///
    /// let mut tree = Tree::new();
    /// tree.set_root(1);
    ///
    /// let root = tree.root().expect("root doesn't exist?");
    ///
    /// assert_eq!(root.data(), &1);
    /// ```
    ///
    pub fn root(&self) -> Option<NodeRef<T>> {
        self.root_id.map(|id| self.new_node_ref(id))
    }

    ///
    /// Returns a `NodeMut` pointing to the root `Node` of the `Tree`.
    ///
    /// ```
    /// use slab_tree::tree::Tree;
    ///
    /// let mut tree = Tree::new();
    /// tree.set_root(1);
    ///
    /// let mut root = tree.root_mut().expect("root doesn't exist?");
    /// assert_eq!(root.data(), &mut 1);
    ///
    /// *root.data() = 2;
    /// assert_eq!(root.data(), &mut 2);
    /// ```
    ///
    pub fn root_mut(&mut self) -> Option<NodeMut<T>> {
        self.root_id.map(move |id| self.new_node_mut(id))
    }

    ///
    /// Returns the `NodeRef` pointing to the `Node` that the given `NodeId` identifies.  If the
    /// `NodeId` in question points to nothing (or belongs to a different `Tree`) a `None`-value
    /// will be returned; otherwise, a `Some`-value will be returned.
    ///
    /// ```
    /// use slab_tree::tree::Tree;
    ///
    /// let mut tree = Tree::new();
    /// tree.set_root(1);
    /// let root_id = tree.root_id().expect("root doesn't exist?");
    ///
    /// let root = tree.get(root_id);
    /// assert!(root.is_some());
    ///
    /// let root = root.unwrap();
    /// assert_eq!(root.data(), &1);
    /// ```
    ///
    pub fn get(&self, node_id: NodeId) -> Option<NodeRef<T>> {
        let _ = self.core_tree.get(node_id)?;
        Some(self.new_node_ref(node_id))
    }

    ///
    /// Returns the `NodeMut` pointing to the `Node` that the given `NodeId` identifies.  If the
    /// `NodeId` in question points to nothing (or belongs to a different `Tree`) a `None`-value
    /// will be returned; otherwise, a `Some`-value will be returned.
    ///
    /// ```
    /// use slab_tree::tree::Tree;
    ///
    /// let mut tree = Tree::new();
    /// tree.set_root(1);
    /// let root_id = tree.root_id().expect("root doesn't exist?");
    ///
    /// let root = tree.get_mut(root_id);
    /// assert!(root.is_some());
    ///
    /// let mut root = root.unwrap();
    ///
    /// *root.data() = 2;
    /// assert_eq!(root.data(), &mut 2);
    /// ```
    ///
    pub fn get_mut(&mut self, node_id: NodeId) -> Option<NodeMut<T>> {
        let _ = self.core_tree.get_mut(node_id)?;
        Some(self.new_node_mut(node_id))
    }

    ///
    /// Remove a `Node` by its `NodeId` and return the data that it contained.
    /// Returns a `Some`-value if the `Node` exists; returns a `None`-value otherwise.
    ///
    /// Children of the removed `Node` can either be dropped with `DropChildren` or orphaned with
    /// `OrphanChildren`.
    ///
    /// ```
    /// use slab_tree::tree::TreeBuilder;
    /// use slab_tree::behaviors::RemoveBehavior::*;
    ///
    /// let mut tree = TreeBuilder::new().with_root(1).build();
    /// let two_id = {
    ///     let mut root = tree.root_mut().expect("root doesn't exist?");
    ///     let two_id = root.append(2).node_id();
    ///     root.append(3);
    ///     two_id
    /// };
    ///
    /// let two = tree.remove(two_id, DropChildren);
    ///
    /// assert!(two.is_some());
    /// assert_eq!(two.unwrap(), 2);
    ///
    /// let root = tree.root().expect("root doesn't exist?");
    /// assert!(root.first_child().is_some());
    /// assert_eq!(root.first_child().unwrap().data(), &mut 3);
    ///
    /// assert!(root.last_child().is_some());
    /// assert_eq!(root.last_child().unwrap().data(), &mut 3);
    /// ```
    ///
    pub fn remove(&mut self, node_id: NodeId, behavior: RemoveBehavior) -> Option<T> {
        if let Some(node) = self.get_node(node_id) {
            let Relatives {
                parent,
                prev_sibling,
                next_sibling,
                ..
            } = node.relatives;

            let (is_first_child, is_last_child) = self.is_node_first_last_child(node_id);

            if is_first_child {
                // parent first child = my next sibling
                self.set_first_child(parent.expect("parent must exist"), next_sibling);
            }
            if is_last_child {
                // parent last child = my prev sibling
                self.set_last_child(parent.expect("parent must exist"), prev_sibling);
            }
            if let Some(prev) = prev_sibling {
                self.set_next_sibling(prev, next_sibling);
            }
            if let Some(next) = next_sibling {
                self.set_prev_sibling(next, prev_sibling);
            }

            match behavior {
                RemoveBehavior::DropChildren => self.drop_children(node_id),
                RemoveBehavior::OrphanChildren => self.orphan_children(node_id),
            };
            if self.root_id == Some(node_id) {
                self.root_id = None;
            }
            self.core_tree.remove(node_id)
        } else {
            None
        }
    }

    pub(crate) fn get_node(&self, node_id: NodeId) -> Option<&Node<T>> {
        self.core_tree.get(node_id)
    }

    pub(crate) fn get_node_mut(&mut self, node_id: NodeId) -> Option<&mut Node<T>> {
        self.core_tree.get_mut(node_id)
    }

    pub(crate) fn set_prev_siblings_next_sibling(
        &mut self,
        current_id: NodeId,
        next_sibling: Option<NodeId>,
    ) {
        if let Some(prev_sibling_id) = self.get_node_prev_sibling_id(current_id) {
            self.set_next_sibling(prev_sibling_id, next_sibling);
        }
    }

    pub(crate) fn set_next_siblings_prev_sibling(
        &mut self,
        current_id: NodeId,
        prev_sibling: Option<NodeId>,
    ) {
        if let Some(next_sibling_id) = self.get_node_next_sibling_id(current_id) {
            self.set_prev_sibling(next_sibling_id, prev_sibling);
        }
    }

    pub(crate) fn set_parent(&mut self, node_id: NodeId, parent_id: Option<NodeId>) {
        if let Some(node) = self.get_node_mut(node_id) {
            node.relatives.parent = parent_id;
        } else {
            unreachable!()
        }
    }

    pub(crate) fn set_prev_sibling(&mut self, node_id: NodeId, prev_sibling: Option<NodeId>) {
        if let Some(node) = self.get_node_mut(node_id) {
            node.relatives.prev_sibling = prev_sibling;
        } else {
            unreachable!()
        }
    }

    pub(crate) fn set_next_sibling(&mut self, node_id: NodeId, next_sibling: Option<NodeId>) {
        if let Some(node) = self.get_node_mut(node_id) {
            node.relatives.next_sibling = next_sibling;
        } else {
            unreachable!()
        }
    }

    pub(crate) fn set_first_child(&mut self, node_id: NodeId, first_child: Option<NodeId>) {
        if let Some(node) = self.get_node_mut(node_id) {
            node.relatives.first_child = first_child;
        } else {
            unreachable!()
        }
    }

    pub(crate) fn set_last_child(&mut self, node_id: NodeId, last_child: Option<NodeId>) {
        if let Some(node) = self.get_node_mut(node_id) {
            node.relatives.last_child = last_child;
        } else {
            unreachable!()
        }
    }

    pub(crate) fn get_node_prev_sibling_id(&self, node_id: NodeId) -> Option<NodeId> {
        if let Some(node) = self.get_node(node_id) {
            node.relatives.prev_sibling
        } else {
            unreachable!()
        }
    }

    pub(crate) fn get_node_next_sibling_id(&self, node_id: NodeId) -> Option<NodeId> {
        if let Some(node) = self.get_node(node_id) {
            node.relatives.next_sibling
        } else {
            unreachable!()
        }
    }

    pub(crate) fn get_node_relatives(&self, node_id: NodeId) -> Relatives {
        if let Some(node) = self.get_node(node_id) {
            node.relatives
        } else {
            unreachable!()
        }
    }

    fn drop_children(&mut self, node_id: NodeId) {
        let sub_tree_ids: Vec<NodeId> = self
            .get(node_id)
            .expect("node must exist")
            .traverse_level_order()
            .skip(1) // skip the "root" of the sub-tree, which is the "current" node
            .map(|node_ref| node_ref.node_id())
            .collect();

        for id in sub_tree_ids {
            self.core_tree.remove(id);
        }
    }

    fn orphan_children(&mut self, node_id: NodeId) {
        let child_ids: Vec<NodeId> = self
            .get(node_id)
            .expect("node must exist")
            .children()
            .map(|node_ref| node_ref.node_id())
            .collect();

        for id in child_ids {
            self.set_parent(id, None);
        }
    }

    fn new_node_ref(&self, node_id: NodeId) -> NodeRef<T> {
        NodeRef::new(node_id, self)
    }

    fn new_node_mut(&mut self, node_id: NodeId) -> NodeMut<T> {
        NodeMut::new(node_id, self)
    }

    fn is_node_first_last_child(&self, node_id: NodeId) -> (bool, bool) {
        if let Some(node) = self.get_node(node_id) {
            node.relatives
                .parent
                .and_then(|parent_id| self.get_node(parent_id))
                .map(|parent| {
                    let Relatives {
                        first_child: first,
                        last_child: last,
                        ..
                    } = parent.relatives;
                    (
                        first.map(|child_id| child_id == node_id).unwrap_or(false),
                        last.map(|child_id| child_id == node_id).unwrap_or(false),
                    )
                })
                .unwrap_or((false, false))
        } else {
            (false, false)
        }
    }
}

impl<T> Default for Tree<T> {
    fn default() -> Self {
        TreeBuilder::new().build()
    }
}

impl<T: std::fmt::Debug> Tree<T> {
    /// Write formatted tree representation and nodes with debug formatting.
    ///
    /// Example:
    ///
    /// ```
    /// use slab_tree::tree::TreeBuilder;
    ///
    /// let mut tree = TreeBuilder::new().with_root(0).build();
    /// let mut root = tree.root_mut().unwrap();
    /// root.append(1)
    ///     .append(2);
    /// root.append(3);
    /// let mut s = String::new();
    /// tree.write_formatted(&mut s).unwrap();
    /// assert_eq!(&s, "\
    /// 0
    /// ├── 1
    /// │   └── 2
    /// └── 3
    /// ");
    /// ```
    ///
    /// Writes nothing if the tree is empty.
    ///
    /// ```
    /// use slab_tree::tree::TreeBuilder;
    ///
    /// let tree = TreeBuilder::<i32>::new().build();
    /// let mut s = String::new();
    /// tree.write_formatted(&mut s).unwrap();
    /// assert_eq!(&s, "");
    /// ```
    pub fn write_formatted<W: std::fmt::Write>(&self, w: &mut W) -> std::fmt::Result {
        if let Some(root) = self.root() {
            let node_id = root.node_id();
            let childn = 0;
            let level = 0;
            let last = vec![];
            let mut stack = vec![(node_id, childn, level, last)];
            while let Some((node_id, childn, level, last)) = stack.pop() {
                debug_assert_eq!(
                    last.len(),
                    level,
                    "each previous level should indicate whether it has reached the last node"
                );
                let node = self
                    .get(node_id)
                    .expect("getting node of existing node ref id");
                if childn == 0 {
                    for i in 1..level {
                        if last[i - 1] {
                            write!(w, "    ")?;
                        } else {
                            write!(w, "│   ")?;
                        }
                    }
                    if level > 0 {
                        if last[level - 1] {
                            write!(w, "└── ")?;
                        } else {
                            write!(w, "├── ")?;
                        }
                    }
                    writeln!(w, "{:?}", node.data())?;
                }
                let mut children = node.children().skip(childn);
                if let Some(child) = children.next() {
                    let mut next_last = last.clone();
                    if children.next().is_some() {
                        stack.push((node_id, childn + 1, level, last));
                        next_last.push(false);
                    } else {
                        next_last.push(true);
                    }
                    stack.push((child.node_id(), 0, level + 1, next_last));
                }
            }
        }
        Ok(())
    }
}

#[cfg_attr(tarpaulin, skip)]
#[cfg(test)]
mod tree_tests {
    use super::*;
    use crate::behaviors::RemoveBehavior::{DropChildren, OrphanChildren};

    #[test]
    fn capacity() {
        let tree = TreeBuilder::new().with_root(1).with_capacity(5).build();
        assert_eq!(tree.capacity(), 5);
    }

    #[test]
    fn root_id() {
        let tree = TreeBuilder::new().with_root(1).build();
        let root_id = tree.root_id().expect("root doesn't exist?");
        let root = tree.get(root_id).unwrap();
        assert_eq!(root.data(), &1);
    }

    #[test]
    fn remove_root_drop() {
        let mut tree = TreeBuilder::new().with_root(1).build();
        let root_id = tree.root_id().expect("root doesn't exist?");

        tree.remove(root_id, RemoveBehavior::DropChildren);
        assert!(tree.root().is_none());
    }

    #[test]
    fn remove_root_orphan() {
        let mut tree = TreeBuilder::new().with_root(1).build();
        let root_id = tree.root_id().expect("root doesn't exist?");

        tree.remove(root_id, RemoveBehavior::OrphanChildren);
        assert!(tree.root().is_none());
    }

    #[test]
    fn root() {
        let tree = TreeBuilder::new().with_root(1).build();
        let root = tree.root().expect("root doesn't exist?");
        assert_eq!(root.data(), &1);
    }

    #[test]
    fn root_mut() {
        let mut tree = TreeBuilder::new().with_root(1).build();
        let mut root = tree.root_mut().expect("root doesn't exist?");

        assert_eq!(root.data(), &mut 1);

        *root.data() = 2;
        assert_eq!(root.data(), &mut 2);
    }

    #[test]
    fn get() {
        let tree = TreeBuilder::new().with_root(1).build();

        let root_id = tree.root_id().expect("root doesn't exist?");
        let root = tree.get(root_id);
        assert!(root.is_some());

        let root = root.unwrap();
        assert_eq!(root.data(), &1);
    }

    #[test]
    fn get_mut() {
        let mut tree = TreeBuilder::new().with_root(1).build();

        let root_id = tree.root_id().expect("root doesn't exist?");
        let root = tree.get_mut(root_id);
        assert!(root.is_some());

        let mut root = root.unwrap();
        assert_eq!(root.data(), &mut 1);

        *root.data() = 2;
        assert_eq!(root.data(), &mut 2);
    }

    #[test]
    fn get_node() {
        let tree = TreeBuilder::new().with_root(1).build();

        let root_id = tree.root_id().expect("root doesn't exist?");
        let root = tree.get_node(root_id);
        assert!(root.is_some());

        let root = root.unwrap();
        assert_eq!(root.data, 1);
    }

    #[test]
    fn get_node_mut() {
        let mut tree = TreeBuilder::new().with_root(1).build();

        let root_id = tree.root_id().expect("root doesn't exist?");
        let root = tree.get_node_mut(root_id);
        assert!(root.is_some());

        let root = root.unwrap();
        assert_eq!(root.data, 1);

        root.data = 2;
        assert_eq!(root.data, 2);
    }

    #[test]
    fn remove_drop() {
        let mut tree = TreeBuilder::new().with_root(1).build();

        let two_id;
        let three_id;
        let four_id;
        let five_id;
        {
            let mut root = tree.root_mut().expect("root doesn't exist?");
            two_id = root.append(2).node_id();
            three_id = root.append(3).node_id();
            four_id = root.append(4).node_id();
        }
        {
            five_id = tree
                .get_mut(three_id)
                .expect("three doesn't exist?")
                .append(5)
                .node_id();
        }

        //        1
        //      / | \
        //     2  3  4
        //        |
        //        5

        tree.remove(three_id, DropChildren);

        let root = tree
            .get_node(tree.root_id().expect("tree doesn't exist?"))
            .unwrap();
        assert!(root.relatives.first_child.is_some());
        assert!(root.relatives.last_child.is_some());
        assert_eq!(root.relatives.first_child.unwrap(), two_id);
        assert_eq!(root.relatives.last_child.unwrap(), four_id);

        let two = tree.get_node(two_id);
        assert!(two.is_some());

        let two = two.unwrap();
        assert_eq!(two.relatives.next_sibling, Some(four_id));

        let four = tree.get_node(four_id);
        assert!(four.is_some());

        let four = four.unwrap();
        assert_eq!(four.relatives.prev_sibling, Some(two_id));

        let five = tree.get_node(five_id);
        assert!(five.is_none());
    }

    /// Test that there is no panic if caller tries to remove a removed node
    #[test]
    fn address_dropped() {
        let mut tree = TreeBuilder::new().with_root(1).build();
        let two_id = tree.root_mut().expect("root doesn't exist").node_id();
        tree.remove(two_id, DropChildren);
        tree.remove(two_id, DropChildren);
    }

    #[test]
    fn remove_orphan() {
        let mut tree = TreeBuilder::new().with_root(1).build();

        let two_id;
        let three_id;
        let four_id;
        let five_id;
        {
            let mut root = tree.root_mut().expect("root doesn't exist?");
            two_id = root.append(2).node_id();
            three_id = root.append(3).node_id();
            four_id = root.append(4).node_id();
        }
        {
            five_id = tree
                .get_mut(three_id)
                .expect("three doesn't exist?")
                .append(5)
                .node_id();
        }

        //        1
        //      / | \
        //     2  3  4
        //        |
        //        5

        tree.remove(three_id, OrphanChildren);

        let root = tree
            .get_node(tree.root_id().expect("tree doesn't exist?"))
            .unwrap();
        assert!(root.relatives.first_child.is_some());
        assert!(root.relatives.last_child.is_some());
        assert_eq!(root.relatives.first_child.unwrap(), two_id);
        assert_eq!(root.relatives.last_child.unwrap(), four_id);

        let two = tree.get_node(two_id);
        assert!(two.is_some());

        let two = two.unwrap();
        assert_eq!(two.relatives.next_sibling, Some(four_id));

        let four = tree.get_node(four_id);
        assert!(four.is_some());

        let four = four.unwrap();
        assert_eq!(four.relatives.prev_sibling, Some(two_id));

        let five = tree.get_node(five_id);
        assert!(five.is_some());

        let five = five.unwrap();
        assert_eq!(five.relatives.parent, None);
    }
}
