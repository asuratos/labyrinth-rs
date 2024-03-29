//! Module for map objects

use std::collections::HashSet;

use bracket_pathfinding::prelude::*;

#[cfg(feature = "serialization")]
mod labyrinth_serialization;

mod tiles;
pub use tiles::MoveType;
pub use tiles::*;

// TODO: Better Map struct documentation
/// Labyrinth2D struct, the output of the MapGenerator2D.
///
/// Implements [`Algorithm2D`] and [`BaseMap`] traits from bracket-pathfinding,
/// which allows for bracket-lib pathfinding algorithms.
/// Also comes with built-in implementations for pathfinding for alternate
/// movement methods (swim and fly).
/// ```rust
/// use labyrinth_map::prelude::*;
///
/// let map = Labyrinth2D::new(10,10);
/// ```
#[derive(Clone, Debug)]
pub struct Labyrinth2D {
    // The vector of tiles in the map.
    tiles: Vec<Tile>,
    dimensions: Point,

    // Internal state vector for pathfinding filters
    _filter: Vec<MoveType>,
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
        self.tiles[_idx].is_opaque()
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
            // // filter to only tiles that are walkable
            .filter(|&pt| self.can_enter(pt, &self._filter))
            // map points -> vector indices
            .map(|pt| self.point2d_to_index(pt))
            // package into final struct
            // TODO: Make the cost variable (have can_enter return (bool, float)?)
            .map(|pos| (pos, 1.0))
            // finally, collect into the final SmallVec
            .collect::<SmallVec<[(_, _); 10]>>()
    }

    fn get_pathing_distance(&self, _idx1: usize, _idx2: usize) -> f32 {
        DistanceAlg::Pythagoras
            .distance2d(self.index_to_point2d(_idx1), self.index_to_point2d(_idx2))
    }
}

impl PartialEq for Labyrinth2D {
    fn eq(&self, other: &Self) -> bool {
        self.tiles == other.tiles && self.dimensions == other.dimensions
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
            _filter: vec![],
        }
    }

    /// Constructs a new Labyrinth with the passed width and height values.
    ///
    /// Initial Tiles are all floors.
    pub fn new_empty(width: usize, height: usize) -> Labyrinth2D {
        Labyrinth2D {
            tiles: vec![Tile::floor(); width * height],
            dimensions: Point::new(width, height),
            _filter: vec![],
        }
    }

    /// Constructus a new Labyrinth with the passed width and height values.
    ///
    /// Initial Tiles are floors, with the boundary tiles being all walls.
    pub fn new_walled(width: usize, height: usize) -> Labyrinth2D {
        let mut tiles = vec![Tile::floor(); width * height];
        tiles = tiles
            .iter()
            .enumerate()
            .map(|(i, tile)| {
                if (i < (width))
                    || (i > ((width * height) - width))
                    || (i % height == 0)
                    || (i % height == width - 1)
                {
                    Tile::wall()
                } else {
                    tile.clone()
                }
            })
            .collect();

        Labyrinth2D {
            tiles,
            dimensions: Point::new(width, height),
            _filter: vec![],
        }
    }

    /// Constructs a new Labyrinth with a size defined by a 2d [`Point`].
    ///
    /// Initial Tiles are all walls.
    pub fn new_from_dims(dimensions: Point) -> Labyrinth2D {
        Labyrinth2D::new(dimensions.x as usize, dimensions.y as usize)
    }

    /// Constructs a new Labyrinth with a size defined by a 2d [`Point`].
    ///
    /// Initial Tiles are all floors.
    pub fn new_empty_from_dims(dimensions: Point) -> Labyrinth2D {
        Labyrinth2D::new_empty(dimensions.x as usize, dimensions.y as usize)
    }

    /// Constructs a new Labyrinth with a size defined by a 2d [`Point`].
    ///
    /// Initial Tiles are floors, with the boundary tiles being all walls.
    pub fn new_walled_from_dims(dimensions: Point) -> Labyrinth2D {
        Labyrinth2D::new_walled(dimensions.x as usize, dimensions.y as usize)
    }

    // -------------------- Pathfinding functions -----------------
    /// Checks if the tile at a given [`Point`] can be entered for an entity
    /// with the specified movement types.
    pub fn can_enter<'a, T>(&self, loc: Point, move_types: T) -> bool
    where
        T: IntoIterator<Item = &'a MoveType>,
    {
        self.tile_at(loc).can_enter(move_types)
    }

    /// Returns the neighbors of a [`Point`] on the [`Labyrinth2D`],
    /// given the usable move types. Takes a collection of [`MoveType`] that
    /// implements Into<Vec<Movetype>>
    pub fn get_neighbors<T>(&mut self, loc: Point, move_types: T) -> Vec<Point>
    where
        T: Into<Vec<MoveType>>,
    {
        self._filter = move_types.into();

        let idx = self.point2d_to_index(loc);

        self.get_available_exits(idx)
            .iter()
            .map(|(idx, _)| self.index_to_point2d(*idx))
            .collect()
    }

    /// Find the path between two [`Points`](Point) for an entity with multiple
    /// movement types.
    // TODO: Examples here
    // TODO: Try a version of this that doesn't require mutable access?
    pub fn find_path<T>(&mut self, start: Point, end: Point, move_types: T) -> NavigationPath
    where
        T: Into<Vec<MoveType>>,
    {
        let move_types_vec = move_types.into();

        // If the movetype is only walk, then pathfinding can be done on
        // the Map as-is
        if move_types_vec == vec![MoveType::Walk] || move_types_vec == vec![] {
            self._filter = vec![MoveType::Walk];
            // return self.find_path_walk(start, end);
        } else {
            // if it's not walk, then sort and assign as filter to the map
            self._filter = move_types_vec;
            self._filter.sort();
        }

        let path = a_star_search(
            self.point2d_to_index(start),
            self.point2d_to_index(end),
            self,
        );

        self._filter.clear();
        path
    }

    /// Returns Dijkstra map for a set of starting [`Points`](Point), given
    /// the movement types of the entity.
    // TODO: Examples here
    pub fn dijkstra_map<T>(&mut self, starts: &[Point], move_types: T) -> DijkstraMap
    where
        T: Into<Vec<MoveType>>,
    {
        let move_types_vec: Vec<MoveType> = move_types.into();
        // if the MoveType is only walk, then it can be done on the map itself
        if move_types_vec == [MoveType::Walk] || move_types_vec == [] {
            self._filter = vec![MoveType::Walk];
        } else {
            self._filter = move_types_vec;
            self._filter.sort();
        }

        let Point {
            x: size_x,
            y: size_y,
        } = self.dimensions;

        let starts_idx: Vec<usize> = starts.iter().map(|&pt| self.point2d_to_index(pt)).collect();

        let dmap = DijkstraMap::new(size_x, size_y, &starts_idx, self, 1024.0);
        self._filter.clear();
        dmap
    }

    // ---------------- Map editing methods --------------
    /// Gets a reference to a tile at a given [`Point`](Point)
    fn tile_at(&self, loc: Point) -> &Tile {
        let idx = self.point2d_to_index(loc);
        &self.tiles[idx]
    }

    /// Gets a mutable reference to a tile at a given [`Point`](Point)
    fn tile_at_mut(&mut self, loc: Point) -> &mut Tile {
        let idx = self.point2d_to_index(loc);
        &mut self.tiles[idx]
    }

    /// Gets the accessibility of a tile at a given [`Point`]
    pub fn tile_access(&self, loc: Point) -> &HashSet<MoveType> {
        self.tile_at(loc).access()
    }

    /// Gets the tile kind of the tile at a given [`Point`]
    pub fn tile_kind(&self, loc: Point) -> &String {
        self.tile_at(loc).kind()
    }

    /// Sets the tile at the given [`Point`](Point) to a [`Tile`].
    pub fn set_tile_at(&mut self, loc: Point, tile: Tile) {
        *self.tile_at_mut(loc) = tile;
    }

    /// Sets the kind of the tile at a given [`Point`]
    pub fn set_tile_kind<T>(&mut self, loc: Point, kind: T)
    where
        T: Into<String>,
    {
        self.tile_at_mut(loc).set_kind(kind);
    }

    /// Sets the opacity of a tile at a given [`Point`].
    pub fn set_tile_opacity(&mut self, loc: Point, opaque: bool) {
        self.tile_at_mut(loc).set_opacity(opaque);
    }

    /// Adds a set of movetypes to a tile at the given [`Point`](Point).
    pub fn add_movetypes<T>(&mut self, loc: Point, move_types: T)
    where
        T: IntoIterator<Item = MoveType>,
    {
        self.tile_at_mut(loc).add_movetypes(move_types);
        // for move_type in move_types {
        //     self.tile_at_mut(loc).add_movetype(move_type);
        // }
    }

    /// Removes a set of movetypes to a tile at the given [`Point`](Point).
    pub fn remove_movetypes<T>(&mut self, loc: Point, move_types: T)
    where
        T: IntoIterator<Item = MoveType>,
    {
        for move_type in move_types {
            self.tile_at_mut(loc).remove_movetype(&move_type);
        }
    }

    // ----------------- Map Accessor Methods --------------
    // TODO: test these probably
    /// Getter for the total size of the [`Labyrinth2D`], in total number of
    /// tiles.
    pub fn size(&self) -> usize {
        self.tiles.len()
    }

    /// Getter for a vector of the tiles within the [`Labyrinth2D`]
    pub fn tiles(&self) -> &Vec<Tile> {
        &self.tiles
    }

    /// Gets an immutable iterator of all tiles in the [`Labyrinth2D`]
    pub fn iter(&self) -> core::slice::Iter<Tile> {
        self.tiles.iter()
    }

    /// Gets a mutable iterator of all tiles in the [`Labyrinth2D`]
    pub fn iter_mut(&mut self) -> core::slice::IterMut<Tile> {
        self.tiles.iter_mut()
    }

    /// Gets an immutable iterator over the rows of the [`Labyrinth2D`]
    pub fn rows(&self) -> Rows<Tile> {
        Rows(self.tiles.chunks(self.dimensions().x as usize))
    }

    /// Gets a mutable iterator over the rows of the [`Labyrinth2D`]
    pub fn rows_mut(&mut self) -> RowsMut<Tile> {
        let width = self.dimensions().x as usize;
        RowsMut(self.tiles.chunks_mut(width))
    }
}

/// Iterator over the rows of a [`Labyrinth2D`]
pub struct Rows<'a, T>(std::slice::Chunks<'a, T>);

impl<'a, T> Iterator for Rows<'a, T> {
    type Item = &'a [T];

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

/// Mutable Iterator over the rows of a [`Labyrinth2D`]
pub struct RowsMut<'a, T>(std::slice::ChunksMut<'a, T>);

impl<'a, T> Iterator for RowsMut<'a, T> {
    type Item = &'a mut [T];

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Serialization
    // #[test]
    // fn serialize() {
    //     let map = Labyrinth2D::new_empty(10, 10);
    //     map.dump();
    // }

    // Trait implementation tests
    fn count_neighbors(map: &Labyrinth2D, idx: usize) -> usize {
        map.get_available_exits(idx).len()
    }

    #[test]
    fn out_of_bounds_neighbors_are_ignored() {
        let mut map = Labyrinth2D::new_empty(3, 3);
        map._filter = vec![MoveType::Walk];

        println!("{:?}", map.get_available_exits(4));

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

    fn prepare_testmap_3x3_for_movtype(movtypes: &[MoveType]) -> Labyrinth2D {
        let mut map = prepare_testmap_3x3();
        map._filter = movtypes.to_vec();
        map
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
        let map = prepare_testmap_3x3_for_movtype(&[MoveType::Walk]);

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
        let mut map = Labyrinth2D::new(3, 3);

        let center = map.point2d_to_index(Point::new(1, 1));

        assert!(map.get_available_exits(center).is_empty());
        map._filter = vec![MoveType::Fly];
        assert!(map.get_available_exits(center).is_empty());
        map._filter = vec![MoveType::Swim];
        assert!(map.get_available_exits(center).is_empty());
    }

    #[test]
    fn custom_movement_types_are_usable() -> Result<(), String> {
        let mut map = Labyrinth2D::new(3, 3);

        let mut phasewall = Tile::wall();
        phasewall.add_movetype(MoveType::custom("phasing"));

        let center = map.point2d_to_index(Point::new(1, 1));

        map.set_tile_at(Point::new(0, 1), phasewall.clone());
        map.set_tile_at(Point::new(1, 0), phasewall.clone());

        let expected: SmallVec<[(usize, f32); 10]> = smallvec![
            (map.point2d_to_index(Point::new(0, 1)), 1.0),
            (map.point2d_to_index(Point::new(1, 0)), 1.0),
        ];

        map._filter = vec![MoveType::custom("phasing")];

        assert!(smallvecs_are_equal(
            map.get_available_exits(center),
            expected
        ));

        Ok(())
    }

    // Map editing tests
    #[test]
    fn edit_opacity() {
        let mut map = Labyrinth2D::new(3, 3);

        let target = Point::new(1, 1);
        let target_idx = map.point2d_to_index(target);

        assert!(map.is_opaque(target_idx));
        map.set_tile_opacity(target, false);
        assert!(!map.is_opaque(target_idx));
    }

    #[test]
    fn edit_accessibility() {
        let mut map = Labyrinth2D::new(3, 3);

        let target = Point::new(1, 1);

        assert!(!map.can_enter(target, &[MoveType::Walk]));

        map.add_movetypes(target, [MoveType::Walk]);
        assert!(map.can_enter(target, &[MoveType::Walk]));

        map.add_movetypes(target, [MoveType::custom("dig")]);
        assert!(map.can_enter(target, &[MoveType::custom("dig")]));
    }

    #[test]
    fn edit_tile_kind() {
        let mut map = Labyrinth2D::new(3, 3);

        let target = Point::new(1, 1);

        assert!(map.tile_kind(target) == "wall");
        map.set_tile_kind(target, "crystal");
        assert!(map.tile_kind(target) == "crystal");
    }

    #[test]
    fn tile_kind_is_case_insensitive() {
        let mut map = Labyrinth2D::new(3, 3);

        let target = Point::new(1, 1);

        assert!(map.tile_kind(target) == "wall");
        map.set_tile_kind(target, "crystal");
        assert!(map.tile_kind(target) == "crystal");
        map.set_tile_kind(target, "Crystal");
        assert!(map.tile_kind(target) == "crystal");
    }

    // Pathfinding tests
    #[test]
    fn find_path_resets_filter() {
        let mut map = Labyrinth2D::new(3, 3);
        let start = Point::new(0, 0);
        let end = Point::new(2, 2);

        map._filter = vec![MoveType::Fly];

        map.find_path(start, end, [MoveType::Walk]);

        assert_eq!(map._filter, vec![]);
    }

    #[test]
    fn dijkstra_resets_filter() {
        let mut map = Labyrinth2D::new(3, 3);
        let start = Point::new(0, 0);

        map._filter = vec![MoveType::Fly];

        map.dijkstra_map(&[start], [MoveType::Walk]);

        assert_eq!(map._filter, vec![]);
    }
}
