//! Module for map objects

use std::collections::HashMap;

use bracket_pathfinding::prelude::*;
use serde::{Deserialize, Serialize};

mod tiles;
pub use tiles::*;

// TODO: Better Map struct documentation
/// Labyrinth2D struct, the output of the MapGenerator2D.
///
/// Implements Algorithm2D and BaseMap traits from bracket-pathfinding,
/// which allows for bracket-lib pathfinding algorithms.
/// Also comes with built-in implementations for pathfinding for alternate
/// movement methods (swim and fly).
/// ```rust
/// use labyrinth_map::prelude::*;
///
/// let map = Labyrinth2D::new(10,10);
/// ```
#[derive(Serialize, Deserialize, Clone)]
pub struct Labyrinth2D {
    /// The vector of tiles in the map.
    tiles: Vec<Tile>,
    dimensions: Point,
    pathfinding_cache: HashMap<Vec<MoveType>, InternalLabyrinth2D>,
}

// Implementing Algorithm2D from bracket-pathfinding on Labyrinth2D
// This gives access to some useful helper methods using bracket-lib Points
impl Algorithm2D for Labyrinth2D {
    fn dimensions(&self) -> Point {
        self.dimensions
    }
}

impl BaseMap for Labyrinth2D {
    fn is_opaque(&self, _idx: usize) -> bool {
        self.tiles[_idx].opaque
    }

    fn get_available_exits(&self, _idx: usize) -> SmallVec<[(usize, f32); 10]> {
        let start = self.index_to_point2d(_idx);
        let deltas = [
            Point::new(-1, 0),
            Point::new(0, -1),
            Point::new(1, 0),
            Point::new(0, 1),
        ];

        deltas
            .iter()
            // apply each delta to the point
            .map(|&diff| start + diff)
            // filter to only points in map bounds
            .filter(|&pt| self.in_bounds(pt))
            // map points -> vector indices
            .map(|pt| self.point2d_to_index(pt))
            // filter to only tiles that are walkable
            .filter(|&pos| self.tiles[pos].can_enter(&[MoveType::Walk]))
            // package into final struct
            .map(|pos| (pos, 1.0))
            // finally, collect into the final SmallVec
            .collect::<SmallVec<[(_, _); 10]>>()
    }

    fn get_pathing_distance(&self, _idx1: usize, _idx2: usize) -> f32 {
        DistanceAlg::Pythagoras
            .distance2d(self.index_to_point2d(_idx1), self.index_to_point2d(_idx2))
    }
}

impl Labyrinth2D {
    // ------------------ Constructors ---------------------------
    /// Constructs a new Labyrinth with the passed width and height values.
    ///
    /// Initial Tiles are all walls.
    pub fn new(width: usize, height: usize) -> Labyrinth2D {
        Labyrinth2D {
            tiles: vec![Default::default(); width * height],
            dimensions: Point::new(width, height),
            pathfinding_cache: HashMap::new(),
        }
    }

    /// Constructs a new Labyrinth with the passed width and height values.
    ///
    /// Initial Tiles are all floors.
    pub fn new_empty(width: usize, height: usize) -> Labyrinth2D {
        Labyrinth2D {
            tiles: vec![Tile::floor(); width * height],
            dimensions: Point::new(width, height),
            pathfinding_cache: HashMap::new(),
        }
    }

    /// Constructs a new Labyrinth with a size defined by a 2d [`Point`].
    ///
    /// Initial Tiles are all walls.
    pub fn new_from_dims(dimensions: Point) -> Labyrinth2D {
        Labyrinth2D {
            tiles: vec![Default::default(); (dimensions.x * dimensions.y) as usize],
            dimensions,
            pathfinding_cache: HashMap::new(),
        }
    }

    // -------------------- Pathfinding functions -----------------
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
        self.find_path(start, end, &[MoveType::Fly])
    }

    /// Find the path between two [`Points`](Point) by swimming
    pub fn find_path_swim(&mut self, start: Point, end: Point) -> NavigationPath {
        self.find_path(start, end, &[MoveType::Swim])
    }

    fn get_from_cache_or_add(&mut self, move_types: &[MoveType]) -> &InternalLabyrinth2D {
        // Check if pathfinding over the movement type has been done before
        let mut move_types_vec = move_types.to_vec();
        move_types_vec.sort();

        if !self.pathfinding_cache.contains_key(&move_types_vec) {
            // if not, then add it to the cache

            let projection = InternalLabyrinth2D::from_map(self, move_types_vec.as_slice());
            self.pathfinding_cache
                .insert(move_types_vec.clone(), projection);
        }

        // then get the map from the cache
        self.pathfinding_cache.get(&move_types_vec).unwrap()
    }

    /// Find the path between two [`Points`](Point) for an entity with multiple
    /// movement types.
    // TODO: Examples here
    pub fn find_path(
        &mut self,
        start: Point,
        end: Point,
        move_types: &[MoveType],
    ) -> NavigationPath {
        // If the movetype is only walk, then pathfinding can be done on
        // the Map as-is
        if move_types == [MoveType::Walk] {
            return self.find_path_walk(start, end);
        }

        // Get the map from the cache if it exists, add it otherwise
        let internal_map = self.get_from_cache_or_add(move_types);

        // then pathfind over it and return the path
        a_star_search(
            internal_map.point2d_to_index(start),
            internal_map.point2d_to_index(end),
            internal_map,
        )
    }

    /// Returns Dijkstra map for a set of starting [`Points`](Point), given
    /// the movement types of the entity.
    // TODO: Examples here
    pub fn dijkstra_map(&mut self, starts: &[Point], move_types: &[MoveType]) -> DijkstraMap {
        // if the MoveType is only walk, then it can be done on the map itself
        if move_types == [MoveType::Walk] {
            return self.dijkstra_map_walk(starts);
        }

        let Point {
            x: size_x,
            y: size_y,
        } = self.dimensions;

        let starts_idx: Vec<usize> = starts.iter().map(|&pt| self.point2d_to_index(pt)).collect();

        // Get the map from the cache if it exists, add it otherwise
        let internal_map = self.get_from_cache_or_add(move_types);

        // Finally, return the Dijkstra map over that internal projection
        DijkstraMap::new(size_x, size_y, &starts_idx, internal_map, 1024.0)
    }

    /// Constructs the Dijkstra map for an entity that can only walk
    pub fn dijkstra_map_walk(&self, starts: &[Point]) -> DijkstraMap {
        let Point {
            x: size_x,
            y: size_y,
        } = self.dimensions;

        let starts_idx: Vec<usize> = starts.iter().map(|&pt| self.point2d_to_index(pt)).collect();

        DijkstraMap::new(size_x, size_y, &starts_idx, self, 1024.0)
    }

    /// Constructs the Dijkstra map for an entity that can only fly
    pub fn dijkstra_map_fly(&mut self, starts: &[Point]) -> DijkstraMap {
        self.dijkstra_map(starts, &[MoveType::Fly])
    }

    /// Constructs the Dijkstra map for an entity that can only fly
    pub fn dijkstra_map_swim(&mut self, starts: &[Point]) -> DijkstraMap {
        self.dijkstra_map(starts, &[MoveType::Swim])
    }

    // ---------------- Map editing methods --------------
    /// Sets the tile at the given [`Point`](Point) to a [`Tile`].
    pub fn set_tile_at(&mut self, loc: Point, tile: Tile) {
        let idx = self.point2d_to_index(loc);
        self.tiles[idx] = tile;
        self.pathfinding_cache.clear();
    }

    /// Sets the tile at the given [`Point`](Point) to a basic floor.
    pub fn set_floor(&mut self, loc: Point) {
        self.set_tile_at(loc, Tile::floor());
    }

    /// Sets the tile at the given [`Point`](Point) to a basic wall.
    pub fn set_wall(&mut self, loc: Point) {
        self.set_tile_at(loc, Tile::wall());
    }

    /// Sets the tile at the given [`Point`](Point) to a basic water tile.
    pub fn set_water(&mut self, loc: Point) {
        self.set_tile_at(loc, Tile::water());
    }

    /// Sets the tile at the given [`Point`](Point) to a basic lava tile.
    pub fn set_lava(&mut self, loc: Point) {
        self.set_tile_at(loc, Tile::lava());
    }

    /// Adds a specified movetype to every [`Tile`] in the entire [`Labyrinth2D`]
    pub fn add_movetype(&mut self, move_type: &str, default_value: bool) {
        self.tiles
            .iter_mut()
            .for_each(|tile| tile.add_movetype(move_type, default_value));
        self.pathfinding_cache.clear();
    }
}

// Internal Map struct for pathfinding using alternate movement types.
// When calling a pathfinding function for swim or fly on the Labyrinth2D struct,
// it generates one of these and pathfinds over that.
#[derive(Serialize, Deserialize, Clone, Debug)]
struct InternalLabyrinth2D {
    opaque: Vec<bool>,
    enterable: Vec<bool>,
    dimensions: Point,
}

impl InternalLabyrinth2D {
    fn from_map(map: &Labyrinth2D, move_types: &[MoveType]) -> InternalLabyrinth2D {
        let enterable = map
            .tiles
            .iter()
            .map(|tile| tile.can_enter(move_types))
            .collect::<Vec<bool>>();

        let opaque: Vec<bool> = map.tiles.iter().map(|tile| tile.opaque).collect();

        InternalLabyrinth2D {
            opaque,
            enterable,
            dimensions: map.dimensions(),
        }
    }
}

impl Algorithm2D for InternalLabyrinth2D {
    fn dimensions(&self) -> Point {
        self.dimensions
    }
}

impl BaseMap for InternalLabyrinth2D {
    fn is_opaque(&self, _idx: usize) -> bool {
        self.opaque[_idx]
    }

    fn get_available_exits(&self, _idx: usize) -> SmallVec<[(usize, f32); 10]> {
        // TODO: Maybe figure out how to generalize this
        let start = self.index_to_point2d(_idx);
        let deltas = [
            Point::new(-1, 0),
            Point::new(0, -1),
            Point::new(1, 0),
            Point::new(0, 1),
        ];

        deltas
            .iter()
            // apply each delta to the point
            .map(|&diff| start + diff)
            // filter to only points in map bounds
            .filter(|&pt| self.in_bounds(pt))
            // map points -> vector indices
            .map(|pt| self.point2d_to_index(pt))
            // filter to only tiles that are walkable
            .filter(|&pos| self.enterable[pos])
            // package into final struct
            .map(|pos| (pos, 1.0))
            // finally, collect into the final SmallVec
            .collect::<SmallVec<[(_, _); 10]>>()
    }

    fn get_pathing_distance(&self, _idx1: usize, _idx2: usize) -> f32 {
        DistanceAlg::Pythagoras
            .distance2d(self.index_to_point2d(_idx1), self.index_to_point2d(_idx2))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Trait implementation tests
    fn count_neighbors(map: &Labyrinth2D, idx: usize) -> usize {
        map.get_available_exits(idx).len()
    }

    #[test]
    fn out_of_bounds_neighbors_are_ignored() {
        let map = Labyrinth2D::new_empty(3, 3);

        assert_eq!(count_neighbors(&map, 4), 4); // Center

        assert_eq!(count_neighbors(&map, 0), 2); // Upper left corner
        assert_eq!(count_neighbors(&map, 2), 2); // Upper right corner
        assert_eq!(count_neighbors(&map, 6), 2); // Lower left corner
        assert_eq!(count_neighbors(&map, 8), 2); // Lower right corner

        assert_eq!(count_neighbors(&map, 1), 3); // Top edge
        assert_eq!(count_neighbors(&map, 3), 3); // Left edge
        assert_eq!(count_neighbors(&map, 5), 3); // Right edge
        assert_eq!(count_neighbors(&map, 7), 3); // Bottom edge
    }

    fn prepare_testmap_3x3() -> Labyrinth2D {
        let mut map = Labyrinth2D::new(3, 3);

        map.set_tile_at(Point::new(0, 1), Tile::water());
        map.set_tile_at(Point::new(1, 0), Tile::floor());
        map.set_tile_at(Point::new(1, 2), Tile::lava());
        map.set_tile_at(Point::new(2, 1), Tile::chasm());

        map
    }

    fn prepare_testmap_3x3_for_movtype(movtypes: &[MoveType]) -> InternalLabyrinth2D {
        let map = prepare_testmap_3x3();
        InternalLabyrinth2D::from_map(&map, movtypes)
    }

    fn smallvecs_are_equal<T: Copy + PartialEq>(
        a: SmallVec<[T; 10]>,
        b: SmallVec<[T; 10]>,
    ) -> bool {
        // if the two vecs have the same length
        if a.len() != b.len() {
            return false;
        }

        let vec_b = b.into_vec();

        // count the number of times each element in a appears in b
        let a_in_b = a
            .iter()
            .filter(|&item| vec_b.contains(item))
            .map(|&a_item| vec_b.iter().filter(|&&b_item| a_item == b_item).count())
            .collect::<Vec<usize>>();

        // count the number of times an element is in a
        let a_in_a = a
            .iter()
            .map(|&a_item| a.iter().filter(|&item| a_item == *item).count())
            .collect::<Vec<usize>>();

        a_in_a == a_in_b
    }

    #[test]
    fn smallvec_equality_tests_work() {
        // smallvecs with the same elements are equal
        assert!(smallvecs_are_equal(
            smallvec![(1, 1.0)],
            smallvec![(1, 1.0)]
        ));

        // order invariance of elements
        assert!(smallvecs_are_equal(
            smallvec![(1, 1.0), (2, 1.0)],
            smallvec![(2, 1.0), (1, 1.0)]
        ));

        // smallvecs with different elements are NOT equal
        // completely different items
        assert!(!smallvecs_are_equal(
            smallvec![(3, 1.0)],
            smallvec![(2, 1.0)]
        ));

        // same items, different occurences
        assert!(!smallvecs_are_equal(
            smallvec![(2, 1.0), (2, 1.0)],
            smallvec![(2, 1.0)]
        ));
        assert!(!smallvecs_are_equal(
            smallvec![(2, 1.0)],
            smallvec![(2, 1.0), (2, 1.0)]
        ));
    }

    #[test]
    fn walk_on_default_tiles() {
        let map = prepare_testmap_3x3();

        let center = map.point2d_to_index(Point::new(1, 1));
        let expected: SmallVec<[(usize, f32); 10]> =
            smallvec![(map.point2d_to_index(Point::new(1, 0)), 1.0)];

        assert_eq!(map.get_available_exits(center), expected);
    }

    #[test]
    fn fly_on_default_tiles() {
        let map = prepare_testmap_3x3_for_movtype(&[MoveType::Fly]);

        let center = map.point2d_to_index(Point::new(1, 1));

        let expected: SmallVec<[(usize, f32); 10]> = smallvec![
            (map.point2d_to_index(Point::new(0, 1)), 1.0), // water
            (map.point2d_to_index(Point::new(1, 0)), 1.0), // floor
            (map.point2d_to_index(Point::new(1, 2)), 1.0), // lava
            (map.point2d_to_index(Point::new(2, 1)), 1.0), // chasm
        ];

        assert!(smallvecs_are_equal(
            map.get_available_exits(center),
            expected
        ));
    }

    #[test]
    fn swim_on_default_tiles() {
        let map = prepare_testmap_3x3_for_movtype(&[MoveType::Swim]);

        let center = map.point2d_to_index(Point::new(1, 1));

        let expected: SmallVec<[(usize, f32); 10]> = smallvec![
            (map.point2d_to_index(Point::new(0, 1)), 1.0), // water
        ];

        assert!(smallvecs_are_equal(
            map.get_available_exits(center),
            expected
        ));
    }

    #[test]
    fn no_movement_can_enter_walls() {
        let walkmap = Labyrinth2D::new(3, 3);
        let flymap = InternalLabyrinth2D::from_map(&walkmap, &[MoveType::Fly]);
        let swimmap = InternalLabyrinth2D::from_map(&walkmap, &[MoveType::Swim]);

        let center = walkmap.point2d_to_index(Point::new(1, 1));

        assert!(walkmap.get_available_exits(center).is_empty());
        assert!(flymap.get_available_exits(center).is_empty());
        assert!(swimmap.get_available_exits(center).is_empty());
    }

    #[test]
    fn custom_movement_types_are_usable() -> Result<(), String> {
        let mut map = Labyrinth2D::new(3, 3);
        let phasewall = TileBuilder::wall()
            .kind("phasewall")
            .add_movetype("phasing", true)
            .build()?;

        let center = map.point2d_to_index(Point::new(1, 1));

        map.add_movetype("phasing", false);

        map.set_tile_at(Point::new(0, 1), phasewall.clone());
        map.set_tile_at(Point::new(1, 0), phasewall.clone());

        let expected: SmallVec<[(usize, f32); 10]> = smallvec![
            (map.point2d_to_index(Point::new(0, 1)), 1.0),
            (map.point2d_to_index(Point::new(1, 0)), 1.0),
        ];

        let phasemap =
            InternalLabyrinth2D::from_map(&map, &[MoveType::Custom("phasing".to_string())]);

        assert!(smallvecs_are_equal(
            phasemap.get_available_exits(center),
            expected
        ));

        Ok(())
    }

    // Map Cache behavior tests
    #[test]
    fn pathfinding_walk_doesnt_add_to_map_cache() {
        let mut map = Labyrinth2D::new(10, 10);

        let start = Point::new(1, 1);
        let end = Point::new(5, 5);

        let mut _path = map.find_path_walk(start, end);
        _path = map.find_path(start, end, &[MoveType::Walk]);

        assert_eq!(map.pathfinding_cache.len(), 0);
    }

    #[test]
    fn pathfinding_adds_to_map_cache() {
        let mut map = Labyrinth2D::new(10, 10);

        let start = Point::new(1, 1);
        let end = Point::new(5, 5);

        let _path = map.find_path_swim(start, end);

        assert_eq!(map.pathfinding_cache.len(), 1);
    }

    #[test]
    fn pathfinding_twice_doesnt_add_to_map_cache() {
        let mut map = Labyrinth2D::new(10, 10);

        let start = Point::new(1, 1);
        let end = Point::new(5, 5);

        let _path1 = map.find_path_swim(start, end);
        let _path2 = map.find_path_swim(start, end);

        assert_eq!(map.pathfinding_cache.len(), 1);
    }

    #[test]
    fn map_cache_entries_are_order_insensitive() {
        let mut map = Labyrinth2D::new(10, 10);

        let start = Point::new(1, 1);
        let end = Point::new(5, 5);

        let movetype1 = &[MoveType::Walk, MoveType::Fly];
        let movetype2 = &[MoveType::Fly, MoveType::Walk];

        let _path1 = map.find_path(start, end, movetype1);
        let _path2 = map.find_path(start, end, movetype2);

        // The second one should not be added to the cache (since they're the same)
        assert_eq!(map.pathfinding_cache.len(), 1);
    }

    #[test]
    fn dijkstra_maps_walk_dont_add_to_map_cache() {
        let mut map = Labyrinth2D::new(10, 10);

        let start = Point::new(1, 1);

        let mut _d_map = map.dijkstra_map_walk(&[start]);
        _d_map = map.dijkstra_map(&[start], &[MoveType::Walk]);

        assert_eq!(map.pathfinding_cache.len(), 0);
    }

    #[test]
    fn dijkstra_maps_add_to_map_cache() {
        let mut map = Labyrinth2D::new(10, 10);

        let start = Point::new(1, 1);

        let _d_map = map.dijkstra_map_fly(&[start]);

        assert_eq!(map.pathfinding_cache.len(), 1);
    }

    #[test]
    fn dijkstra_maps_twice_doesnt_add_to_map_cache() {
        let mut map = Labyrinth2D::new(10, 10);

        let start = Point::new(1, 1);

        let _d_map = map.dijkstra_map_fly(&[start]);
        let _d_map2 = map.dijkstra_map_fly(&[start]);

        assert_eq!(map.pathfinding_cache.len(), 1);
    }

    #[test]
    fn pathfinding_and_dijkstra_maps_share_map_cache() {
        let mut map = Labyrinth2D::new(10, 10);

        let start = Point::new(1, 1);
        let end = Point::new(5, 5);

        let _d_map = map.dijkstra_map_fly(&[start]);
        let _path = map.find_path_fly(start, end);

        assert_eq!(map.pathfinding_cache.len(), 1);
    }

    // Map editing tests
    #[test]
    fn editing_map_clears_cache() {
        let mut map = Labyrinth2D::new(10, 10);

        let start = Point::new(1, 1);
        let end = Point::new(5, 5);

        let mut _path = map.find_path_fly(start, end);
        assert_eq!(map.pathfinding_cache.len(), 1);
        map.set_tile_at(Point::new(3, 3), Tile::wall());
        assert_eq!(map.pathfinding_cache.len(), 0);

        _path = map.find_path_fly(start, end);
        assert_eq!(map.pathfinding_cache.len(), 1);
        map.set_floor(Point::new(3, 3));
        assert_eq!(map.pathfinding_cache.len(), 0);

        _path = map.find_path_fly(start, end);
        assert_eq!(map.pathfinding_cache.len(), 1);
        map.set_lava(Point::new(3, 3));
        assert_eq!(map.pathfinding_cache.len(), 0);

        _path = map.find_path_fly(start, end);
        assert_eq!(map.pathfinding_cache.len(), 1);
        map.add_movetype("something", false);
        assert_eq!(map.pathfinding_cache.len(), 0);
    }
}
