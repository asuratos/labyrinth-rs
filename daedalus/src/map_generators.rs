//! Module containing the Generator structs

// use std::collections::HashMap;

use bracket_geometry::prelude::*;
// use bracket_pathfinding::prelude::*;

use crate::genalgs;

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
/// use daedalus::prelude::*;
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
    map: Labyrinth2D,
    rooms: Vec<Box<dyn genalgs::rooms::Room>>,
    dimensions: Point,
}

impl MapGenerator2D {
    // ------------------ Initialization Methods ----------------------
    /// Creates a new Generator struct using width and height inputs
    pub fn new(width: usize, height: usize) -> MapGenerator2D {
        MapGenerator2D {
            map: Labyrinth2D::new(width, height),
            rooms: vec![],
            dimensions: Point::new(width, height),
        }
    }

    // ----------------- Access Methods ---------------------
    /// Retrieves a reference to the internal [`Labyrinth2D`] of the Generator
    pub fn map(&self) -> &Labyrinth2D {
        &self.map
    }

    // ----------------- Generation Methods -------------------------
    /// Generates a FinishedMap using the current settings.
    pub fn generate(&mut self, method: FloorGenAlg) -> Result<Labyrinth2D, BuilderError> {
        // Start with a new map
        self.flush_map();

        // Figure out the correct way to build the map
        match method {
            FloorGenAlg::Basic => {
                genalgs::build_rooms_and_corridors(self);
            }
            _ => {
                return Err(BuilderError::BuildError(format!(
                    "FloorGenAlg {:?} is unimplemented for this Generator",
                    method
                )))
            }
        };

        Ok(self.map.clone())
    }

    /// Resets the internal [`Labyrinth2D`] to a complely filled-in map
    pub fn flush_map(&mut self) {
        self.map = Labyrinth2D::new_from_dims(self.dimensions);
    }

    /// Resets the internal [`Labyrinth2D`] to an open map with walls
    pub fn walled_map(&mut self) {
        self.map = Labyrinth2D::new_walled_from_dims(self.dimensions);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
