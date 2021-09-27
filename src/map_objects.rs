//! Module for map objects

use std::collections::HashMap;

use bracket_pathfinding::prelude::*;
use serde::{Deserialize, Serialize};

mod tiles;
pub use tiles::*;

/// Enum defining possible movement methods
#[derive(PartialEq, Eq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize, Clone)]
pub enum MoveType {
    /// variant for walking
    Walk,

    /// variant for flying
    Fly,

    /// variant for swimming
    Swim,

    /// variant for a user-defined movement type
    Custom(String),
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
    /// The vector of tiles in the map.
    pub tiles: Vec<Tile>,
    dimensions: Point,
    // TODO: If any changes happen to the map, the cache MUST be cleared.
    pathfinding_cache: HashMap<Vec<MoveType>, MapInternal>,
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
    /// Constructs a new map with the passed width and height values.
    ///
    /// Initial Tiles are all walls.
    pub fn new(width: usize, height: usize) -> Map {
        Map {
            tiles: vec![Default::default(); width * height],
            dimensions: Point::new(width, height),
            pathfinding_cache: HashMap::new(),
        }
    }

    /// Constructs a new map with a size defined by a 2d [`Point`].
    ///
    /// Initial Tiles are all walls.
    pub fn new_from_dims(dimensions: Point) -> Map {
        Map {
            tiles: vec![Default::default(); (dimensions.x * dimensions.y) as usize],
            dimensions: dimensions,
            pathfinding_cache: HashMap::new(),
        }
    }

    // Pathfinding functions
    /// Find the path between two [`Points`](Point) by walking
    pub fn find_path_walk(&self, start: Point, end: Point) -> NavigationPath {
        a_star_search(
            self.point2d_to_index(start),
            self.point2d_to_index(end),
            self,
        )
    }

    /// Find the path between two [`Points`](Point) by flying
    pub fn find_path_fly(&mut self, start: Point, end: Point) -> NavigationPath {
        self.find_path(start, end, &[MoveType::Fly]).unwrap()
    }

    /// Find the path between two [`Points`](Point) by swimming
    pub fn find_path_swim(&mut self, start: Point, end: Point) -> NavigationPath {
        self.find_path(start, end, &[MoveType::Swim]).unwrap()
    }

    /// Find the path between two [`Points`](Point) for an entity with multiple
    /// movement types.
    pub fn find_path(
        &mut self,
        start: Point,
        end: Point,
        move_types: &[MoveType],
    ) -> Result<NavigationPath, String> {
        if move_types == &[MoveType::Walk] {
            return Ok(self.find_path_walk(start, end));
        }

        let internal_map: &MapInternal;

        // Check if pathfinding over the movement type has been done before
        let mut move_types_vec = move_types.to_vec();
        move_types_vec.sort();

        if !self.pathfinding_cache.contains_key(&move_types_vec) {
            // if not, then add it to the cache

            let projection = MapInternal::from_map(self, move_types_vec.as_slice())?;
            self.pathfinding_cache
                .insert(move_types_vec.clone(), projection);
        }

        // then get the map from the cache
        internal_map = self.pathfinding_cache.get(&move_types_vec).unwrap();

        // then pathfind over it and return the path
        Ok(a_star_search(
            internal_map.point2d_to_index(start),
            internal_map.point2d_to_index(end),
            internal_map,
        ))
    }
}

// Internal Map struct for pathfinding using alternate movement types.
// When calling a pathfinding function for swim or fly on the Map struct,
// it generates one of these and pathfinds over that.
#[derive(Serialize, Deserialize, Clone, Debug)]
struct MapInternal {
    opaque: Vec<bool>,
    enterable: Vec<bool>,
    dimensions: Point,
}

impl MapInternal {
    fn from_map(map: &Map, move_types: &[MoveType]) -> Result<MapInternal, String> {
        let enterable = map
            .tiles
            .iter()
            .map(|tile| tile.can_enter(move_types))
            .collect::<Result<Vec<bool>, String>>()?;

        let opaque: Vec<bool> = map.tiles.iter().map(|tile| tile.opaque).collect();

        Ok(MapInternal {
            opaque: opaque,
            enterable: enterable,
            dimensions: map.dimensions(),
        })
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

    #[test]
    fn pathfinding_adds_to_map_cache() {
        let mut map = Map::new(10, 10);

        let start = Point::new(1, 1);
        let end = Point::new(5, 5);

        let _path = map.find_path_swim(start, end);

        assert_eq!(map.pathfinding_cache.len(), 1);
    }

    #[test]
    fn pathfinding_twice_doesnt_add_to_map_cache() {
        let mut map = Map::new(10, 10);

        let start = Point::new(1, 1);
        let end = Point::new(5, 5);

        let _path1 = map.find_path_swim(start, end);
        let _path2 = map.find_path_swim(start, end);

        assert_eq!(map.pathfinding_cache.len(), 1);
    }

    #[test]
    fn map_cache_entries_are_order_insensitive() {
        let mut map = Map::new(10, 10);

        let start = Point::new(1, 1);
        let end = Point::new(5, 5);

        let movetype1 = &[MoveType::Walk, MoveType::Fly];
        let movetype2 = &[MoveType::Fly, MoveType::Walk];

        let _path1 = map.find_path(start, end, movetype1);
        let _path2 = map.find_path(start, end, movetype2);

        // The second one should not be added to the cache (since they're the same)
        assert_eq!(map.pathfinding_cache.len(), 1);
    }
}
