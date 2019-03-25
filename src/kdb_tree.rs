// TODO determine how to split: use Bbox?

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

use std::fmt;

use super::Point;

pub trait KdbNode {
    fn to_string(&self, pad: &str, arrow: &str) -> String;
}

pub struct KdbTree {
    // split on the "k" dimension (referencing
    // the indexing of the point tuple)
    // There should be one less split than children
    splits: Vec<Split>,
    children: Vec<Box<KdbNode>>,
}

impl fmt::Display for KdbTree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let pad = "    ";
        let mut out = self.children[self.children.len() - 1]
            .to_string(&pad, "↦ ");

        for i in (0..self.splits.len()).rev() {
            out.push_str(&format!("\n{:?}\n", self.splits[i]));
            out.push_str(&self.children[i].to_string(&pad, "↦ "));
        }
        write!(f, "{}\n", out)
        //"↳ ""↱ "
    }
}

struct KdbInnerNode {
    splits: Vec<Split>,
    children: Vec<Box<KdbNode>>,
}
impl KdbNode for KdbInnerNode {
    fn to_string(&self, pad: &str, arrow: &str) -> String {
        let mut out = self.children[self.children.len() - 1]
            .to_string(&(pad.to_owned() + pad), "↦ ");

        for i in (0..self.splits.len()).rev() {
            out.push_str(&format!("\n{}{}{:?}\n", pad, arrow, self.splits[i]));
            out.push_str(&self.children[i]
                .to_string(&(pad.to_owned() + pad), "↦ ")
            );
        }
        out
    }
}

struct KdbLeaf {
    points: Vec<Point>,
    values: Vec<i64>, // reads in a vec of block size
}
impl KdbNode for KdbLeaf {
    fn to_string(&self, pad: &str, arrow: &str) -> String{
        format!("{}{}points: {:?} values: {:?}", pad, arrow, self.points, self.values)
    }
}

#[derive(Debug)]
struct Split {
    axis: u64,
    value: u64,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[ignore]
    fn basic_create_kdb_tree() {
        // leaf 1 top right quadrant
        let leaf_1 = KdbLeaf {
            points: vec![
                Point {
                    x: 1,
                    y: 1,
                },
                Point {
                    x: 2,
                    y: 2,
                },
            ],
            values: vec![11, 12],
        };

        // leaf 2 bottom right quadrant
        let leaf_2 = KdbLeaf {
            points: vec![
                Point {
                    x: 1,
                    y: -1,
                },
                Point {
                    x: 2,
                    y: -2,
                },
            ],
            values: vec![21, 22],
        };

        // leaf 3 bottom left quadrant
        let leaf_3 = KdbLeaf {
            points: vec![
                Point {
                    x: -1,
                    y: -1,
                },
                Point {
                    x: -2,
                    y: -2,
                },
            ],
            values: vec![31, 32],
        };

        // leaf 4 top left quadrant
        let leaf_4 = KdbLeaf {
            points: vec![
                Point {
                    x: -1,
                    y: 1,
                },
                Point {
                    x: -2,
                    y: 2,
                },
            ],
            values: vec![41, 42],
        };

        // construct kdb tree, with root split on "horizontal",
        // children on "vertical"

        let tree = KdbTree {
            splits: vec![Split { axis: 0, value: 0 }],
            children: vec![
                Box::new(KdbInnerNode {
                    splits: vec![Split { axis: 1, value: 0 }],
                    children: vec![ Box::new(leaf_4), Box::new(leaf_1)],
                }),
                Box::new(KdbInnerNode {
                    splits: vec![Split { axis: 1, value: 0 }],
                    children: vec![ Box::new(leaf_3), Box::new(leaf_2)],
                }),
            ],
        };

        println!("{}", tree);
        panic!();
    }
}
