use crate::{Dimension, TNode, Tree, DEFAULT_CAPACITY, DEFAULT_MAX_DEPTH};

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

impl Dimension<4> for Quad {
    fn overlaps(&self, area: &Self) -> bool {
        !( (self.x - area.x).abs() > (self.half_x + area.half_x)
        || (self.y - area.y).abs() > (self.half_y + area.half_y))
    }
    fn contains_center(&self, point: &Self) -> bool {
        !( point.x >= self.x + self.half_x
        || point.y >= self.y + self.half_y
        || point.x <= self.x - self.half_x
        || point.y <= self.y - self.half_y)
    }
    fn subdivisions(&self) -> [Self; 4] {
        let half_x = self.half_x / 2.0;
        let half_y = self.half_y / 2.0;
        let west = self.x - half_x;
        let east = self.x + half_x;
        let south = self.y - half_y;
        let north = self.y + half_y;

        let sw = Self::new(
            west,
            south,
            half_x,
            half_y,
        );
        let se = Self::new(
            east,
            south,
            half_x,
            half_y,
        );
        let nw = Self::new(
            west,
            north,
            half_x,
            half_y,
        );
        let ne = Self::new(
            east,
            north,
            half_x,
            half_y,
        );

        [sw, se, nw, ne]
    }
}

///2-dimensional tree representation.
pub type QuadTree<I> = Tree<I, 4, Quad, TNode<I, 4, Quad>>;

impl<I: Clone> QuadTree<I> {
    pub fn new(translation: Quad) -> Self {
        Tree::new_tree(TNode::new(translation, DEFAULT_CAPACITY, DEFAULT_MAX_DEPTH))
    }
}

#[cfg(test)]
mod quadtree_tests {
    use super::*;

    #[test]
    fn quadtree() {
        let mut quadtree = QuadTree::new(Quad::new(0.0, 0.0, 500.0, 500.0));
        for x in -50..50 {
            for y in -50..50 {
                quadtree.insert(x * 1009 + y * 1013, Quad::new(x as f32 * 10.0 + 2.5, y as f32 * 10.0 + 2.5, 0.5, 0.5));
            }
        }
        let result = quadtree.search(&Quad::new(0.0, 0.0, 50.0, 50.0));
        assert_eq!(result.len(), 169);
    }
}