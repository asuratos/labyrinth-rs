//! This is a crate for building 2d Roguelike-stype maps.
//! # Example Usage
//! ```rust
//! use labyrinth_rs::prelude::*;
//!
//!
//! ```
// TODO: Top level crate docs

mod errors;

mod map_generators;
mod map_objects;

pub mod prelude {
    //! Re-exported important objects (public API)
    pub use crate::map_generators::*;
    pub use crate::map_objects::*;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
