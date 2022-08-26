//! Module containing the Generator structs

// use std::collections::HashMap;

use std::collections::HashSet;

use bracket_geometry::prelude::*;
use bracket_pathfinding::prelude::*;

use crate::genalgs;
use genalgs::rooms::*;

use super::errors::BuilderError;

use labyrinth_map::prelude::*;

#[derive(Debug)]
pub enum FloorGenAlg {
    Basic, // Rooms and Corridors
}

/// Builder struct for 2D Maps
///
/// # Example Usage
/// ```rust
/// use daedalus::prelude::*;
///
/// let mut mapgen = MapGenerator2D::new(80,50);
/// let floor1 = mapgen.generate(FloorGenAlg::Basic);
/// assert!(floor1.is_ok());
///
/// let floor2 = mapgen.generate(FloorGenAlg::Basic);
/// assert!(floor2.is_ok());
///
/// let floor3 = mapgen.generate(FloorGenAlg::Basic);
/// assert!(floor3.is_ok());
/// ```
pub struct MapGenerator2D {
    map: Labyrinth2D,
    rooms: CompoundRoom,
    dimensions: Point,
    dirty: bool,
}

impl MapGenerator2D {
    // ------------------ Initialization Methods ----------------------
    /// Creates a new Generator struct using width and height inputs
    pub fn new(width: usize, height: usize) -> MapGenerator2D {
        MapGenerator2D {
            map: Labyrinth2D::new(width, height),
            rooms: CompoundRoom::new(),
            dimensions: Point::new(width, height),
            dirty: false,
        }
    }

    // ----------------- Access Methods ---------------------
    /// Retrieves a reference to the internal [`Labyrinth2D`] of the Generator
    pub fn map(&self) -> &Labyrinth2D {
        &self.map
    }

    /// Retrieves a mutable reference to the internal [`Labyrinth2D`] of the Generator
    pub fn map_mut(&mut self) -> &mut Labyrinth2D {
        &mut self.map
    }

    pub fn map_iter_mut(&mut self) -> core::slice::IterMut<Tile> {
        self.map.iter_mut()
    }

    pub fn connections(&self) -> &HashSet<Point> {
        &self.rooms.connections
    }

    pub fn rooms(&self) -> &CompoundRoom {
        &self.rooms
    }

    pub fn dimensions(&self) -> &Point {
        &self.dimensions
    }

    // ----------------- Generation Methods -------------------------
    /// Generates a FinishedMap using the current settings.
    pub fn generate(&mut self, method: FloorGenAlg) -> Result<Labyrinth2D, BuilderError> {
        // Start with a new map
        self.flush_map();

        // Figure out the correct way to build the map
        match method {
            FloorGenAlg::Basic => {
                genalgs::build_rooms_and_corridors(self);
            }
            _ => {
                return Err(BuilderError::BuildError(format!(
                    "FloorGenAlg {:?} is unimplemented for this Generator",
                    method
                )))
            }
        };

        Ok(self.map.clone())
    }

    /// Resets the internal [`Labyrinth2D`] to a complely filled-in map
    pub fn flush_map(&mut self) {
        self.map = Labyrinth2D::new_from_dims(self.dimensions);
        self.rooms = CompoundRoom::new();
        self.dirty = true;
    }

    /// Resets the internal [`Labyrinth2D`] to an open map with walls
    pub fn walled_map(&mut self) {
        self.map = Labyrinth2D::new_walled_from_dims(self.dimensions);
    }

    /// adds a single room to the internal map
    pub fn add_room<T: Room + 'static>(&mut self, room: T) {
        self.rooms.rooms_mut().push(Box::new(room));
        self.dirty = true;
    }

    pub fn extend_rooms(&mut self, newrooms: Vec<Box<dyn Room>>) {
        self.rooms.rooms_mut().extend(newrooms);
        self.dirty = true;
    }

    pub fn add_compound_room(&mut self, croom: CompoundRoom) {
        self.rooms = croom;
        // for room in croom.rooms {
        //     self.rooms.rooms.push(room);
        // }
        // for pt in croom.connections {
        //     self.rooms.connections.insert(pt);
        // }
    }

    /// ataches a single room to the internal map
    pub fn attach_room<T: RoomCollisions + 'static>(&mut self, room: T, connection: Point) {
        self.rooms.attach_room(room, connection);
        //     push(Box::new(room));
        // self.rooms.connections.insert(connection);
        self.dirty = true;
    }

    /// flashes the contents of the rooms to the internal map,
    /// but only if it's been updated since
    pub fn update_rooms(&mut self) {
        if self.dirty {
            for room in self.rooms.rooms() {
                for &floortile in room.floor().iter() {
                    if self.map.in_bounds(floortile) {
                        self.map.set_tile_at(floortile, Tile::floor());
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
