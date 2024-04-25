use crate::{Dimension, TNode, Tree};
use std::{fmt::Debug, hash::Hash};

///4-dimensional tesseract.
#[derive(Debug, Clone, Copy)]
pub struct Tess {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
    pub half_x: f32,
    pub half_y: f32,
    pub half_z: f32,
    pub half_w: f32,
}

impl Tess {
    pub fn new(
        x: f32,
        y: f32,
        z: f32,
        w: f32,
        half_x: f32,
        half_y: f32,
        half_z: f32,
        half_w: f32,
    ) -> Self {
        Self {
            x,
            y,
            z,
            w,
            half_x,
            half_y,
            half_z,
            half_w,
        }
    }
}

impl Dimension for Tess {
    const SUB_COUNT: usize = 16;
    fn overlaps(&self, area: &Self) -> bool {
        if (self.x - area.x).abs() > (self.half_x + area.half_x) {
            return false;
        }
        if (self.y - area.y).abs() > (self.half_y + area.half_y) {
            return false;
        }
        if (self.z - area.z).abs() > (self.half_z + area.half_z) {
            return false;
        }
        if (self.w - area.w).abs() > (self.half_w + area.half_w) {
            return false;
        }

        return true;
    }
    fn contains_center(&self, point: &Self) -> bool {
        point.x <= self.x + self.half_x
            && point.y <= self.y + self.half_y
            && point.z <= self.z + self.half_z
            && point.w <= self.w + self.half_w
            && point.x >= self.x - self.half_x
            && point.y >= self.y - self.half_y
            && point.z >= self.z - self.half_z
            && point.w >= self.w - self.half_w
    }
    fn subdivisions(&self) -> [Self; Self::SUB_COUNT] {
        let xbsw = Self::new(
            self.x - self.half_x / 2.0,
            self.y - self.half_y / 2.0,
            self.z - self.half_z / 2.0,
            self.w - self.half_w / 2.0,
            self.half_x / 2.0,
            self.half_y / 2.0,
            self.half_z / 2.0,
            self.half_w / 2.0,
        );
        let xbse = Self::new(
            self.x + self.half_x / 2.0,
            self.y - self.half_y / 2.0,
            self.z - self.half_z / 2.0,
            self.w - self.half_w / 2.0,
            self.half_x / 2.0,
            self.half_y / 2.0,
            self.half_z / 2.0,
            self.half_w / 2.0,
        );
        let xbnw = Self::new(
            self.x - self.half_x / 2.0,
            self.y + self.half_y / 2.0,
            self.z - self.half_z / 2.0,
            self.w - self.half_w / 2.0,
            self.half_x / 2.0,
            self.half_y / 2.0,
            self.half_z / 2.0,
            self.half_w / 2.0,
        );
        let xbne = Self::new(
            self.x + self.half_x / 2.0,
            self.y + self.half_y / 2.0,
            self.z - self.half_z / 2.0,
            self.w - self.half_w / 2.0,
            self.half_x / 2.0,
            self.half_y / 2.0,
            self.half_z / 2.0,
            self.half_w / 2.0,
        );
        let xasw = Self::new(
            self.x - self.half_x / 2.0,
            self.y - self.half_y / 2.0,
            self.z + self.half_z / 2.0,
            self.w - self.half_w / 2.0,
            self.half_x / 2.0,
            self.half_y / 2.0,
            self.half_z / 2.0,
            self.half_w / 2.0,
        );
        let xase = Self::new(
            self.x + self.half_x / 2.0,
            self.y - self.half_y / 2.0,
            self.z + self.half_z / 2.0,
            self.w - self.half_w / 2.0,
            self.half_x / 2.0,
            self.half_y / 2.0,
            self.half_z / 2.0,
            self.half_w / 2.0,
        );
        let xanw = Self::new(
            self.x - self.half_x / 2.0,
            self.y + self.half_y / 2.0,
            self.z + self.half_z / 2.0,
            self.w - self.half_w / 2.0,
            self.half_x / 2.0,
            self.half_y / 2.0,
            self.half_z / 2.0,
            self.half_w / 2.0,
        );
        let xane = Self::new(
            self.x + self.half_x / 2.0,
            self.y + self.half_y / 2.0,
            self.z + self.half_z / 2.0,
            self.w - self.half_w / 2.0,
            self.half_x / 2.0,
            self.half_y / 2.0,
            self.half_z / 2.0,
            self.half_w / 2.0,
        );
        let ybsw = Self::new(
            self.x - self.half_x / 2.0,
            self.y - self.half_y / 2.0,
            self.z - self.half_z / 2.0,
            self.w + self.half_w / 2.0,
            self.half_x / 2.0,
            self.half_y / 2.0,
            self.half_z / 2.0,
            self.half_w / 2.0,
        );
        let ybse = Self::new(
            self.x + self.half_x / 2.0,
            self.y - self.half_y / 2.0,
            self.z - self.half_z / 2.0,
            self.w + self.half_w / 2.0,
            self.half_x / 2.0,
            self.half_y / 2.0,
            self.half_z / 2.0,
            self.half_w / 2.0,
        );
        let ybnw = Self::new(
            self.x - self.half_x / 2.0,
            self.y + self.half_y / 2.0,
            self.z - self.half_z / 2.0,
            self.w + self.half_w / 2.0,
            self.half_x / 2.0,
            self.half_y / 2.0,
            self.half_z / 2.0,
            self.half_w / 2.0,
        );
        let ybne = Self::new(
            self.x + self.half_x / 2.0,
            self.y + self.half_y / 2.0,
            self.z - self.half_z / 2.0,
            self.w + self.half_w / 2.0,
            self.half_x / 2.0,
            self.half_y / 2.0,
            self.half_z / 2.0,
            self.half_w / 2.0,
        );
        let yasw = Self::new(
            self.x - self.half_x / 2.0,
            self.y - self.half_y / 2.0,
            self.z + self.half_z / 2.0,
            self.w + self.half_w / 2.0,
            self.half_x / 2.0,
            self.half_y / 2.0,
            self.half_z / 2.0,
            self.half_w / 2.0,
        );
        let yase = Self::new(
            self.x + self.half_x / 2.0,
            self.y - self.half_y / 2.0,
            self.z + self.half_z / 2.0,
            self.w + self.half_w / 2.0,
            self.half_x / 2.0,
            self.half_y / 2.0,
            self.half_z / 2.0,
            self.half_w / 2.0,
        );
        let yanw = Self::new(
            self.x - self.half_x / 2.0,
            self.y + self.half_y / 2.0,
            self.z + self.half_z / 2.0,
            self.w + self.half_w / 2.0,
            self.half_x / 2.0,
            self.half_y / 2.0,
            self.half_z / 2.0,
            self.half_w / 2.0,
        );
        let yane = Self::new(
            self.x + self.half_x / 2.0,
            self.y + self.half_y / 2.0,
            self.z + self.half_z / 2.0,
            self.w + self.half_w / 2.0,
            self.half_x / 2.0,
            self.half_y / 2.0,
            self.half_z / 2.0,
            self.half_w / 2.0,
        );

        [
            xbsw, xbse, xbnw, xbne, xasw, xase, xanw, xane, ybsw, ybse, ybnw, ybne, yasw, yase,
            yanw, yane,
        ]
    }
}

///4-dimensional tree representation.
pub type TessTree<I> = Tree<I, Tess, TNode<I, Tess>>;

impl<I: Debug + Clone + Copy + Hash + PartialEq + Eq> TessTree<I> {
    pub fn new(translation: Tess, capacity: u8, max_depth: u8) -> Self {
        Tree::new_tree(TNode::new(translation, capacity, max_depth))
    }
}
