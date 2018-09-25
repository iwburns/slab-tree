use std::error::Error;
use std::fmt;

// todo: consider removing these errors in favor of just returning Option instead of Result types
// if we keep these in though, we need to write tests for all of these

///
/// Enum for all of the possible `NodeId` errors that could occur.
///
#[derive(Debug, Eq, PartialEq)]
pub enum NodeIdError {
    /// Indicates a `NodeId` was used to access data in a `Tree` from which it did not originate.
    WrongTree,
    /// Indicates a `NodeId` was used that doesn't point to anything.  This is most likely because the
    /// `NodeId` is an old NodeId that was cloned (for whatever reason) by the library consumer.  It is
    /// also possible for this to occur because of a bug in slab_tree.
    BadNodeId,
}

impl NodeIdError {
    fn to_string(&self) -> &str {
        match *self {
            NodeIdError::WrongTree => "The given NodeId belongs to a different Tree.",
            NodeIdError::BadNodeId => {
                "The given NodeId does not point to any data in the Tree. The Node in
                question has most likely been removed."
            }
        }
    }
}

impl fmt::Display for NodeIdError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "NodeIdError: {}", self.to_string())
    }
}

impl Error for NodeIdError {
    fn description(&self) -> &str {
        self.to_string()
    }
}
