mod line;
mod quad;
mod oct;
mod tess;
mod tnode;
mod tree;

pub use crate::{line::*, quad::*, oct::*, tess::*, tnode::*, tree::*};

pub const DEFAULT_CAPACITY: u8 = 17;
pub const DEFAULT_MAX_DEPTH: u8 = 8;