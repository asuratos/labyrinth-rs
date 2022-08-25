use bracket_pathfinding::prelude::{Algorithm2D, Point};
use labyrinth_map::prelude::*;
use std::{collections::HashSet, iter::Map};

use rand::Rng;

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

fn apply_room_to_map<T: Room + ?Sized>(map: &mut Labyrinth2D, room: &T) {
    for &floortile in room.floor().iter() {
        if map.in_bounds(floortile) {
            map.set_tile_at(floortile, Tile::floor());
        }
    }
}

fn apply_compound_room_to_map(map: &mut Labyrinth2D, croom: CompoundRoom) {
    for room in croom.rooms {
        apply_room_to_map(map, &(*room));
    }

    for door in croom.connections {
        // TODO: door kind of tile?
        if map.in_bounds(door) {
            map.set_tile_at(door, Tile::floor());
        }
    }
}

fn room_in_bounds<T: Room>(mapgen: &mut MapGenerator2D, room: &T) -> bool {
    room.floor().iter().all(|&pt| mapgen.map().in_bounds(pt))
}

fn fit_room<T: RoomCollisions, U: Rng>(
    mapgen: &mut MapGenerator2D,
    rooms: &CompoundRoom,
    mut newroom: T,
    rng: &mut U,
) -> Option<(T, Point)> {
    let attempts = 10;

    // TODO: Make this seedable
    let mut rng = rand::thread_rng();

    // get attachment points of new room
    let attach_points = newroom.entries();

    // get attachment points (walls) of current compound room
    let walls = rooms.walls();

    // select an attachment point of new room
    let idx = rng.gen_range(0..attach_points.len());
    let attach_point_new = attach_points.iter().nth(idx).unwrap().clone();
    // println!("{:?}", newroom.floor());

    //bring the room to (0, 0) for correct transformations
    newroom.shift(attach_point_new * -1);

    // find a valid place to attach
    for _ in 0..attempts {
        let idx = rng.gen_range(0..walls.len());
        let attach_point_old = walls.iter().nth(idx).unwrap().clone();

        for _ in 0..5 {
            // TODO: randomize the transform here?
            // println!("{:?}", newroom.floor());
            newroom.rotate_right();

            newroom.shift(attach_point_old);

            if rooms.connects_to(&newroom) && room_in_bounds(mapgen, &newroom) {
                //if there's no collission with the rooms,
                // and the room is within bounds of the mapgen,
                // we return the room and the connection to it.
                // println!("Attach point: {:?}", attach_point_old);
                return Some((newroom, attach_point_old));
            }

            // back to (0, 0) for the next attempt
            newroom.shift(attach_point_old * -1);
        }
    }
    None
}

pub fn build_rooms_and_corridors(mapgen: &mut MapGenerator2D) {
    // generate n rooms
    let n = 20;

    // start with a central small rectangle
    let mut firstroom = RectRoom::new(5, 5);
    firstroom.shift((mapgen.map().dimensions() / 2) - Point::new(2, 2));

    let mut rooms = CompoundRoom::from_room(firstroom);

    // mapgen.add_room(firstroom);

    let mut rng = rand::thread_rng();
    for _ in 0..n {
        // while true { // version where it tries to fill the room
        // generate a rectangle room or a corridor
        // let newroom = if rng.gen::<f32>() > 0.5 {
        //     CompoundRoom::from_room(Hall::new_horizontal(5, 1))
        // } else {
        //     CompoundRoom::from_room(RectRoom::new(3, 3))
        // };

        let w = rng.gen_range(3..11);
        let h = rng.gen_range(3..7);

        let newroom = CompoundRoom::from_room(RectRoom::new(w, h));

        // try to attach each room to the map
        if let Some((newroom, connection)) = fit_room(mapgen, &rooms, newroom, &mut rng) {
            rooms.attach_room(newroom, connection);
        } else {
            break;
        }
    }

    // apply rooms to the map
    // apply_compound_room_to_map(mapgen.map_mut(), rooms);
    mapgen.add_compound_room(rooms);
    mapgen.update_rooms();

    // println!("{:?}", mapgen.rooms());

    // TODO: check that the map is fully connected
}
