use bracket_geometry::prelude::Point;

use std::collections::HashMap;

#[derive(Clone, Copy)]
pub enum TileType {
    Floor,
    Wall,
    Entrance,
    Exit,
}

pub struct FinishedMap {
    map: Vec<TileType>,
    width: usize,
    height: usize,
    // entities: Option<HashMap<String, Point>>,
    // TODO: allow the builder to populate the map
}

pub trait MapBuilder {
    fn build(&mut self) -> FinishedMap;
}

mod builder2d {
    use super::{FinishedMap, MapBuilder, TileType};

    /// Builder Class for 2D Maps
    ///
    pub struct MapBuilder2D {
        map: Vec<TileType>,
        width: usize,
        height: usize,
    }

    impl MapBuilder for MapBuilder2D {
        fn build(&mut self) -> FinishedMap {
            FinishedMap {
                map: self.map.to_owned(),
                width: self.width,
                height: self.height,
            }
        }
    }

    impl MapBuilder2D {
        fn new(width: usize, height: usize) -> MapBuilder2D {
            MapBuilder2D {
                map: vec![TileType::Floor; width*height],
                width: width,
                height: height,
            }
        }
    }
}
