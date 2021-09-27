use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::MoveType;

/// Builder struct for Tiles.
/// Will fail if required fields (everything except custom_properties) is None.
///
/// # Usage
/// ```rust
/// use labyrinth_rs::prelude::*;
///
/// // Building a Tile manually
/// let wall_manual = TileBuilder::new()
///     .kind("wall")
///     .opaque(true)
///     .walk(false)
///     .fly(false)
///     .swim(false)
///     .build()
///     .unwrap();
///
/// // Building a tile from a template
/// let wall_from_template = TileBuilder::wall() // starts from preset wall template
///     .kind("crystalwall")
///     .opaque(false)
///     .build()
///     .unwrap();
///
/// // Building a wall with a special access property
/// let phasable_wall = TileBuilder::wall()
///     .kind("phasebarrier")
///     .property("phase", true)
///     // This tile is accessible to any entity that can move via phasing
///     .build()
///     .unwrap();
/// ```
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
            .swim(false)
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

    pub fn chasm() -> TileBuilder {
        TileBuilder::new()
            .kind("chasm")
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
            Ok(Tile {
                kind: self.kind.unwrap(),
                opaque: self.opaque.unwrap(),
                walk: self.walk.unwrap(),
                fly: self.fly.unwrap(),
                swim: self.swim.unwrap(),
                other_movement: self.custom_properties,
            })
        }
    }
}

/// Tile struct that contains the accessibility information and its name.
///
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

    /// Check if an entity with the given move types can enter a tile.
    ///
    /// Returns an error if any of the movement types specified are not
    /// specified for the tile.
    pub fn can_enter(&self, move_types: &[MoveType]) -> Result<bool, String> {
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

        custom_tile.can_enter(&[MoveType::Walk])?;

        Ok(())
    }

    #[test]
    fn tile_enterable_with_multiple_matching_movtype() -> Result<(), String> {
        let custom_tile = TileBuilder::floor().kind("sometile").build()?;

        custom_tile.can_enter(&[MoveType::Walk, MoveType::Fly])?;

        Ok(())
    }

    #[test]
    fn tile_enterable_with_matching_and_unmatching_movtype() -> Result<(), String> {
        let custom_tile = TileBuilder::floor().kind("sometile").build()?;

        custom_tile.can_enter(&[MoveType::Walk, MoveType::Swim])?;

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
