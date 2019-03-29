use super::Point;

/// A k-dimensional tree implemented as a linear array
///
// Has an optional buffer size; if set, will return
// when tree goes beyond buffer. Used to trigger
// a bkd-tree update.
pub struct KdTree {
    // buffer holds Option, because the tree is not
    // necessarily "even" at the leaves and we need
    // to be able to hold null values
    buffer: Vec<Option<Point>>
}

impl KdTree {
    pub fn with_buffer_capacity(n: usize) -> Self {
        KdTree {
            buffer: Vec::with_capacity(n),
        }
    }

    pub fn insert(&mut self, point: Point) -> bool {
        //v represents a node
        // start from root
        let mut v_idx = 0;

        // depth needs to be tracked, because split (comparison) is done
        // on dimension of point depending on depth of tree
        let mut depth = 0;

        // walk
        while let Some(v) = self.buffer.get(v_idx) {
            // - if it's null, then break out of the loop and point will be
            //     inserted at v_idx
            // - if it's not null, then 
            //   - if it's equal to point, then terminate and return false
            //   - if it's !equal to point, then travel to l/r child
            //     depending on if point is < v, or >= v by setting
            //     `v_idx = 2*v_idx + (1|2);`
            //
            //     Even levels split orthogonal to x axis (vertical line)
            //     Odd levels split orthogonal to y axis (horizontal line)

            // dimension is either x or y for now; in future support k dims.
            // dimension is 0-indexed
            let dimension = depth % 2;

            if let Some(v) = v {
                if *v == point {
                    return false;
                } else {
                    match dimension {
                        0 => {
                            if point.x < v.x {
                                v_idx = 2*v_idx + 1;
                            } else {
                                v_idx = 2*v_idx + 2;
                            }
                        },
                        1 => {
                            if point.y < v.y {
                                v_idx = 2*v_idx + 1;
                            } else {
                                v_idx = 2*v_idx + 2;
                            }
                        },
                        _ => unreachable!(),
                    }
                }

                depth += 1;
            } else {
                break;
            }
        }

        // if v_idx is outside of vec len, extend vec len. This is inexpensive
        //   because vec allocates full capacity at beginning.
        if v_idx >= self.buffer.len() {
            let extension = (0..= v_idx - self.buffer.len())
                .map(|_| None);

            self.buffer.extend(extension);
        }

        // insert
        // can use idx, because vec len was already extended if necessary in loop
        self.buffer[v_idx] = Some(point);
        true
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn kd_tree_basic() {
        let mut tree = KdTree::with_buffer_capacity(10);
        tree.insert(Point { x: 5, y: 5 });

        // root splits on x, so this is left node
        tree.insert(Point { x: 4, y: 6 });

        // root splits on x, so this is left node of root
        // then it's the left node of that one
        tree.insert(Point { x: 3, y: 5 });

        // left node of root, right node of childe
        tree.insert(Point { x: 4, y: 7 });

        assert_eq!(tree.buffer,
            vec![
                Some(Point { x: 5, y: 5 }),
                Some(Point { x: 4, y: 6 }),
                None,
                Some(Point { x: 3, y: 5 }),
                Some(Point { x: 4, y: 7 }),
            ],
        );
    }
}
