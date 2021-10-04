//! This module holds the [`Tile`] and [`TileBuilder`] structs.
//!
//!
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::MoveType;

/// Builder struct for [`Tiles`](Tile).
///
/// Will fail to build if any of the required fields
/// (all except custom_movetypes) are None.
///
/// # Examples
/// ```rust
/// fn main() -> Result<(), String> {
///     use labyrinth_map::prelude::*;
///
///     // Building a Tile manually
///     let wall_manual = TileBuilder::new()
///         .kind("wall")
///         .opaque(true)
///         .walk(false)
///         .fly(false)
///         .swim(false)
///         .build()?;
///
///     // Building a tile from a template
///     let wall_from_template = TileBuilder::wall()
///         // starts from preset wall template (same as above example)
///         .kind("crystalwall")
///         .opaque(false)
///         .build()?;
///
///     // Building a wall with a special access property
///     let phasable_wall = TileBuilder::wall()
///         .kind("phasebarrier")
///         .add_movetype("phase", true)
///         // This tile is accessible to any entity that can move via phasing
///         .build()?;
///
///     // Attempting a build when the TileBuilder isn't fully initialized will return an error
///     assert!(TileBuilder::new().build().is_err());
///
///     Ok(())
/// }
/// ```
///
/// # Note: Direct construction of Tiles
/// For common basic tiles (wall, floor, water, lava, chasm), direct constructors
/// have been implemented on the [`Tile`] struct so you don't have to go through
/// the builder for basic tiles.
///
/// ```rust
/// use labyrinth_map::prelude::*;
///
/// let plain_wall = Tile::wall();
/// let plain_floor = Tile::floor();
/// let plain_water = Tile::water();
/// let plain_lava = Tile::lava();
/// let plain_chasm = Tile::chasm();
/// ```
pub struct TileBuilder {
    /// The name of the tile to be built.
    ///
    /// When set through TileBuilder::kind(), the string is converted to lowercase
    pub kind: Option<String>,

    /// The opacity of the tile to be built, used to perform FoV calculations.
    pub opaque: Option<bool>,

    /// Whether or not the tile to be built allows entry via walking
    pub walk: Option<bool>,

    /// Whether or not the tile to be built allows entry via flying
    pub fly: Option<bool>,

    /// Whether or not the tile to be built allows entry via swimming
    pub swim: Option<bool>,

    /// A hashmap containing all user-defined movement methods, set using the
    /// [`TileBuilder::add_movetype`](TileBuilder::add_movetype) method
    pub custom_movetypes: HashMap<String, bool>,
}

impl Default for TileBuilder {
    fn default() -> Self {
        TileBuilder::new()
    }
}

impl TileBuilder {
    /// Initializes a TileBuilder with all Option fields set to None.
    pub fn new() -> TileBuilder {
        TileBuilder {
            kind: None,
            opaque: None,
            walk: None,
            fly: None,
            swim: None,
            custom_movetypes: HashMap::new(),
        }
    }

    /// Initializes a TileBuilder with the same properties as a basic wall tile:
    /// - blocks vision
    /// - impassable
    pub fn wall() -> TileBuilder {
        TileBuilder::new()
            .kind("wall")
            .opaque(true)
            .walk(false)
            .fly(false)
            .swim(false)
    }

    /// Initializes a TileBuilder with the same properties as a basic floor tile:
    /// - doesn't block vision
    /// - passable to walkers and flyers
    pub fn floor() -> TileBuilder {
        TileBuilder::new()
            .kind("floor")
            .opaque(false)
            .walk(true)
            .fly(true)
            .swim(false)
    }

    /// Initializes a TileBuilder with the same properties as a basic water tile:
    /// - doesn't block vision
    /// - passable to swimmers and flyers
    pub fn water() -> TileBuilder {
        TileBuilder::new()
            .kind("water")
            .opaque(false)
            .walk(false)
            .fly(true)
            .swim(true)
    }

    /// Initializes a TileBuilder with the same properties as a basic lava tile:
    /// - doesn't block vision
    /// - passable to flyers only
    pub fn lava() -> TileBuilder {
        TileBuilder::new()
            .kind("lava")
            .opaque(false)
            .walk(false)
            .fly(true)
            .swim(false)
    }

    /// Initializes a TileBuilder with the same properties as a basic chasm tile:
    /// - doesn't block vision
    /// - passable to flyers only
    pub fn chasm() -> TileBuilder {
        TileBuilder::new()
            .kind("chasm")
            .opaque(false)
            .walk(false)
            .fly(true)
            .swim(false)
    }

    /// Sets the kind of the tile to be built. Essentially its name.
    ///
    /// As pathfinding is based solely on the other fields, this field doesn't
    /// actually affect any calculations.
    ///
    /// However, it can be useful for differentiating between tiles that have
    /// similar movement properties that have to be distinct from each other
    /// for one reason or another (i.e. smoothened walls vs rough walls).
    pub fn kind(mut self, kind: &str) -> TileBuilder {
        let kind = kind.to_lowercase();
        self.kind = Some(kind);

        self
    }

    /// Sets the opacity of the tile. Whether or not it can be seen through.
    ///
    /// This is used in Field of View calculations.
    pub fn opaque(mut self, value: bool) -> TileBuilder {
        self.opaque = Some(value);
        self
    }

    /// Sets walk accessibility to the tile. Whether or not it can be accessed
    /// via walking.
    ///
    /// Used for pathfinding calculations
    pub fn walk(mut self, value: bool) -> TileBuilder {
        self.walk = Some(value);
        self
    }

    /// Sets fly accessibility to the tile. Whether or not it can be accessed
    /// via flying.
    ///
    /// Used for pathfinding calculations
    pub fn fly(mut self, value: bool) -> TileBuilder {
        self.fly = Some(value);
        self
    }

    /// Sets swim accessibility to the tile. Whether or not it can be accessed
    /// via swimming.
    ///
    /// Used for pathfinding calculations
    pub fn swim(mut self, value: bool) -> TileBuilder {
        self.swim = Some(value);
        self
    }

    /// Sets a custom movement property to the tile. Whether or not it can be
    /// accessed via a user-defined movement method.
    /// (i.e. lava walking, digging, etc.)
    pub fn add_movetype(mut self, movtype: &str, value: bool) -> TileBuilder {
        let prop = movtype.to_lowercase();

        self.custom_movetypes.entry(prop).or_insert(value);

        self
    }

    /// Checks if all required fields on the TileBuilder have been set.
    pub fn is_fully_initialized(&self) -> bool {
        self.kind.is_some()
            && self.opaque.is_some()
            && self.walk.is_some()
            && self.fly.is_some()
            && self.swim.is_some()
    }

    /// Attempts to build the tile. Returns an error if any required field
    /// is uninitialized.
    pub fn build(self) -> Result<Tile, String> {
        if !self.is_fully_initialized() {
            Err("Builder is not fully initialized!".to_string())
        } else {
            Ok(Tile {
                kind: self.kind.unwrap(),
                opaque: self.opaque.unwrap(),
                walk: self.walk.unwrap(),
                fly: self.fly.unwrap(),
                swim: self.swim.unwrap(),
                other_movement: self.custom_movetypes,
            })
        }
    }
}

/// Tile struct that contains its name (for differentiation purposes),
/// and its accessibility properties.
///
/// # Construction
/// ## Direct Construction
/// Direct constructors are provided for some basic tile types:
/// - Wall through [`Tile::wall()`]
///     - Blocks vision
///     - Impassable
/// - Floor through [`Tile::floor()`]
///     - Doesn't block vision
///     - Passable for walkers and flyers
/// - Water through [`Tile::water()`]
///     - Doesn't block vision
///     - Passable for flyers and swimmers
/// - Lava through [`Tile::lava()`]
///     - Doesn't block vision
///     - Passable for flyers
/// - Chasm through [`Tile::chasm()`]
///     - Doesn't block vision
///     - Passable for flyers
/// ## Using TileBuilder
/// For more complex tiles, a [`TileBuilder`] struct is provided.
/// For more in-depth examples, have a look at its [documentation](TileBuilder).
/// ```rust
/// fn main() -> Result<(), String> {
///     use labyrinth_map::prelude::*;
///
///     let crystal_wall = TileBuilder::wall()
///         .kind("crystal_wall")
///         .opaque(false)
///         .build()?;
///
///     Ok(())
/// }
/// ```

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Tile {
    /// The kind of tile it is.
    pub kind: String,

    /// Whether or not the tile blocks vision.
    pub opaque: bool,

    /// Whether or not a walking entity can access the tile.
    pub walk: bool,

    /// Whether or not a flying entity can access the tile.
    pub fly: bool,

    /// Whether or not a swimming entity can access the tile.
    pub swim: bool,

    /// Any other user-defined alternate forms of movement and their accessibility
    /// for that form of movement.
    pub other_movement: HashMap<String, bool>,
}

impl Default for Tile {
    fn default() -> Self {
        Tile::wall()
    }
}

impl Tile {
    // Basic Tile constructors
    /// Direct constructor for a wall tile
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

    /// Direct constructor for a floor tile
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

    /// Quick constructor for a water tile
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

    /// Direct constructor for a lava tile
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

    /// Direct constructor for a chasm tile
    pub fn chasm() -> Tile {
        Tile {
            kind: "chasm".to_string(),
            opaque: false,
            walk: false,
            fly: true,
            swim: false,
            other_movement: HashMap::new(),
        }
    }

    /// Adds a custom movement property to the tile. Whether or not it can be
    /// accessed via a user-defined movement method.
    /// (i.e. lava walking, digging, etc.)
    pub fn add_movetype(&mut self, movtype: &str, value: bool) {
        let prop = movtype.to_lowercase();

        self.other_movement.entry(prop).or_insert(value);
    }

    /// Check if an entity with the given move types can enter a tile.
    ///
    /// Returns an error if any of the movement types specified are not
    /// specified for the tile.
    ///
    /// # Example
    /// ```rust
    /// fn main() -> Result<(), String> {
    ///     use labyrinth_map::prelude::*;
    ///
    ///     let tile = TileBuilder::water().build()?;
    ///     
    ///     // Returns Ok(false) when inaccessible by movement type
    ///     assert!(!tile.can_enter(&[MoveType::Walk])?);
    ///
    ///     // Returns Ok(true) when accessible by movement type
    ///     assert!(tile.can_enter(&[MoveType::Fly])?);
    ///
    ///     // ... but will return an error on an undefined movement type
    ///     assert!(tile
    ///         .can_enter(&[MoveType::Custom("undefined_movetype".to_string())])
    ///         .is_err());
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn can_enter(&self, move_types: &[MoveType]) -> Result<bool, String> {
        // TODO: This should return a user-facing error
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
}

#[cfg(test)]
mod tests {
    use super::*;

    // Tile tests
    fn is_wall(tile: Tile) {
        assert!(tile.opaque);
        assert!(!tile.walk);
        assert!(!tile.fly);
        assert!(!tile.swim);
        assert_eq!(tile.other_movement, HashMap::new());
    }
    fn is_floor(tile: Tile) {
        assert!(!tile.opaque);
        assert!(tile.walk);
        assert!(tile.fly);
        assert!(!tile.swim);
        assert_eq!(tile.other_movement, HashMap::new());
    }
    fn is_water(tile: Tile) {
        assert!(!tile.opaque);
        assert!(!tile.walk);
        assert!(tile.fly);
        assert!(tile.swim);
        assert_eq!(tile.other_movement, HashMap::new());
    }
    fn is_lava(tile: Tile) {
        assert!(!tile.opaque);
        assert!(!tile.walk);
        assert!(tile.fly);
        assert!(!tile.swim);
        assert_eq!(tile.other_movement, HashMap::new());
    }
    fn is_chasm(tile: Tile) {
        assert!(!tile.opaque);
        assert!(!tile.walk);
        assert!(tile.fly);
        assert!(!tile.swim);
        assert_eq!(tile.other_movement, HashMap::new());
    }

    #[test]
    fn builder_wall_template() {
        let builder = TileBuilder::wall();
        let tile = builder.build().unwrap();

        // assert that it's a wall
        is_wall(tile);
    }

    #[test]
    fn builder_floor_template() {
        let builder = TileBuilder::floor();
        let tile = builder.build().unwrap();

        // assert that it's a floor
        is_floor(tile);
    }

    #[test]
    fn builder_water_template() {
        let builder = TileBuilder::water();
        let tile = builder.build().unwrap();

        // assert that it's water
        is_water(tile);
    }

    #[test]
    fn builder_lava_template() {
        let builder = TileBuilder::lava();
        let tile = builder.build().unwrap();

        // assert that it's lava
        is_lava(tile);
    }
    #[test]
    fn builder_chasm_template() {
        let builder = TileBuilder::chasm();
        let tile = builder.build().unwrap();

        // assert that it's a chasm
        is_chasm(tile);
    }

    #[test]
    #[should_panic]
    fn unfinished_builder_should_panic() {
        let builder = TileBuilder::new();
        builder.build().unwrap();
    }

    #[test]
    fn tile_enterable_with_one_matching_movtype() -> Result<(), String> {
        let custom_tile = TileBuilder::floor().kind("sometile").build()?;

        assert!(custom_tile.can_enter(&[MoveType::Walk])?);

        Ok(())
    }

    #[test]
    fn tile_enterable_with_multiple_matching_movtype() -> Result<(), String> {
        let custom_tile = TileBuilder::floor().kind("sometile").build()?;

        assert!(custom_tile.can_enter(&[MoveType::Walk, MoveType::Fly])?);

        Ok(())
    }

    #[test]
    fn tile_enterable_with_matching_and_unmatching_movtype() -> Result<(), String> {
        let custom_tile = TileBuilder::floor().kind("sometile").build()?;

        assert!(custom_tile.can_enter(&[MoveType::Walk, MoveType::Swim])?);

        Ok(())
    }

    #[test]
    fn tile_enterable_with_custom_movetype() -> Result<(), String> {
        let custom_tile = TileBuilder::wall().add_movetype("dig", true).build()?;

        assert!(custom_tile.can_enter(&[MoveType::Custom("dig".to_string())])?);

        Ok(())
    }

    #[test]
    fn movetypes_can_be_directly_added_to_tiles() -> Result<(), String> {
        let mut tile = Tile::wall();
        tile.add_movetype("dig", true);

        assert!(tile.can_enter(&[MoveType::Custom("dig".to_string())])?);
        Ok(())
    }

    #[test]
    fn tile_not_enterable() -> Result<(), String> {
        let custom_tile = TileBuilder::floor().kind("sometile").build()?;
        let res = custom_tile.can_enter(&[MoveType::Swim])?;

        assert!(!res);
        Ok(())
    }

    #[test]
    fn undefined_movtype_causes_error() -> Result<(), String> {
        let custom_tile = TileBuilder::floor().kind("sometile").build()?;
        let res = custom_tile.can_enter(&[MoveType::Custom("undefined".to_string())]);

        assert!(res.is_err());
        Ok(())
    }

    #[test]
    fn undefined_movtype_always_causes_error() -> Result<(), String> {
        let custom_tile = TileBuilder::floor().kind("sometile").build()?;
        let res =
            custom_tile.can_enter(&[MoveType::Walk, MoveType::Custom("undefined".to_string())]);

        assert!(res.is_err());
        Ok(())
    }

    #[test]
    fn tiles_with_diff_kind_can_still_have_same_properties() -> Result<(), String> {
        let custom_tile = TileBuilder::wall().kind("smoothwall").build()?;

        let wall = Tile::wall();

        let movement_types = vec![
            vec![MoveType::Walk],
            vec![MoveType::Fly],
            vec![MoveType::Swim],
        ];

        for movtype in movement_types {
            assert_eq!(wall.can_enter(&movtype), custom_tile.can_enter(&movtype))
        }
        Ok(())
    }
}
