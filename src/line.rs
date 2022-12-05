pub use line::*;

mod line {
    use std::{hash::Hash, fmt::Debug};
    use crate::{Dimension, TNode, Tree};

    ///1-dimensional line.
    #[derive(Debug, Clone, Copy)]
    pub struct Line {
        pub x : f32,
        pub half_x : f32,
    }

    impl Line {
        pub fn new(x : f32, half_x : f32) -> Self {
            Self {
                x,
                half_x,
            }
        }
    }

    impl Dimension for Line {
        const SUB_COUNT : usize = 2;
        fn overlaps(&self, area : &Self) -> bool {
            if (self.x - area.x).abs() > (self.half_x + area.half_x) {return false;}
            return true;
        }
        fn contains_center(&self, point : &Self) -> bool {
            point.x <= self.x + self.half_x
            && point.x >= self.x - self.half_x
        }
        fn subdivisions(&self) -> [Self; Self::SUB_COUNT] {
            let w = Self::new(
                self.x - self.half_x / 2.0,
                self.half_x / 2.0,
            );
            let e = Self::new(
                self.x + self.half_x / 2.0,
                self.half_x / 2.0,
            );

            [w, e]
        }
    }

    ///1-dimensional tree representation.
    pub type LineTree<I> = Tree<I, Line, TNode<I, Line>>;

    impl<I : Debug + Clone + Copy + Hash + PartialEq + Eq> LineTree<I> {
        pub fn new(translation : Line, capacity : u8, max_depth : u8) -> Self {
            Tree::new_tree(TNode::new(translation, capacity, max_depth))
        }
    }
}