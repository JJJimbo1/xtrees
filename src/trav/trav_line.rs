use crate::{TravIndex, Line};




#[derive(Debug, Clone, Copy)]
pub enum TravLineDirection {
    Left,
    Right,
}

#[derive(Debug, Clone)]
pub struct TravLine {
    index: TravIndex<TravLineDirection>,
    line: Line,
}



// #[derive(Debug, Clone, Copy)]
// pub struct TravLineIndex(u32);

// impl TravIndex for TravLineIndex {
//     const DEPTH_MASK: u32 = 0b_0000_0000_0000_0000_0000_0000_0001_1111;
//     const DIRECTION_MASK: u32 = 0b_1111_1111_1111_1111_1111_1111_1110_0000;
//     fn depth(&self) -> u32 {
//         let depth_mask = 0b_0000_0000_0000_0000_0000_0000_0001_1111;
//         self.0 & depth_mask
//     }
//     fn depth_direction(&self) -> u32 {
        
//     }
// }