//! Module containing the Generator structs

// use std::collections::HashMap;

use bracket_geometry::prelude::*;
// use bracket_pathfinding::prelude::*;

use super::errors::BuilderError;
use labyrinth_map::prelude::*;

#[derive(Debug)]
pub enum FloorGenAlg {
    Basic, // Rooms and Corridors
}

/// Builder struct for 2D Maps
///
/// # Example Usage
/// ```rust
/// use labyrinth_rs::prelude::*;
///
/// let mut mapgen = MapGenerator2D::new(80,50);
/// let floor1 = mapgen.generate(FloorGenAlg::Basic);
/// assert!(floor1.is_ok());
///
/// let floor2 = mapgen.generate(FloorGenAlg::Basic);
/// assert!(floor2.is_ok());
///
/// let floor3 = mapgen.generate(FloorGenAlg::Basic);
/// assert!(floor3.is_ok());
/// ```
pub struct MapGenerator2D {
    map: Labyrinth,
    dimensions: Point,
}

impl MapGenerator2D {
    /// Creates a new Generator struct using width and height inputs
    pub fn new(width: usize, height: usize) -> MapGenerator2D {
        MapGenerator2D {
            map: Labyrinth::new(width, height),
            dimensions: Point::new(width, height),
        }
    }

    /// Generates a FinishedMap using the current settings.
    pub fn generate(&mut self, method: FloorGenAlg) -> Result<Labyrinth, BuilderError> {
        // Start with a new map
        self.flush_map();

        // Figure out the correct way to build the map
        match method {
            FloorGenAlg::Basic => {
                // generation function for this goes here
                // self.map = build_rooms_and_corridors
            } // _ => {
              //     return Err(BuilderError::BuildError(format!(
              //         "FloorGenAlg {:?} is unimplemented for this Generator",
              //         method
              //     )))
              // }
        };

        Ok(self.map.clone())
    }

    fn flush_map(&mut self) {
        self.map = Labyrinth::new_from_dims(self.dimensions);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
