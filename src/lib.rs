//! This is a crate for building 2d Roguelike-stype maps.
//! # Example Usage

mod errors;

mod map_generators;

pub mod prelude {
    //! Re-exported important objects (public API)
    pub use crate::map_generators::*;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
