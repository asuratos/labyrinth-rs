//! This module holds the [`Tile`] and [`TileBuilder`] structs.
//!
//!

#[cfg(feature = "serialization")]
use serde::{Serialize, Deserialize};

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
    pub fn custom(movtype: &str) -> MoveType {
        let lcase = movtype.to_lowercase();

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
    pub kind: String,

    /// Whether or not the tile blocks vision.
    pub opaque: bool,

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
    pub fn new(kind: &str, opaque: bool, access: HashSet<MoveType>) -> Tile {
        Tile {
            kind: kind.to_lowercase(),
            access,
            opaque,
        }
    }

    /// Direct constructor for a wall tile
    pub fn wall() -> Tile {
        Tile::new("wall", true, set![])
    }

    /// Direct constructor for a floor tile
    pub fn floor() -> Tile {
        Tile::new("floor", false, set![MoveType::Walk, MoveType::Fly])
    }

    /// Quick constructor for a water tile
    pub fn water() -> Tile {
        Tile::new("water", false, set![MoveType::Swim, MoveType::Fly])
    }

    /// Direct constructor for a lava tile
    pub fn lava() -> Tile {
        Tile::new("lava", false, set![MoveType::Fly])
    }

    /// Direct constructor for a chasm tile
    pub fn chasm() -> Tile {
        Tile::new("chasm", false, set![MoveType::Fly])
    }

    /// Adds a custom movement property to the tile. Whether or not it can be
    /// accessed via a user-defined movement method.
    /// (i.e. lava walking, digging, etc.)
    pub fn add_movetype(&mut self, movtype: &MoveType) -> bool {
        match movtype {
            MoveType::Custom(str) => self.access.insert(MoveType::custom(str)),
            _ => self.access.insert(movtype.clone()),
        }
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
    pub fn can_enter(&self, move_types: &[MoveType]) -> bool {
        move_types
            .iter()
            .map(|move_type| match move_type {
                MoveType::Custom(kind) => MoveType::custom(&kind.clone()),
                _ => move_type.clone(),
            })
            .any(|move_type| self.access.contains(&move_type))
    }

    /// Returns the accessibility of a tile
    pub fn accessbility(&self) -> &HashSet<MoveType> {
        &self.access
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
        tile.add_movetype(&MoveType::custom("dig"));

        assert!(tile.can_enter(&[MoveType::Custom("dig".to_string())]));
        Ok(())
    }

    #[test]
    fn tile_enterable_with_custom_movetype() {
        let custom_tile = Tile::new("softwall", false, set![MoveType::custom("dig")]);

        assert!(custom_tile.can_enter(&[MoveType::Custom("dig".to_string())]));
    }

    #[test]
    fn custom_movetypes_are_case_insensitive() {
        let mut tile = Tile::wall();
        tile.add_movetype(&MoveType::custom("Dig"));

        assert!(tile.can_enter(&[MoveType::Custom("dig".to_string())]));
        assert!(tile.can_enter(&[MoveType::Custom("Dig".to_string())]));
    }

    #[test]
    fn tile_not_enterable() {
        let custom_tile = Tile::floor();
        let res = custom_tile.can_enter(&[MoveType::Swim]);

        assert!(!res);
    }
}
