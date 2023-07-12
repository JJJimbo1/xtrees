pub use tree::*;

mod tree {
    use std::{hash::Hash, marker::PhantomData};
    use crate::TNode;

    pub trait Dimension : Sized + Copy {
        const SUB_COUNT : usize;
        // type Other;
        ///Should return true when the dimensions overlap.
        fn overlaps(&self, other : &Self) -> bool;
        ///Should return true when the dimension contains the center of the other dim.
        fn contains_center(&self, point : &Self) -> bool;
        ///Should return a vector of the dims subdivisions filled with every permutation of the dim's halfway points and half sizes, which is 2^dims.
        fn subdivisions(&self) -> [Self; Self::SUB_COUNT];
    }

    pub trait Node<I : Copy, D : Dimension> {
        ///Insert object into the tree. Should return tree if the object was not previously inserted.
        fn insert(&mut self, id : I, other : D) -> bool;
        ///Subdivide the tree.
        fn subdivide(&mut self);
        ///Search the tree.
        fn search_simple(&self, area : &D, buffer : &mut Vec<(I, D)>);
        ///Search the tree with custom overlap logic.
        fn search_custom<OF>(&self, overlaps : &OF, buffer : &mut Vec<(I, D)>) where OF : Fn(&D) -> bool;
        ///Clear the tree.
        fn clear(&mut self);
    }

    #[derive(Debug, Clone)]
    pub struct Tree<I : Hash + PartialEq + Eq + Copy, D : Dimension, N : Node<I, D> = TNode<I, D>> {
        prime : N,
        count : usize,
        id : PhantomData<I>,
        dim : PhantomData<D>,
    }

    impl<I : Hash + PartialEq + Eq + Copy, D : Dimension, N : Node<I, D>> Tree<I, D, N> {
        pub fn new_tree(prime : N) -> Self {
            Self {
                prime,
                count : 0,
                id : PhantomData,
                dim : PhantomData,
            }
        }
        pub fn insert(&mut self, id : I, item : D) -> bool {
            if self.prime.insert(id, item) { self.count += 1; return true; }
            return false;
        }
        pub fn search_simple(&self, area : &D) -> Vec<(I, D)> {
            let mut buffer = Vec::with_capacity(self.count);
            self.prime.search_simple(area, &mut buffer);
            buffer.into_iter().collect()
        }
        pub fn search_custom<OF>(&self, overlaps : &OF) -> Vec<(I, D)>
        where OF : Fn(&D) -> bool {
            let mut buffer = Vec::with_capacity(self.count);
            self.prime.search_custom(overlaps, &mut buffer);
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