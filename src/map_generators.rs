//! Module containing the Generator structs

use std::collections::HashMap;

use bracket_geometry::prelude::*;
use bracket_pathfinding::prelude::*;

use super::errors::BuilderError;

pub use map_objects::{Map, Tile};

mod map_objects {
    use super::*;
    use serde::{Deserialize, Serialize};

    /// Tile struct that contains the tile type and its properties
    #[derive(Clone, PartialEq, Serialize, Deserialize)]
    pub struct Tile {
        pub kind: String,
        pub opaque: bool,
        pub walk: bool,
        pub fly: bool,
        pub swim: bool,
    }

    impl Default for Tile {
        fn default() -> Self {
            Tile {
                kind: "wall".to_string(),
                opaque: true,
                walk: false,
                fly: false,
                swim: false,
            }
        }
    }

    #[derive(Serialize, Deserialize, Clone)]
    pub struct Map {
        pub tiles: Vec<Tile>,
        dimensions: Point,
    }

    // Implementing Algorithm2D from bracket-pathfinding on map
    // This gives access to some useful helper methods using bracket-lib Points
    impl Algorithm2D for Map {
        fn dimensions(&self) -> Point {
            self.dimensions
        }
    }

    impl BaseMap for Map {
        fn is_opaque(&self, _idx: usize) -> bool {
            self.tiles[_idx].opaque
        }

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
                if self.tiles[neighbor].walk {
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
        // Constructors
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

        // Pathfinding functions
        /// Find the path from start to end by walking
        pub fn find_path_walk(&self, start: Point, end: Point) -> NavigationPath {
            a_star_search(
                self.point2d_to_index(start),
                self.point2d_to_index(end),
                self,
            )
        }

        /// Find the path from start to end by flying
        pub fn find_path_fly(&self, start: Point, end: Point) -> NavigationPath {
            self.find_path_alternate(start, end, "fly")
        }

        /// Find the path from start to end by swimming
        pub fn find_path_swim(&self, start: Point, end: Point) -> NavigationPath {
            self.find_path_alternate(start, end, "swim")
        }

        fn find_path_alternate(&self, start: Point, end: Point, move_type: &str) -> NavigationPath {
            // TODO: this should error on bad input
            if move_type == "walk" {
                return self.find_path_walk(start, end);
            }

            // first generate internal map for move type
            let internal_map = MapInternal::from_map(self, move_type);

            // then pathfind over it and return the path
            a_star_search(
                internal_map.point2d_to_index(start),
                internal_map.point2d_to_index(end),
                &internal_map,
            )
        }
    }

    // Internal Map struct to for pathfinding using alternate movement types.
    // When calling a pathfinding function for swim or fly on the Map struct,
    // it generates one of these and pathfinds over that.
    struct MapInternal {
        opaque: Vec<bool>,
        enterable: Vec<bool>,
        dimensions: Point,
    }

    impl MapInternal {
        fn from_map(map: &Map, move_type: &str) -> MapInternal {
            // TODO: error handling here
            let enterable: Result<Vec<bool>, &str> = map
                .tiles
                .iter()
                .map(|tile| match move_type {
                    "fly" => Ok(tile.fly),
                    "swim" => Ok(tile.swim),
                    _ => Err("Invalid movement type"),
                })
                .collect();

            let opaque: Vec<bool> = map.tiles.iter().map(|tile| tile.opaque).collect();

            MapInternal {
                opaque: opaque,
                enterable: enterable.unwrap(),
                dimensions: map.dimensions(),
            }
        }
    }

    impl Algorithm2D for MapInternal {
        fn dimensions(&self) -> Point {
            self.dimensions
        }
    }

    impl BaseMap for MapInternal {
        fn is_opaque(&self, _idx: usize) -> bool {
            self.opaque[_idx]
        }

        fn get_available_exits(&self, _idx: usize) -> SmallVec<[(usize, f32); 10]> {
            // TODO: figure out how to share implementation with the Map struct
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
                if self.enterable[neighbor] {
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
