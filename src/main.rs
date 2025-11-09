mod bptree;
use bptree::BPlusTree;

fn main() {
    let name = env!("CARGO_PKG_NAME");
    let author = env!("CARGO_PKG_AUTHORS");
    let version = env!("CARGO_PKG_VERSION");

    println!("{name} v{version} by {author}");

    let _t: BPlusTree<i32, i32, 4> = BPlusTree::new();
    println!("{}", _t.debug_dump());
}
