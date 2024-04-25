use crate::{Dimension, TNode, Tree};
use std::{fmt::Debug, hash::Hash};

///2-dimensional square.
#[derive(Debug, Clone, Copy)]
pub struct Quad {
    pub x: f32,
    pub y: f32,
    pub half_x: f32,
    pub half_y: f32,
}

impl Quad {
    pub fn new(x: f32, y: f32, half_x: f32, half_y: f32) -> Self {
        Self {
            x,
            y,
            half_x,
            half_y,
        }
    }
}

impl Dimension for Quad {
    const SUB_COUNT: usize = 4;
    fn overlaps(&self, area: &Self) -> bool {
        !((self.x - area.x).abs() > (self.half_x + area.half_x) || (self.y - area.y).abs() > (self.half_y + area.half_y))
    }
    fn contains_center(&self, point: &Self) -> bool {
        let point_x = point.x - self.x;
        let point_y = point.y - self.y;

            point_x <= self.half_x
            && point_y <= self.half_y
            && point_x >= -self.half_x
            && point_y >= -self.half_y
    }
    fn subdivisions(&self) -> [Self; Self::SUB_COUNT] {

        let half_x = self.half_x / 2.0;
        let half_y = self.half_y / 2.0;
        let left = self.x - half_x;
        let right = self.x + half_x;
        let down = self.y - half_y;
        let up = self.y + half_y;

        let sw = Self::new(
            left,
            down,
            half_x,
            half_y,
        );
        let se = Self::new(
            right,
            down,
            half_x,
            half_y,
        );
        let nw = Self::new(
            left,
            up,
            half_x,
            half_y,
        );
        let ne = Self::new(
            right,
            up,
            half_x,
            half_y,
        );

        [sw, se, nw, ne]
    }
}

///2-dimensional tree representation.
pub type QuadTree<I> = Tree<I, Quad, TNode<I, Quad>>;

impl<I: Debug + Clone + Copy + Hash + PartialEq + Eq> QuadTree<I> {
    pub fn new(translation: Quad, capacity: u8, max_depth: u8) -> Self {
        Tree::new_tree(TNode::new(translation, capacity, max_depth))
    }
}
