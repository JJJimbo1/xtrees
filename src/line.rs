use crate::{Dimension, TNode, Tree, DEFAULT_CAPACITY, DEFAULT_MAX_DEPTH};

///1-dimensional line.
#[derive(Debug, Clone, Copy)]
pub struct Line {
    pub x: f32,
    pub half_x: f32,
}

impl Line {
    pub fn new(x: f32, half_x: f32) -> Self {
        Self { x, half_x }
    }
}

impl Dimension<2> for Line {
    fn overlaps(&self, area: &Self) -> bool {
        !( (self.x - area.x).abs() > (self.half_x + area.half_x))
    }
    fn contains_center(&self, point: &Self) -> bool {
        !( point.x >= self.x + self.half_x
        || point.x <= self.x - self.half_x)
    }
    fn subdivisions(&self) -> [Self; 2] {
        let half_x = self.half_x / 2.0;
        let west = self.x - half_x;
        let east = self.x + half_x;
        let w = Self::new(
            west,
            half_x,
        );
        let e = Self::new(
            east,
            half_x,
        );

        [w, e]
    }
}

///1-dimensional tree representation.
pub type LineTree<I> = Tree<I, 2, Line, TNode<I, 2, Line>>;

impl<I: Clone> LineTree<I> {
    pub fn new(translation: Line) -> Self {
        Tree::new_tree(TNode::new(translation, DEFAULT_CAPACITY, DEFAULT_MAX_DEPTH))
    }
}

#[cfg(test)]
mod linetree_tests {
    use super::*;

    #[test]
    fn linetree() {
        let mut linetree = LineTree::new(Line::new(0.0, 500.0));
        for x in -50..50 {
            linetree.insert(x * 1009, Line::new(x as f32 * 10.0 + 2.5, 0.5));
        }
        let result = linetree.search(&Line::new(0.0, 50.0));
        assert_eq!(result.len(), 25);
    }
}