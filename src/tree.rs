pub use tree::*;

mod tree {
    use crate::TNode;
    use std::marker::PhantomData;

    pub trait Dimension<const S: usize>: Sized {
        ///Should return true when the dimensions overlap.
        fn overlaps(&self, other: &Self) -> bool;
        ///Should return true when the dimension contains the center of the other dim.
        fn contains_center(&self, point: &Self) -> bool;
        ///Should return a vector of the dims subdivisions filled with every permutation of the dim's halfway points and half sizes, which is 2^dims.
        fn subdivisions(&self) -> [Self; S];
    }

    pub trait Node<I, const S: usize, D: Dimension<S>> {
        ///Insert object into the tree. Should return tree if the object was actually inserted (was not already present).
        fn insert(&mut self, id: I, other: D) -> bool;
        ///Subdivide the tree.
        fn subdivide(&mut self);
        ///Search the tree using the same shape that makes up the tree nodes.
        fn search(&self, area: &D, buffer: &mut Vec<(I, D)>);
        ///Search the tree with custom overlap logic.
        fn search_with<OF>(&self, overlaps: &OF, buffer: &mut Vec<(I, D)>)
        where OF: Fn(&D) -> bool;
        ///Clear the tree.
        fn clear(&mut self);
    }

    #[derive(Debug, Clone)]
    pub struct Tree<I, const S: usize, D: Dimension<S>, N: Node<I, S, D> = TNode<I, S, D>> {
        prime: N,
        count: usize,
        i: PhantomData<I>,
        dim: PhantomData<D>,
    }

    impl<I, const S: usize, D: Dimension<S>, N: Node<I, S, D>> Tree<I, S, D, N> {
        pub fn new_tree(prime: N) -> Self {
            Self {
                prime,
                count: 0,
                i: PhantomData,
                dim: PhantomData,
            }
        }
        pub fn insert(&mut self, id: I, item: D) -> bool {
            if self.prime.insert(id, item) {
                self.count += 1;
                return true;
            }
            return false;
        }
        pub fn search(&self, area: &D) -> Vec<(I, D)> {
            let mut buffer = Vec::with_capacity(self.count);
            self.prime.search(area, &mut buffer);
            buffer.into_iter().collect()
        }
        pub fn search_custom<OF>(&self, overlaps: &OF) -> Vec<(I, D)>
        where
            OF: Fn(&D) -> bool,
        {
            let mut buffer = Vec::with_capacity(self.count);
            self.prime.search_with(overlaps, &mut buffer);
            buffer.into_iter().collect()
        }
        pub fn clear(&mut self) {
            self.prime.clear();
        }
        pub fn prime(&self) -> &N {
            &self.prime
        }
        pub fn count(&self) -> usize {
            self.count
        }
    }
}
