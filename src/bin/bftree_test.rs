use bf_tree::BfTree;
use bf_tree::LeafReadResult;

fn main() {
    let mut config = bf_tree::Config::default();
    config.cb_min_record_size(4);
    let tree = BfTree::with_config(config, None).unwrap();
    tree.insert(b"key", b"value");

    let mut buffer = [0u8; 1024];
    let read_size = tree.read(b"key", &mut buffer);

    assert_eq!(read_size, LeafReadResult::Found(5));
    assert_eq!(&buffer[..5], b"value");
}
