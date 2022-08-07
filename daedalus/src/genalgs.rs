use bracket_pathfinding::prelude::{Algorithm2D, Point};
use labyrinth_map::prelude::Labyrinth2D;

use crate::map_generators::MapGenerator2D;
use labyrinth_map::prelude::MoveType;

pub mod rooms;
mod shapes;

fn is_fully_connected(map: &mut Labyrinth2D) -> bool {
    // TODO: Make this work for different kinds of move types?
    // get a set of walkable tiles
    let mut walkable: Vec<Point> = map
        .iter()
        .enumerate()
        .filter(|(_, tile)| tile.access().contains(&MoveType::Walk))
        .map(|(i, _)| map.index_to_point2d(i))
        .collect();

    // see that every point is accessible from a point

    // get the last walkable point
    if let Some(pt) = walkable.pop() {
        // try to find a path to every other point from that one.
        for target in walkable {
            let path = map.find_path(pt, target, [MoveType::Walk]);

            if !path.success {
                return false;
            }
        }
    } else {
        // Trivial case: if there are no walkable tiles at all
        return false;
    }

    true
}

pub fn build_rooms_and_corridors(map: &mut MapGenerator2D) {
    // generate n rooms
    let n = 20;
    for _ in 0..n {
        // generate a rectangle room and a corridor
        // try to attach each room to the map
    }

    // check that the map is fully connected
}
