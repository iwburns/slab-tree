extern crate slab_tree;

use slab_tree::*;

fn main() {
    //      "hello"
    //        / \
    // "world"   "trees"
    //              |
    //            "are"
    //              |
    //            "cool"

    let mut tree = TreeBuilder::new().with_root("hello").build();
    let root_id = tree.root_id().expect("root doesn't exist?");
    let mut hello = tree.get_mut(root_id).unwrap();

    hello.append("world");
    hello.append("trees").append("are").append("cool");
}
