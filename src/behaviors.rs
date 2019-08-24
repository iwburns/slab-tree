///
/// Describes all the possible ways to remove a Node from a Tree.
///
pub enum RemoveBehavior {
    ///
    /// All children of the removed Node will be dropped from the Tree.  All children (and all
    /// Nodes in each of their sub-trees) will no longer exist in the Tree after this operation.
    ///
    /// This is slower than `OrphanChildren` but frees up space inside the Tree.
    ///
    DropChildren,

    ///
    /// All children of the removed Node will be left in the Tree (still accessible via NodeIds).
    /// However, each child (and their sub-trees) will no longer be connected to the rest of the
    /// Nodes in the Tree.
    ///
    /// Orphaned nodes will live in the Tree until they are manually removed or until the Tree is
    /// Dropped.  This is faster than `DropChildren` but doesn't free up any space inside the Tree.
    ///
    OrphanChildren,
}
