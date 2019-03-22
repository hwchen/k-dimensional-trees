/// Pointer points at a child which represents values
/// - after values in previous child in list, (including prev child K)
/// - up to and not including K

// First basic with 2-dim integer points, and integer value-ids
//   - basic tree structure
//   - manually create tree
//   - basic query
//   - basic construct from bulk load
//   - real tree structure
//   - real construct from bulk load
//   - real query
//   - write to disk and read back nodes
//   - bkd insert algorithm
// then make generic structure work

mod kdb_tree;

use kdb_tree::KdbTree;

#[derive(Debug)]
pub struct Point {
    x: i64,
    y: i64,
}

pub struct BkdTree {
    subtrees: Vec<KdbTree>,
}
