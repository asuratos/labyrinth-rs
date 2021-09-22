//! Module containing the Generator structs

use std::collections::HashMap;

use bracket_geometry::prelude::Point;

use super::errors::BuilderError;

use map_objects::{FinishedMap, TileType};

mod map_objects {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Copy, Serialize, Deserialize)]
    pub enum TileType {
        Floor,
        Wall,
        Door,
    }

    /// Output struct for
    #[derive(Serialize, Deserialize)]
    pub struct FinishedMap {
        pub map: Vec<TileType>,
        pub width: usize,
        pub height: usize,
        // pub entities: Option<HashMap<String, Point>>,
        // TODO: allow the builder to populate the map
    }
}

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
/// let mut mapgen = MapGenerator2D::new(10,10);
/// let map = mapgen.generate(FloorGenAlg::Basic);
/// ```
pub struct MapGenerator2D {
    map: Vec<TileType>,
    width: usize,
    height: usize,
}

impl MapGenerator2D {
    /// Creates a new Generator struct using width and height inputs
    pub fn new(width: usize, height: usize) -> MapGenerator2D {
        MapGenerator2D {
            map: vec![TileType::Floor; width * height],
            width: width,
            height: height,
        }
    }

    /// Generates a FinishedMap using the current settings.
    pub fn generate(&mut self, method: FloorGenAlg) -> Result<FinishedMap, BuilderError> {
        // Start with a new map
        self.flush_map();

        // Figure out the correct way to build the map
        match method {
            FloorGenAlg::Basic => {
                // generation function for this goes here
                // self.map = build_rooms_and_corridors
            }
            _ => {
                return Err(BuilderError::BuildError(format!(
                    "FloorGenAlg {:?} is unimplemented for this Generator",
                    method
                )))
            }
        };

        Ok(FinishedMap {
            map: self.map.to_owned(),
            width: self.width,
            height: self.height,
        })
    }

    fn flush_map(&mut self) {
        self.map = vec![TileType::Floor; self.width * self.height];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn working() {
        assert_eq!(1 + 1, 2);
    }
}
