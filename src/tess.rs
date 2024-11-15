use crate::{Dimension, TNode, Tree, DEFAULT_CAPACITY, DEFAULT_MAX_DEPTH};

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

impl Dimension<16> for Tess {
    fn overlaps(&self, area: &Self) -> bool {
        !( (self.x - area.x).abs() > (self.half_x + area.half_x)
        || (self.y - area.y).abs() > (self.half_y + area.half_y)
        || (self.z - area.z).abs() > (self.half_z + area.half_z)
        || (self.w - area.w).abs() > (self.half_w + area.half_w))
    }
    fn contains_center(&self, point: &Self) -> bool {
        !( point.x >= self.x + self.half_x
        || point.y >= self.y + self.half_y
        || point.z >= self.z + self.half_z
        || point.w >= self.w + self.half_w
        || point.x <= self.x - self.half_x
        || point.y <= self.y - self.half_y
        || point.z <= self.z - self.half_z
        || point.w <= self.w - self.half_w)
    }
    fn subdivisions(&self) -> [Self; 16] {

        let half_x = self.half_x / 2.0;
        let half_y = self.half_y / 2.0;
        let half_z = self.half_z / 2.0;
        let half_w = self.half_w / 2.0;
        let west = self.x - half_x;
        let east = self.x + half_x;
        let south = self.y - half_y;
        let north = self.y + half_y;
        let down = self.z - half_z;
        let up = self.z + half_z;
        let back = self.w - half_w;
        let fore = self.w + half_w;

        let xbsw = Self::new(
            west,
            south,
            down,
            back,
            half_x,
            half_y,
            half_z,
            half_w,
        );
        let xbse = Self::new(
            east,
            south,
            down,
            back,
            half_x,
            half_y,
            half_z,
            half_w,
        );
        let xbnw = Self::new(
            west,
            north,
            down,
            back,
            half_x,
            half_y,
            half_z,
            half_w,
        );
        let xbne = Self::new(
            east,
            north,
            down,
            back,
            half_x,
            half_y,
            half_z,
            half_w,
        );
        let xasw = Self::new(
            west,
            south,
            up,
            back,
            half_x,
            half_y,
            half_z,
            half_w,
        );
        let xase = Self::new(
            east,
            south,
            up,
            back,
            half_x,
            half_y,
            half_z,
            half_w,
        );
        let xanw = Self::new(
            west,
            north,
            up,
            back,
            half_x,
            half_y,
            half_z,
            half_w,
        );
        let xane = Self::new(
            east,
            north,
            up,
            back,
            half_x,
            half_y,
            half_z,
            half_w,
        );
        let ybsw = Self::new(
            west,
            south,
            down,
            fore,
            half_x,
            half_y,
            half_z,
            half_w,
        );
        let ybse = Self::new(
            east,
            south,
            down,
            fore,
            half_x,
            half_y,
            half_z,
            half_w,
        );
        let ybnw = Self::new(
            west,
            north,
            down,
            fore,
            half_x,
            half_y,
            half_z,
            half_w,
        );
        let ybne = Self::new(
            east,
            north,
            down,
            fore,
            half_x,
            half_y,
            half_z,
            half_w,
        );
        let yasw = Self::new(
            west,
            south,
            up,
            fore,
            half_x,
            half_y,
            half_z,
            half_w,
        );
        let yase = Self::new(
            east,
            south,
            up,
            fore,
            half_x,
            half_y,
            half_z,
            half_w,
        );
        let yanw = Self::new(
            west,
            north,
            up,
            fore,
            half_x,
            half_y,
            half_z,
            half_w,
        );
        let yane = Self::new(
            east,
            north,
            up,
            fore,
            half_x,
            half_y,
            half_z,
            half_w,
        );

        [
            xbsw, xbse, xbnw, xbne, xasw, xase, xanw, xane, ybsw, ybse, ybnw, ybne, yasw, yase,
            yanw, yane,
        ]
    }
}

///4-dimensional tree representation.
pub type TessTree<I> = Tree<I, 16, Tess, TNode<I, 16, Tess>>;

impl<I: Clone> TessTree<I> {
    pub fn new(translation: Tess) -> Self {
        Tree::new_tree(TNode::new(translation, DEFAULT_CAPACITY, DEFAULT_MAX_DEPTH))
    }
}

#[cfg(test)]
mod tesstree_tests {
    use super::*;

    #[test]
    fn tesstree() {
        let mut tesstree = TessTree::new(Tess::new(0.0, 0.0, 0.0, 0.0, 500.0, 500.0, 500.0, 500.0));
        for x in -15..15 {
            for y in -15..15 {
                for z in -15..15 {
                    for w in -15..15 {
                        tesstree.insert(x * 1009 + y * 1013 + z * 1019 + w * 1021, Tess::new(x as f32 * 10.0 + 2.5, y as f32 * 10.0 + 2.5, z as f32 * 10.0 + 2.5, w as f32 * 10.0 + 2.5, 0.5, 0.5, 0.5, 0.5));
                    }
                }
            }
        }
        let result = tesstree.search(&Tess::new(0.0, 0.0, 0.0, 0.0, 50.0, 50.0, 50.0, 50.0));
        assert_eq!(result.len(), 28561);
    }
}