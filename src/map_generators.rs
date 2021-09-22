//! Module containing the Generator structs

use std::collections::HashMap;

use bracket_geometry::prelude::*;
use bracket_pathfinding::prelude::*;

use super::errors::BuilderError;

use map_objects::Map;

mod map_objects {
    use super::{Algorithm2D, BaseMap, DistanceAlg, Point, SmallVec};
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Copy, PartialEq, Serialize, Deserialize)]
    pub enum TileKind {
        Floor,
        Wall,
        Door,
    }

    #[derive(Clone, Copy, PartialEq, Serialize, Deserialize)]
    pub struct Tile {
        // kind: TileKind,
        opaque: bool,
        enterable: bool,
        // flyable: bool
    }

    impl Default for Tile {
        fn default() -> Self {
            Tile {
                // kind: TileKind::Wall,
                opaque: true,
                enterable: false,
            }
        }
    }

    #[derive(Serialize, Deserialize, Clone)]
    pub struct Map {
        pub tiles: Vec<Tile>,
        pub dimensions: Point,
    }

    // Implementing Algorithm2D from bracket-pathfinding on map
    // This gives access to some useful helper methods using bracket-lib Points
    impl Algorithm2D for Map {
        fn dimensions(&self) -> Point {
            self.dimensions
        }
    }

    impl BaseMap for Map {
        fn get_available_exits(&self, _idx: usize) -> SmallVec<[(usize, f32); 10]> {
            let mut neighbors = SmallVec::new();

            let start = self.index_to_point2d(_idx);
            let deltas = &[
                Point::new(-1, 0),
                Point::new(0, -1),
                Point::new(1, 0),
                Point::new(0, 1),
            ];

            for diff in deltas {
                let neighbor = self.point2d_to_index(start + *diff);
                if self.tiles[neighbor].enterable {
                    neighbors.push((neighbor, 1.0));
                }
            }

            neighbors
        }

        fn get_pathing_distance(&self, _idx1: usize, _idx2: usize) -> f32 {
            DistanceAlg::Pythagoras
                .distance2d(self.index_to_point2d(_idx1), self.index_to_point2d(_idx2))
        }
    }

    impl Map {
        pub fn new(width: usize, height: usize) -> Map {
            Map {
                tiles: vec![Default::default(); width * height],
                dimensions: Point::new(width, height),
            }
        }

        pub fn new_from_dims(dimensions: Point) -> Map {
            Map {
                tiles: vec![Default::default(); (dimensions.x * dimensions.y) as usize],
                dimensions: dimensions,
            }
        }
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
/// let mut mapgen = MapGenerator2D::new(80,50);
/// let floor1 = mapgen.generate(FloorGenAlg::Basic);
/// let floor2 = mapgen.generate(FloorGenAlg::Basic);
/// let floor3 = mapgen.generate(FloorGenAlg::Basic);
/// ```
pub struct MapGenerator2D {
    map: Map,
    dimensions: Point,
}

impl MapGenerator2D {
    /// Creates a new Generator struct using width and height inputs
    pub fn new(width: usize, height: usize) -> MapGenerator2D {
        MapGenerator2D {
            map: Map::new(width, height),
            dimensions: Point::new(width, height),
        }
    }

    /// Generates a FinishedMap using the current settings.
    pub fn generate(&mut self, method: FloorGenAlg) -> Result<Map, BuilderError> {
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

        Ok(self.map.clone())
    }

    fn flush_map(&mut self) {
        self.map = Map::new_from_dims(self.dimensions);
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
