pub mod coordinate;
pub mod direction;
pub mod gridtools;
pub mod submit;

pub use coordinate::*;
pub use direction::*;
pub use gridtools::*;
pub use submit::*;

pub fn diff(c: u8, sq: Option<u8>) -> bool {
    sq.is_none_or(|x| x != c)
}
