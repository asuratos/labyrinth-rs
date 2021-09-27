//! Module for map objects

use std::collections::HashMap;

use bracket_pathfinding::prelude::*;
use serde::{Deserialize, Serialize};

mod tiles;
pub use tiles::*;

#[derive(PartialEq, Eq, Hash, Serialize, Deserialize, Clone)]
pub enum MoveType {
    Walk,
    Fly,
    Swim,
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
        self.find_path_alternate(start, end, vec![MoveType::Fly])
            .unwrap()
    }

    /// Find the path from start to end by swimming
    pub fn find_path_swim(&mut self, start: Point, end: Point) -> NavigationPath {
        self.find_path_alternate(start, end, vec![MoveType::Swim])
            .unwrap()
    }

    fn find_path_alternate(
        &mut self,
        start: Point,
        end: Point,
        move_types: Vec<MoveType>,
        // TODO: change this to use MoveProperties struct?
    ) -> Result<NavigationPath, String> {
        if move_types == vec![MoveType::Walk] {
            return Ok(self.find_path_walk(start, end));
        }

        let internal_map: &MapInternal;

        // Check if pathfinding over the movement type has been done before
        if !self.pathfinding_cache.contains_key(&move_types) {
            // if not, then add it to the cache
            self.pathfinding_cache.insert(
                move_types.clone(),
                MapInternal::from_map(self, &move_types)?,
            );
        }

        // then get the map from the cache
        internal_map = self.pathfinding_cache.get(&move_types).unwrap();

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
#[derive(Serialize, Deserialize, Clone)]
struct MapInternal {
    opaque: Vec<bool>,
    enterable: Vec<bool>,
    dimensions: Point,
}

impl MapInternal {
    fn from_map(map: &Map, move_types: &Vec<MoveType>) -> Result<MapInternal, String> {
        // TODO: what about things that can both fly and swim?
        //      This should be a bitwise OR over the movers movement options
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
