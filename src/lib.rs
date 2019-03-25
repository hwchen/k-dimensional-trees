/// collection of trees for k-dimensional values (points)
/// - kd-tree, an in-memory binary tree
/// - kdb-tree, an external (stored on disk) b-tree
/// - bkd-tree, a hybrid external tree which has about the same
///     performance as a kdb-tree, but much better amortized update
///     times and index disk usage.

// Starting out with only 2 dimensions, and only on integers

mod kd_tree;
pub use kd_tree::KdTree;

/// Currently experimental, only basic outline of structure implemented
mod kdb_tree;
pub use kdb_tree::KdbTree;

/// An interface to allow support for k-dimensional points (integers and
/// floats). But for now, only allows 2 dimensions, and only integers
#[derive(Debug, PartialEq, Eq)]
pub struct Point {
    x: i64,
    y: i64,
}

