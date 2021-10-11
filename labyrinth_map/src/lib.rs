//! TODO: Crate level documentation
//! 
//! 

#![warn(missing_docs)]

mod map_objects;

/// Prelude for re-exporting all important structs from the crate.
pub mod prelude {
    pub use super::map_objects::*;
}
