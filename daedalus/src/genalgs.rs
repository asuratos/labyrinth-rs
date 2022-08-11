use bracket_pathfinding::prelude::{Algorithm2D, Point};
use labyrinth_map::prelude::Labyrinth2D;
use std::collections::HashSet;

use crate::map_generators::MapGenerator2D;
use labyrinth_map::prelude::MoveType;

pub mod rooms;
use rooms::*;
mod shapes;

fn is_fully_connected(map: &mut Labyrinth2D) -> bool {
    // TODO: Make this work for different kinds of move types?
    let movtype = MoveType::Walk;

    // get a set of walkable tiles
    let mut walkable: Vec<Point> = map
        .iter()
        .enumerate()
        .filter(|(_, tile)| tile.can_enter(&[movtype.clone()]))
        .map(|(i, _)| map.index_to_point2d(i))
        .collect();

    // see that every point is accessible from a point

    // get the last walkable point
    if let Some(pt) = walkable.pop() {
        // try to find a path to every other point from that one.

        if walkable
            .iter()
            .any(|&end| !map.find_path(pt, end, [movtype.clone()]).success)
        {
            return false;
        }
    } else {
        // Trivial case: if there are no walkable tiles at all
        return false;
    }

    true
}

fn attach_room<T: Room>(r1: CompoundRoom, r2: T) -> CompoundRoom {
    let possible_entrances = r1.walls();

    let possible_attachments = r2.walls(); //or entry points?

    r1
}

pub fn build_rooms_and_corridors(map: &mut MapGenerator2D) {
    // generate n rooms
    let n = 20;

    // start with a central small rectangle
    let mut firstroom = RectRoom::new(5, 5);
    firstroom.shift((map.map().dimensions() / 2) - Point::new(2, 2));

    let mut rooms = vec![firstroom];

    for _ in 0..n {
        // generate a rectangle room and a corridor
        let mut newroom = RectRoom::new(3, 3);
        let mut newhall = Hall::new_horizontal(1, 1);
        // try to attach each room to the map
    }

    // check that the map is fully connected
}
