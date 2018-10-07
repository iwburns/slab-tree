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

    let mut tree = Tree::new("hello");
    let root_id = tree.root_id();
    let mut hello = tree.get_mut(root_id).ok().unwrap();

    hello.append("world");
    hello
        .append("trees")
        .append("are")
        .append("cool");
}