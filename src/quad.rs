pub use quad::*;

mod quad {
    use std::{hash::Hash, fmt::Debug};
    use crate::{Dimension, TNode, Tree};

    ///2-dimensional square.
    #[derive(Debug, Clone, Copy)]
    pub struct Quad {
        pub x : f32,
        pub y : f32,
        pub half_x : f32,
        pub half_y : f32,
    }

    impl Quad {
        pub fn new(x : f32, y : f32, half_x : f32, half_y : f32,) -> Self {
            Self {
                x,
                y,
                half_x,
                half_y,
            }
        }
    }

    impl Dimension for Quad {
        const SUB_COUNT : usize = 4;
        fn overlaps(&self, area : &Self) -> bool {
            if (self.x - area.x).abs() > (self.half_x + area.half_x) {return false;}
            if (self.y - area.y).abs() > (self.half_y + area.half_y) {return false;}
            return true;
        }
        fn contains_center(&self, point : &Self) -> bool {
            point.x <= self.x + self.half_x
            && point.y <= self.y + self.half_y
            && point.x >= self.x - self.half_x
            && point.y >= self.y - self.half_y
        }
        fn subdivisions(&self) -> [Self; Self::SUB_COUNT] {
            let sw = Self::new(
                self.x - self.half_x / 2.0,
                self.y - self.half_y / 2.0,
                self.half_x / 2.0,
                self.half_y / 2.0,
            );
            let se = Self::new(
                self.x + self.half_x / 2.0,
                self.y - self.half_y / 2.0,
                self.half_x / 2.0,
                self.half_y / 2.0,
            );
            let nw = Self::new(
                self.x - self.half_x / 2.0,
                self.y + self.half_y / 2.0,
                self.half_x / 2.0,
                self.half_y / 2.0,
            );
            let ne = Self::new(
                self.x + self.half_x / 2.0,
                self.y + self.half_y / 2.0,
                self.half_x / 2.0,
                self.half_y / 2.0,
            );

            [sw, se, nw, ne]
        }
    }

    ///2-dimensional tree representation.
    pub type QuadTree<I> = Tree<I, Quad, TNode<I, Quad>>;

    impl<I : Debug + Clone + Copy + Hash + PartialEq + Eq> QuadTree<I> {
        pub fn new(translation : Quad, capacity : u8, max_depth : u8) -> Self {
            Tree::new_tree(TNode::new(translation, capacity, max_depth))
        }
    }
}