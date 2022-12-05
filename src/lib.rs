#![feature(generic_const_exprs)]
#![feature(iterator_try_collect)]
pub mod trav;
pub use trav::*;



pub mod tree;
pub mod tnode;
pub mod oct;
pub mod quad;
pub mod line;
pub mod tess;


pub use crate::{
    tree::*,
    tnode::*,
    line::*,
    quad::*,
    oct::*,
    tess::*,
};

pub const BASE_CAPACITY : u8 = 8;
pub const MAX_DEPTH : u8 = 8;

#[cfg(fearure = "hashbrown")]
pub use hashbrown::HashMap;

#[test]
fn atest() {
    use rand::random;
    use mathfu::D1;
    let mut qt = QuadTree::<usize>::new_tree(TNode::new(Quad::new(0.0, 0.0, 500.0, 500.0), 17, 12));
    for i in 0..1000 {
        // rand.range(-1000.0, 1000.0);
        // rand.range(-1000.0, 1000.0);
        let x = random();
        let y = random();
        qt.insert(i, Quad::new(x, y, 0.5, 0.5));
    }
    let v = qt.search_simple(&Quad::new(0.0, 0.0, 10000.0, 8000.0));
    println!("{}", v.len());
}

#[test]
fn btest() {
    let mut qt = OctTree::<usize>::new(Oct::new(0.0, 0.0, 0.0, 1000.0, 1000.0, 1000.0));
    let mut oct1 = Oct::new(0.0, 2.5, 2.5, 2.0, 2.0, 2.0);
    let mut oct2 = Oct::new(0.0, 2.5, 7.5, 2.0, 2.0, 2.0);
    let mut oct3 = Oct::new(0.0, 2.5, 12.5, 2.0, 2.0, 2.0);
    qt.insert(0, oct1);
    qt.insert(1, oct2);

    let len = qt.search_simple(&oct3);
    println!("{:?}", len);


}

#[test]
fn ctest() {
    let x = 0b_0000_0001;
    let y = 0b_1000_0000;
    println!("x:{}", x);
    println!("y:{}", y);
}