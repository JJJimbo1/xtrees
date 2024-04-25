use crate::{Dimension, TNode, Tree, BASE_CAPACITY, MAX_DEPTH};
use std::{fmt::Debug, hash::Hash};

///3-dimensional cube.
#[derive(Debug, Clone, Copy)]
pub struct Oct {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub half_x: f32,
    pub half_y: f32,
    pub half_z: f32,
}

impl Oct {
    pub fn new(x: f32, y: f32, z: f32, half_x: f32, half_y: f32, half_z: f32) -> Self {
        Self {
            x,
            y,
            z,
            half_x,
            half_y,
            half_z,
        }
    }
}

impl Dimension for Oct {
    const SUB_COUNT: usize = 8;
    #[inline]
    fn overlaps(&self, area: &Self) -> bool {
        (self.x - area.x).abs() < (self.half_x + area.half_x)
            && (self.y - area.y).abs() < (self.half_y + area.half_y)
            && (self.z - area.z).abs() < (self.half_z + area.half_z)
    }
    #[inline]
    fn contains_center(&self, point: &Self) -> bool {
        point.x <= self.x + self.half_x
            && point.y <= self.y + self.half_y
            && point.z <= self.z + self.half_z
            && point.x >= self.x - self.half_x
            && point.y >= self.y - self.half_y
            && point.z >= self.z - self.half_z
    }
    fn subdivisions(&self) -> [Self; Self::SUB_COUNT] {
        let bsw = Self::new(
            self.x - self.half_x / 2.0,
            self.y - self.half_y / 2.0,
            self.z - self.half_z / 2.0,
            self.half_x / 2.0,
            self.half_y / 2.0,
            self.half_z / 2.0,
        );
        let bse = Self::new(
            self.x + self.half_x / 2.0,
            self.y - self.half_y / 2.0,
            self.z - self.half_z / 2.0,
            self.half_x / 2.0,
            self.half_y / 2.0,
            self.half_z / 2.0,
        );
        let bnw = Self::new(
            self.x - self.half_x / 2.0,
            self.y + self.half_y / 2.0,
            self.z - self.half_z / 2.0,
            self.half_x / 2.0,
            self.half_y / 2.0,
            self.half_z / 2.0,
        );
        let bne = Self::new(
            self.x + self.half_x / 2.0,
            self.y + self.half_y / 2.0,
            self.z - self.half_z / 2.0,
            self.half_x / 2.0,
            self.half_y / 2.0,
            self.half_z / 2.0,
        );
        let asw = Self::new(
            self.x - self.half_x / 2.0,
            self.y - self.half_y / 2.0,
            self.z + self.half_z / 2.0,
            self.half_x / 2.0,
            self.half_y / 2.0,
            self.half_z / 2.0,
        );
        let ase = Self::new(
            self.x + self.half_x / 2.0,
            self.y - self.half_y / 2.0,
            self.z + self.half_z / 2.0,
            self.half_x / 2.0,
            self.half_y / 2.0,
            self.half_z / 2.0,
        );
        let anw = Self::new(
            self.x - self.half_x / 2.0,
            self.y + self.half_y / 2.0,
            self.z + self.half_z / 2.0,
            self.half_x / 2.0,
            self.half_y / 2.0,
            self.half_z / 2.0,
        );
        let ane = Self::new(
            self.x + self.half_x / 2.0,
            self.y + self.half_y / 2.0,
            self.z + self.half_z / 2.0,
            self.half_x / 2.0,
            self.half_y / 2.0,
            self.half_z / 2.0,
        );

        [bsw, bse, bnw, bne, asw, ase, anw, ane]
    }
}

///3-dimensional tree representation.
pub type OctTree<I> = Tree<I, Oct, TNode<I, Oct>>;

impl<I: Debug + Clone + Copy + Hash + PartialEq + Eq> OctTree<I> {
    pub fn new(translation: Oct) -> Self {
        Tree::new_tree(TNode::new(translation, BASE_CAPACITY, MAX_DEPTH))
    }
}
