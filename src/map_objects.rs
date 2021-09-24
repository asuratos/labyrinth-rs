//! Module for map objects

use std::collections::HashMap;

use bracket_pathfinding::prelude::*;
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
        Tile::wall()
    }
}

impl Tile {
    // Basic Tile constructors
    pub fn wall() -> Tile {
        Tile {
            kind: "wall".to_string(),
            opaque: true,
            walk: false,
            fly: false,
            swim: false,
        }
    }

    pub fn floor() -> Tile {
        Tile {
            kind: "floor".to_string(),
            opaque: false,
            walk: true,
            fly: true,
            swim: false,
        }
    }

    pub fn water() -> Tile {
        Tile {
            kind: "water".to_string(),
            opaque: false,
            walk: false,
            fly: true,
            swim: true,
        }
    }

    pub fn lava() -> Tile {
        Tile {
            kind: "lava".to_string(),
            opaque: false,
            walk: false,
            fly: true,
            swim: false,
        }
    }

    // property getters
    // TODO: figure out what kind of representation I want for tile properties
    pub fn move_properties(&self) -> (bool, bool, bool) {
        (self.walk, self.fly, self.swim)
    }
}

/// 2D Map struct, the output of the MapGenerator2D.
///
/// Implements Algorithm2D and BaseMap traits from bracket-pathfinding,
/// which allows for bracket-lib pathfinding algorithms.
/// Also comes with built-in implementations for pathfinding for alternate
/// movement methods (swim and fly).
/// ```rust
/// use labyrinth_rs::prelude::Map;
///
/// let map = Map::new(10,10);
/// ```
#[derive(Serialize, Deserialize, Clone)]
pub struct Map {
    pub tiles: Vec<Tile>,
    dimensions: Point,
    // TODO: If any changes happen to the map, the cache MUST be cleared.
    pathfinding_cache: HashMap<String, MapInternal>,
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
            pathfinding_cache: HashMap::new(),
        }
    }

    pub fn new_from_dims(dimensions: Point) -> Map {
        Map {
            tiles: vec![Default::default(); (dimensions.x * dimensions.y) as usize],
            dimensions: dimensions,
            pathfinding_cache: HashMap::new(),
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
    pub fn find_path_fly(&mut self, start: Point, end: Point) -> NavigationPath {
        self.find_path_alternate(start, end, "fly")
    }

    /// Find the path from start to end by swimming
    pub fn find_path_swim(&mut self, start: Point, end: Point) -> NavigationPath {
        self.find_path_alternate(start, end, "swim")
    }

    fn find_path_alternate(&mut self, start: Point, end: Point, move_type: &str) -> NavigationPath {
        // TODO: this should error on bad input
        if move_type == "walk" {
            return self.find_path_walk(start, end);
        }

        let internal_map: &MapInternal;
        // Check if pathfinding over the movement type has been done before

        if !self.pathfinding_cache.contains_key(move_type) {
            // if not, then add it to the cache
            self.pathfinding_cache.insert(
                move_type.to_string(),
                MapInternal::from_map(self, move_type),
            );
        }

        // then get the map from the cache
        internal_map = self.pathfinding_cache.get(move_type).unwrap();

        // then pathfind over it and return the path
        a_star_search(
            internal_map.point2d_to_index(start),
            internal_map.point2d_to_index(end),
            internal_map,
        )
    }
}

// Internal Map struct for pathfinding using alternate movement types.
// When calling a pathfinding function for swim or fly on the Map struct,
// it generates one of these and pathfinds over that.
// TODO: if multiple fliers have to pathfind, multiple projections are created.
//     This may not be a problem if we assume the maps will be small,
//     or enemies few.
#[derive(Serialize, Deserialize, Clone)]
struct MapInternal {
    opaque: Vec<bool>,
    enterable: Vec<bool>,
    dimensions: Point,
}

impl MapInternal {
    fn from_map(map: &Map, move_type: &str) -> MapInternal {
        // TODO: what about things that can both fly and swim?
        //      This should be a bitwise OR over the movers movement options
        let enterable: Result<Vec<bool>, &str> = map
            .tiles
            .iter()
            .map(|tile| match move_type {
                "fly" => Ok(tile.fly),
                "swim" => Ok(tile.swim),
                _ => Err("Invalid movement type"),
            })
            .collect();

        // TODO: error handling here

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

#[cfg(test)]
mod tests {
    use super::*;

    // Tile tests
}
