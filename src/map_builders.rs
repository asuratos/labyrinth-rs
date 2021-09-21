use bracket_geometry::prelude::Point;

use std::collections::HashMap;


pub struct FinishedMap {
    map: Vec<u8>,
    width: u8,
    height: u8,
    start: Point,
    end: Point,
    // entities: Option<HashMap<String, Point>>,
    // TODO: allow the builder to populate the map
}

pub trait MapBuilder {
    fn build(self) -> FinishedMap;
}

mod builder2d {
    use super::{MapBuilder, FinishedMap, Point};

    /// Builder Class for 2D Maps
    ///
    pub struct MapBuilder2D { }

    impl MapBuilder for MapBuilder2D {
        fn build(self) -> FinishedMap {
            FinishedMap {
                map: vec![0],
                width: 80,
                height: 60,
                start: Point::new(0,0),
                end: Point::new(0,0)
            }
        }
    }
}
