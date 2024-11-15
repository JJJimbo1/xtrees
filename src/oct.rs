use crate::{Dimension, TNode, Tree, DEFAULT_CAPACITY, DEFAULT_MAX_DEPTH};

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

impl Dimension<8> for Oct {
    #[inline]
    fn overlaps(&self, area: &Self) -> bool {
        !( (self.x - area.x).abs() > (self.half_x + area.half_x)
        || (self.y - area.y).abs() > (self.half_y + area.half_y)
        || (self.z - area.z).abs() > (self.half_z + area.half_z))
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
    fn subdivisions(&self) -> [Self; 8] {

        let half_x = self.half_x / 2.0;
        let half_y = self.half_y / 2.0;
        let half_z = self.half_z / 2.0;
        let west = self.x - half_x;
        let east = self.x + half_x;
        let south = self.y - half_y;
        let north = self.y + half_y;
        let down = self.z - half_z;
        let up = self.z + half_z;

        let bsw = Self::new(
            west,
            south,
            down,
            half_x,
            half_y,
            half_z,
        );
        let bse = Self::new(
            east,
            south,
            down,
            half_x,
            half_y,
            half_z,
        );
        let bnw = Self::new(
            west,
            north,
            down,
            half_x,
            half_y,
            half_z,
        );
        let bne = Self::new(
            east,
            north,
            down,
            half_x,
            half_y,
            half_z,
        );
        let asw = Self::new(
            west,
            south,
            up,
            half_x,
            half_y,
            half_z,
        );
        let ase = Self::new(
            east,
            south,
            up,
            half_x,
            half_y,
            half_z,
        );
        let anw = Self::new(
            west,
            north,
            up,
            half_x,
            half_y,
            half_z,
        );
        let ane = Self::new(
            east,
            north,
            up,
            half_x,
            half_y,
            half_z,
        );

        [bsw, bse, bnw, bne, asw, ase, anw, ane]
    }
}

///3-dimensional tree representation.
pub type OctTree<I> = Tree<I, 8, Oct, TNode<I, 8, Oct>>;

impl<I: Clone> OctTree<I> {
    pub fn new(translation: Oct) -> Self {
        Tree::new_tree(TNode::new(translation, DEFAULT_CAPACITY, DEFAULT_MAX_DEPTH))
    }
}

#[cfg(test)]
mod octtree_tests {
    use super::*;

    #[test]
    fn octtree() {
        let mut octtree = OctTree::new(Oct::new(0.0, 0.0, 0.0, 500.0, 500.0, 500.0));
        for x in -25..25 {
            for y in -25..25 {
                for z in -25..25 {
                    octtree.insert(x * 1009 + y * 1013 + z * 1019, Oct::new(x as f32 * 10.0 + 2.5, y as f32 * 10.0 + 2.5, z as f32 * 10.0 + 2.5, 0.5, 0.5, 0.5));
                }
            }
        }
        let result = octtree.search(&Oct::new(0.0, 0.0, 0.0, 50.0, 50.0, 50.0));
        assert_eq!(result.len(), 2197);
    }
}