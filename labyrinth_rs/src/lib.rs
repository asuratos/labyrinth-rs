//! This is a crate for building 2d Roguelike-stype maps.
//! # Example Usage
//! ```rust
//! use labyrinth_rs::prelude::*;
//!
//!
//! ```
// TODO: Top level crate docs
#![warn(missing_docs)]

use labyrinth_map;

mod errors;

mod map_generators;

pub mod prelude {
    //! Re-exported important objects (public API)
    pub use crate::map_generators::*;
    pub use labyrinth_map::prelude::*;
}

pub mod labyrinth {
    pub use labyrinth_map::prelude::*;
}

#[cfg(test)]
mod tests {}
