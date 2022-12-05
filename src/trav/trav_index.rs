



// pub trait TravIndex {

//     const DEPTH_MASK: u32;
//     const DIRECTION_MASK: u32;

//     fn depth(&self) -> u32;
//     fn depth_direction(&self) -> u32;
// }

use std::ops::Add;

#[derive(Debug, Clone)]
pub struct TravIndex<D> {
    depth: u8,
    direction: Vec<D>,
}

impl<D> TravIndex<D> {
    pub fn new() -> Self {
        Self {
            depth: 0,
            direction: Vec::new(),
        }
    }

    pub fn depth(&self) -> u8 {
        self.depth
    }

    pub fn direciton(&self) -> &Vec<D> {
        &self.direction
    }

    pub fn direciton_at_depth(&self) -> &D {
        &self.direction[self.depth as usize]
    }
}

impl<D : Clone> Add<D> for TravIndex<D> {
    type Output = Self;

    fn add(self, rhs: D) -> Self::Output {
        let mut direction = self.direction.clone();
        direction.push(rhs);
        Self {
            depth: self.depth + 1,
            direction
        }
    }
}