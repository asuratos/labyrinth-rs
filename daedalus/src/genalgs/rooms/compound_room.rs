use super::*;
use rand::Rng;

#[derive(Debug, PartialEq)]
pub struct CompoundRoom {
    pub rooms: Vec<Box<dyn Room>>,
    pub connections: HashSet<Point>,
}

impl CompoundRoom {
    pub fn new() -> CompoundRoom {
        CompoundRoom {
            rooms: vec![],
            connections: HashSet::new(),
        }
    }

    pub fn from_room<T: Room + 'static>(room: T) -> CompoundRoom {
        CompoundRoom {
            rooms: vec![Box::new(room)],
            connections: HashSet::new(),
        }
    }

    // pub fn find_and_attach_room<T: RoomCollisions + 'static>(&mut self, room: T) -> bool {
    //     if let Some((room, conn)) = self.find_valid_attachment(room) {
    //         self.attach_room(room, conn);
    //         return true;
    //     }
    //     false
    // }

    pub fn attach_room<T: RoomCollisions + 'static>(&mut self, room: T, connection: Point) -> bool {
        if self.walls().contains(&connection) && self.connects_to(&room) {
            self.rooms.push(Box::new(room));
            self.connections.insert(connection);
            return true;
        }
        false
    }

    // pub fn find_valid_attachment<T: RoomCollisions>(&mut self, mut room: T) -> Option<(T, Point)> {
    //     let attempts = 20;
    //     let mut room_found = false;

    //     // TODO: Make this seedable
    //     let mut rng = rand::thread_rng();

    //     // get attachment points of new room
    //     let attach_points = room.entries();

    //     // get attachment points (walls) of current compound room
    //     let walls = self.walls();

    //     // find a valid place to attach
    //     for _ in 0..attempts {
    //         // attachment point of new room + any wall of current room
    //         let idx = rng.gen_range(0..attach_points.len());
    //         let attach_point_new = attach_points.iter().nth(idx).unwrap().clone();

    //         let idx = rng.gen_range(0..walls.len());
    //         let attach_point_old = walls.iter().nth(idx).unwrap().clone();

    //         //bring the room to (0, 0) for correct transformations
    //         room.shift(attach_point_new * -1);

    //         for _ in 0..3 {
    //             // TODO: randomize the transform here?
    //             room.rotate_right();

    //             room.shift(attach_point_old);

    //             if !self.collides_with(&room) {
    //                 //if there's no collission with the rooms, we can end here

    //                 return Some((room, attach_point_old));
    //             }

    //             // back to (0, 0) for the next attempt
    //             room.shift(attach_point_old * -1);
    //         }
    //     }

    //     None
    // }
}

impl Room for CompoundRoom {
    fn floor(&self) -> HashSet<Point> {
        let mut floor = self.rooms.iter().fold(HashSet::new(), |mut acc, room| {
            acc.extend(room.floor());
            acc
        });

        floor.extend(self.connections.iter());
        floor
    }

    fn borders(&self) -> HashSet<Point> {
        let floor = self.floor();
        let borders = self.rooms.iter().fold(HashSet::new(), |mut acc, room| {
            acc.extend(room.borders());
            acc
        });

        borders.difference(&floor).cloned().collect()
    }

    fn all_points(&self) -> HashSet<Point> {
        self.rooms.iter().fold(HashSet::new(), |mut acc, room| {
            acc.extend(room.all_points());
            acc
        })
    }

    fn walls(&self) -> HashSet<Point> {
        self.rooms.iter().fold(HashSet::new(), |mut acc, room| {
            acc.extend(room.walls());
            acc
        })
    }

    fn entries(&self) -> HashSet<Point> {
        self.rooms.iter().fold(HashSet::new(), |mut acc, room| {
            acc.extend(room.entries());
            acc
        })
    }

    fn point_in_room(&self, pt: Point) -> bool {
        self.rooms.iter().any(|r| r.point_in_room(pt))
    }

    fn mirror(&mut self) {
        self.rooms.iter_mut().for_each(|r| r.mirror());
    }

    fn rotate_left(&mut self) {
        self.rooms.iter_mut().for_each(|r| r.rotate_left());
    }

    fn rotate_right(&mut self) {
        self.rooms.iter_mut().for_each(|r| r.rotate_right());
    }

    fn shift(&mut self, offset: Point) {
        self.rooms.iter_mut().for_each(|r| r.shift(offset));
    }
}

impl RoomCollisions for CompoundRoom {}
