//! Module for map objects

use std::collections::HashMap;

use bracket_pathfinding::prelude::*;
use serde::{Deserialize, Serialize};

// Helper struct for the movement properties of a tile
// Has boolean values for movement types, as well as fields for adding
// custom movement types
#[derive(Debug, PartialEq)]
pub(crate) struct MoveProperties {
    walk: bool,
    fly: bool,
    swim: bool,
    other: HashMap<String, bool>,
}

#[derive(PartialEq, Eq, Hash, Serialize, Deserialize, Clone)]
pub enum MoveType {
    Walk,
    Fly,
    Swim,
    Custom(String),
}
// Builder struct for tiles
// will fail if required fields (everything except custom_properties) is None
pub struct TileBuilder {
    kind: Option<String>,
    opaque: Option<bool>,
    walk: Option<bool>,
    fly: Option<bool>,
    swim: Option<bool>,
    custom_properties: HashMap<String, bool>,
}

impl TileBuilder {
    pub fn new() -> TileBuilder {
        TileBuilder {
            kind: None,
            opaque: None,
            walk: None,
            fly: None,
            swim: None,
            custom_properties: HashMap::new(),
        }
    }

    pub fn wall() -> TileBuilder {
        TileBuilder::new()
            .kind("wall")
            .opaque(true)
            .walk(false)
            .fly(false)
            .swim(false)
    }

    pub fn floor() -> TileBuilder {
        TileBuilder::new()
            .kind("floor")
            .opaque(false)
            .walk(true)
            .fly(true)
            .swim(true)
    }

    pub fn water() -> TileBuilder {
        TileBuilder::new()
            .kind("water")
            .opaque(false)
            .walk(false)
            .fly(true)
            .swim(true)
    }

    pub fn lava() -> TileBuilder {
        TileBuilder::new()
            .kind("lava")
            .opaque(false)
            .walk(false)
            .fly(true)
            .swim(false)
    }

    pub fn kind(mut self, kind: &str) -> TileBuilder {
        let kind = kind.to_lowercase();
        self.kind = Some(kind);

        self
    }

    pub fn opaque(mut self, value: bool) -> TileBuilder {
        self.opaque = Some(value);
        self
    }

    pub fn walk(mut self, value: bool) -> TileBuilder {
        self.walk = Some(value);
        self
    }

    pub fn fly(mut self, value: bool) -> TileBuilder {
        self.fly = Some(value);
        self
    }

    pub fn swim(mut self, value: bool) -> TileBuilder {
        self.swim = Some(value);
        self
    }

    pub fn property(mut self, prop: &str, value: bool) -> TileBuilder {
        let prop = prop.to_lowercase();
        let lcase_prop = prop.as_str();

        self.custom_properties
            .entry(lcase_prop.to_string())
            .or_insert(value);

        self
    }

    pub fn is_fully_initialized(&self) -> bool {
        self.kind.is_some()
            && self.opaque.is_some()
            && self.walk.is_some()
            && self.fly.is_some()
            && self.swim.is_some()
    }

    pub fn build(self) -> Result<Tile, String> {
        if !self.is_fully_initialized() {
            Err("Builder is not fully initialized!".to_string())
        } else {
            Ok(Tile::default())
        }
    }
}

/// Tile struct that contains the tile type and its properties
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Tile {
    pub kind: String,
    pub opaque: bool,
    pub walk: bool,
    pub fly: bool,
    pub swim: bool,
    pub other_movement: HashMap<String, bool>,
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
            other_movement: HashMap::new(),
        }
    }

    pub fn floor() -> Tile {
        Tile {
            kind: "floor".to_string(),
            opaque: false,
            walk: true,
            fly: true,
            swim: false,
            other_movement: HashMap::new(),
        }
    }

    pub fn water() -> Tile {
        Tile {
            kind: "water".to_string(),
            opaque: false,
            walk: false,
            fly: true,
            swim: true,
            other_movement: HashMap::new(),
        }
    }

    pub fn lava() -> Tile {
        Tile {
            kind: "lava".to_string(),
            opaque: false,
            walk: false,
            fly: true,
            swim: false,
            other_movement: HashMap::new(),
        }
    }

    // this determines if an entity with a given set of movement types can
    // enter this tile.
    pub fn can_enter(&self, move_types: &Vec<MoveType>) -> Result<bool, String> {
        // TODO: This should return a usr-facing error
        move_types
            .iter()
            .map(|move_type| match move_type {
                MoveType::Walk => Ok(self.walk),
                MoveType::Fly => Ok(self.fly),
                MoveType::Swim => Ok(self.swim),
                MoveType::Custom(custom) => self.other_movement.get(custom).copied().ok_or(
                    format!("Movement type {} does not exist for this tile", custom),
                ),
            }) // Vec<Result<bool, String>>
            .collect::<Result<Vec<bool>, String>>()
            .map(|resvec| resvec.iter().any(|res| *res))
    }

    // property getters
    pub(crate) fn move_properties(&self) -> MoveProperties {
        MoveProperties {
            walk: self.walk,
            fly: self.fly,
            swim: self.swim,
            other: self.other_movement.clone(),
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    // Tile tests
    #[test]
    fn tile_creation_from_wall_template() {
        let builder = TileBuilder::wall();
        let tile = builder.build().unwrap();

        // assert that it's a wall
        assert_eq!(tile.opaque, true);
        assert_eq!(
            tile.move_properties(),
            MoveProperties {
                walk: false,
                fly: false,
                swim: false,
                other: HashMap::new(),
            }
        );
    }

    #[test]
    #[should_panic]
    fn unfinished_builder_should_panic() {
        let builder = TileBuilder::new();
        builder.build().unwrap();
    }

    #[test]
    fn tiles_with_diff_kind_can_still_have_same_properties() -> Result<(), String> {
        let custom_tile = TileBuilder::wall().kind("smoothwall").build()?;

        let wall = Tile::wall();

        assert_eq!(wall.move_properties(), custom_tile.move_properties());
        Ok(())
    }
}
