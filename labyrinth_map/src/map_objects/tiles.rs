//! This module holds the [`Tile`] and [`TileBuilder`] structs.
//!
//!

#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};

use std::collections::HashSet;

macro_rules! set {
    ( $( $x:expr ),* ) => {  // Match zero or more comma delimited items
        {
            let mut temp_set = HashSet::new();  // Create a mutable HashSet
            $(
                temp_set.insert($x); // Insert each item matched into the HashSet
            )*
            temp_set // Return the populated HashSet
        }
    };
}

/// Enum defining possible movement methods
#[derive(PartialEq, Eq, Ord, PartialOrd, Hash, Debug, Clone)]
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
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

impl MoveType {
    /// Convenience function for building a MoveType::Custom.
    /// Converts to lowercase.
    pub fn custom<T>(movtype: T) -> MoveType
    where
        T: Into<String>,
    {
        let lcase = movtype.into().to_lowercase();

        MoveType::Custom(lcase)
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
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
pub struct Tile {
    /// The kind of tile it is.
    kind: String,

    /// Whether or not the tile blocks vision.
    opaque: bool,

    /// A hashset that defines the movement types that can enter the Tile.
    access: HashSet<MoveType>,
}

impl Default for Tile {
    fn default() -> Self {
        Tile::wall()
    }
}

impl Tile {
    // Basic Tile constructors

    /// Explicit tile constructor
    pub fn new<T, U>(kind: T, opaque: bool, access: U) -> Tile
    where
        T: Into<String>,
        U: IntoIterator<Item = MoveType>,
    {
        let mut access_map = HashSet::new();

        for movtype in access {
            access_map.insert(movtype.to_owned());
        }

        Tile {
            kind: kind.into().to_lowercase(),
            access: access_map,
            opaque,
        }
    }

    /// Direct constructor for a wall tile
    pub fn wall() -> Tile {
        Tile::new("wall", true, [])
    }

    /// Direct constructor for a floor tile
    pub fn floor() -> Tile {
        Tile::new("floor", false, [MoveType::Walk, MoveType::Fly])
    }

    /// Quick constructor for a water tile
    pub fn water() -> Tile {
        Tile::new("water", false, [MoveType::Swim, MoveType::Fly])
    }

    /// Direct constructor for a lava tile
    pub fn lava() -> Tile {
        Tile::new("lava", false, [MoveType::Fly])
    }

    /// Direct constructor for a chasm tile
    pub fn chasm() -> Tile {
        Tile::new("chasm", false, [MoveType::Fly])
    }

    pub fn kind(&self) -> &String {
        &self.kind
    }

    pub fn set_kind<T>(&mut self, kind: T)
    where
        T: Into<String>,
    {
        self.kind = kind.into().to_lowercase();
    }

    pub fn is_opaque(&self) -> bool {
        self.opaque
    }

    pub fn set_opacity(&mut self, opacity: bool) {
        self.opaque = opacity;
    }

    /// Adds a custom movement property to the tile. Whether or not it can be
    /// accessed via a user-defined movement method.
    /// (i.e. lava walking, digging, etc.)
    pub fn add_movetype(&mut self, movtype: MoveType) -> bool {
        match movtype {
            MoveType::Custom(str) => self.access.insert(MoveType::custom(str)),
            _ => self.access.insert(movtype.clone()),
        }
    }

    pub fn add_movetypes<T>(&mut self, movtypes: T)
    where
        T: IntoIterator<Item = MoveType>,
    {
        self.access
            .extend(movtypes.into_iter().map(|movtype| match movtype {
                MoveType::Custom(str) => MoveType::custom(str),
                _ => movtype.clone(),
            }))
    }

    /// Removes a custom movement property to the tile. Whether or not it can be
    /// accessed via a user-defined movement method.
    /// (i.e. lava walking, digging, etc.)
    pub fn remove_movetype(&mut self, movtype: &MoveType) -> bool {
        match movtype {
            MoveType::Custom(str) => self.access.remove(&MoveType::custom(str)),
            _ => self.access.remove(movtype),
        }
    }

    /// Checks if an entity with the given move types can enter the tile.
    /// Transforms MoveType::Custom strings into lowercase before matching
    pub fn can_enter<'a, T>(&self, move_types: T) -> bool
    where
        T: IntoIterator<Item = &'a MoveType>,
    {
        move_types
            .into_iter()
            .map(|move_type| match move_type {
                MoveType::Custom(kind) => MoveType::custom(&kind.clone()),
                _ => move_type.clone(),
            })
            .any(|move_type| self.access.contains(&move_type))
    }

    /// Returns the accessibility of a tile
    pub fn access(&self) -> &HashSet<MoveType> {
        &self.access
    }
}

pub struct TileBuilder {
    kind: Option<String>,
    opaque: Option<bool>,
    access: Vec<MoveType>,
}

impl TileBuilder {
    pub fn new() -> TileBuilder {
        TileBuilder {
            kind: None,
            opaque: None,
            access: vec![],
        }
    }

    pub fn wall() -> TileBuilder {
        TileBuilder {
            kind: Some(String::from("wall")),
            opaque: Some(true),
            access: vec![],
        }
    }

    pub fn floor() -> TileBuilder {
        TileBuilder {
            kind: Some(String::from("floor")),
            opaque: Some(false),
            access: vec![MoveType::Walk, MoveType::Fly],
        }
    }

    pub fn water() -> TileBuilder {
        TileBuilder {
            kind: Some(String::from("water")),
            opaque: Some(false),
            access: vec![MoveType::Swim, MoveType::Fly],
        }
    }

    pub fn lava() -> TileBuilder {
        TileBuilder {
            kind: Some(String::from("lava")),
            opaque: Some(false),
            access: vec![MoveType::Fly],
        }
    }

    pub fn chasm() -> TileBuilder {
        TileBuilder {
            kind: Some(String::from("chasm")),
            opaque: Some(false),
            access: vec![MoveType::Fly],
        }
    }

    pub fn with_kind<T>(mut self, kind: T) -> TileBuilder
    where
        T: Into<String>,
    {
        self.kind = Some(kind.into().to_lowercase());
        self
    }

    pub fn with_opacity(mut self, opacity: bool) -> TileBuilder {
        self.opaque = Some(opacity);
        self
    }

    pub fn with_access(mut self, movtypes: &[MoveType]) -> TileBuilder {
        self.access.extend_from_slice(movtypes);
        self
    }

    //TODO: Builder Error?
    pub fn build(self) -> Result<Tile, String> {
        if self.opaque.is_none() && self.kind.is_none() {
            return Err(String::from("Builder not fully initialized!"));
        }

        Ok(Tile::new(
            &self.kind.unwrap(),
            self.opaque.unwrap(),
            self.access,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Tile tests
    fn is_wall(tile: Tile) {
        assert!(tile.opaque);
        assert!(tile.access == set![])
    }
    fn is_floor(tile: Tile) {
        assert!(!tile.opaque);
        assert!(tile.access == set![MoveType::Walk, MoveType::Fly]);
    }
    fn is_water(tile: Tile) {
        assert!(!tile.opaque);
        assert!(tile.access == set![MoveType::Swim, MoveType::Fly]);
    }
    fn is_lava(tile: Tile) {
        assert!(!tile.opaque);
        assert!(tile.access == set![MoveType::Fly]);
    }
    fn is_chasm(tile: Tile) {
        assert!(!tile.opaque);
        assert!(tile.access == set![MoveType::Fly]);
    }

    #[test]
    fn wall_template() {
        let tile = Tile::wall();

        // assert that it's a wall
        is_wall(tile);
    }

    #[test]
    fn floor_template() {
        let tile = Tile::floor();

        // assert that it's a floor
        is_floor(tile);
    }

    #[test]
    fn water_template() {
        let tile = Tile::water();

        // assert that it's water
        is_water(tile);
    }

    #[test]
    fn lava_template() {
        let tile = Tile::lava();

        // assert that it's lava
        is_lava(tile);
    }
    #[test]
    fn chasm_template() {
        let tile = Tile::chasm();

        // assert that it's a chasm
        is_chasm(tile);
    }

    #[test]
    fn tile_enterable_with_one_matching_movtype() {
        let custom_tile = Tile::floor();

        assert!(custom_tile.can_enter(&[MoveType::Walk]));
    }

    #[test]
    fn tile_enterable_with_multiple_matching_movtype() {
        let custom_tile = Tile::floor();

        assert!(custom_tile.can_enter(&[MoveType::Walk, MoveType::Fly]));
    }

    #[test]
    fn tile_enterable_with_matching_and_unmatching_movtype() {
        let custom_tile = Tile::floor();

        assert!(custom_tile.can_enter(&[MoveType::Walk, MoveType::Swim]));
    }

    #[test]
    fn movetypes_can_be_directly_added_to_tiles() -> Result<(), String> {
        let mut tile = Tile::wall();
        tile.add_movetype(MoveType::custom("dig"));

        assert!(tile.can_enter(&[MoveType::custom("dig")]));
        Ok(())
    }

    #[test]
    fn tile_enterable_with_custom_movetype() {
        let custom_tile = Tile::new("softwall", false, [MoveType::custom("dig")]);

        assert!(custom_tile.can_enter(&[MoveType::custom("dig")]));
    }

    #[test]
    fn custom_movetypes_are_case_insensitive() {
        let mut tile = Tile::wall();
        tile.add_movetype(MoveType::custom("Dig"));

        assert!(tile.can_enter(&[MoveType::Custom("dig".to_string())]));
        assert!(tile.can_enter(&[MoveType::Custom("Dig".to_string())]));
    }

    #[test]
    fn movetypes_are_order_insensitive() {
        let mut tile1 = Tile::wall();
        let mut tile2 = Tile::wall();

        tile1.add_movetype(MoveType::Fly);
        tile1.add_movetype(MoveType::Walk);

        tile2.add_movetype(MoveType::Walk);
        tile2.add_movetype(MoveType::Fly);

        assert_eq!(tile1, tile2);

        let mut tile3 = Tile::wall();
        let mut tile4 = Tile::wall();

        tile3.add_movetypes([MoveType::Fly, MoveType::Walk]);
        tile4.add_movetypes([MoveType::Walk, MoveType::Fly]);

        assert_eq!(tile3, tile4);
    }

    #[test]
    fn tile_not_enterable() {
        let custom_tile = Tile::floor();
        let res = custom_tile.can_enter(&[MoveType::Swim]);

        assert!(!res);
    }

    // TileBuilder tests
    #[test]
    #[should_panic]
    fn builder_checks_initialization() {
        TileBuilder::new().build().unwrap();
    }

    #[test]
    fn builder_templates() -> Result<(), String> {
        assert_eq!(Tile::wall(), TileBuilder::wall().build()?);
        assert_eq!(Tile::floor(), TileBuilder::floor().build()?);
        assert_eq!(Tile::water(), TileBuilder::water().build()?);
        assert_eq!(Tile::lava(), TileBuilder::lava().build()?);
        assert_eq!(Tile::chasm(), TileBuilder::chasm().build()?);
        Ok(())
    }

    #[test]
    fn tile_config_works() -> Result<(), String> {
        let newtile: Tile = TileBuilder::new()
            .with_kind("slime")
            .with_opacity(false)
            .with_access(&[
                MoveType::Fly,
                MoveType::Swim,
                MoveType::Custom(String::from("liquidwalk")),
            ])
            .build()?;

        assert_eq!(newtile.kind, String::from("slime"));
        assert_eq!(newtile.opaque, false);

        let mut expected_access = HashSet::new();
        expected_access.insert(MoveType::Fly);
        expected_access.insert(MoveType::Swim);
        expected_access.insert(MoveType::Custom(String::from("liquidwalk")));

        assert_eq!(newtile.access(), &expected_access);
        Ok(())
    }
}
