use bracket_geometry::prelude::*;

use std::{collections::HashSet, iter::FromIterator};

use super::shapes;

pub trait Room {
    fn floor(&self) -> HashSet<Point>;
    fn walls(&self) -> HashSet<Point>;
    fn borders(&self) -> HashSet<Point>;

    fn all_points(&self) -> HashSet<Point> {
        let mut all = self.floor();
        all.extend(&self.borders());
        all
    }

    fn point_in_room(&self, pt: Point) -> bool {
        self.floor().contains(&pt)
    }

    fn shift(&mut self, offset: Point);
    fn rotate_left(&mut self) {}
    fn rotate_right(&mut self) {}
    fn mirror(&mut self) {}
}

pub trait RoomCollisions: Room {
    fn collides_with<T: Room>(&self, other: &T) -> bool {
        !self.all_points().is_disjoint(&other.all_points())
    }
}

#[derive(Debug, PartialEq)]
struct RectRoom {
    internal: Rect,
}

impl RectRoom {
    pub fn new(w: i32, h: i32) -> RectRoom {
        // TODO: add checks to make sure w, h are > 0
        RectRoom {
            internal: Rect::with_size(0, 0, w, h),
        }
    }
}

impl RoomCollisions for RectRoom {}

impl Room for RectRoom {
    fn floor(&self) -> HashSet<Point> {
        self.internal.point_set()
    }

    fn borders(&self) -> HashSet<Point> {
        let mut border = self.walls();

        // add corners
        for x in [self.internal.x1 - 1, self.internal.x2 + 1] {
            for y in [self.internal.y1 - 1, self.internal.y2 + 1] {
                border.insert(Point::new(x, y));
            }
        }

        // remove door spaces?
        border
    }

    fn walls(&self) -> HashSet<Point> {
        let mut border = HashSet::new();

        // add walls
        for x in self.internal.x1..=self.internal.x2 {
            border.insert(Point::new(x, self.internal.y1 - 1));
            border.insert(Point::new(x, self.internal.y2 + 1));
        }
        for y in self.internal.y1..=self.internal.y2 {
            border.insert(Point::new(self.internal.x1 - 1, y));
            border.insert(Point::new(self.internal.x2 + 1, y));
        }

        border
    }

    fn mirror(&mut self) {
        // do nothing to the internals

        // mirror attached hallways?
    }

    fn rotate_left(&mut self) {
        let old = self.internal;
        self.internal = Rect::with_exact(old.y1, -old.x2, old.y2, -old.x1);
    }

    fn rotate_right(&mut self) {
        let old = self.internal;
        self.internal = Rect::with_exact(-old.y2, old.x1, -old.y1, old.x2);
    }

    fn shift(&mut self, offset: Point) {
        let old = self.internal;
        self.internal = Rect::with_size(
            old.x1 + offset.x,
            old.y1 + offset.y,
            old.width(),
            old.height(),
        );
    }
}

#[derive(Debug, PartialEq)]
struct Hall {
    start: Point,
    horizontal: bool,
    length: i32,
    thickness: i32,
}

impl Hall {
    pub fn new_horizontal(length: i32, thickness: i32) -> Hall {
        Hall {
            start: Point::new(0, 0),
            horizontal: true,
            length,
            thickness,
        }
    }

    pub fn new_vertical(length: i32, thickness: i32) -> Hall {
        Hall {
            start: Point::new(0, 0),
            horizontal: false,
            length,
            thickness,
        }
    }

    fn endpoint(&self) -> Point {
        let d: Point = if self.horizontal {
            Point::new(0, self.length)
        } else {
            Point::new(self.length, 0)
        };

        self.start + d
    }
}

impl RoomCollisions for Hall {}

impl Room for Hall {
    fn floor(&self) -> HashSet<Point> {
        let end = self.endpoint();

        HashSet::from_iter(line2d_bresenham(self.start, end).iter().cloned())
    }

    fn walls(&self) -> HashSet<Point> {
        let end = self.endpoint();
        let side_wall_starts: [Point; 2];
        let start_wall: Point;
        let end_wall: Point;

        if self.horizontal {
            side_wall_starts = [Point::new(0, 1), Point::new(0, -1)];
            if self.length < 0 {
                start_wall = Point::new(1, 0);
                end_wall = Point::new(-1, 0);
            } else {
                start_wall = Point::new(-1, 0);
                end_wall = Point::new(1, 0);
            }
        } else {
            side_wall_starts = [Point::new(1, 0), Point::new(-1, 0)];
            if self.length < 0 {
                start_wall = Point::new(0, 1);
                end_wall = Point::new(0, -1);
            } else {
                start_wall = Point::new(0, -1);
                end_wall = Point::new(0, 1);
            }
        };

        let mut walls = HashSet::new();
        for d in side_wall_starts {
            walls.extend(line2d_bresenham(self.start + d, end + d).iter())
        }
        walls.insert(self.start + start_wall);
        walls.insert(end + end_wall);
        walls
    }

    fn borders(&self) -> HashSet<Point> {
        let mut borders = self.walls();

        let diagonals = [
            Point::new(1, 1),
            Point::new(1, -1),
            Point::new(-1, 1),
            Point::new(-1, -1),
        ];

        for pt in [self.start, self.endpoint()] {
            for d in diagonals {
                borders.insert(pt + d);
            }
        }

        borders
    }

    fn shift(&mut self, offset: Point) {
        self.start += offset;
    }
}

#[cfg(test)]
mod tests {
    use super::{RectRoom, Room};

    fn rectroom_is_valid(rm: &RectRoom) {
        assert!(rm.internal.x1 < rm.internal.x2);
        assert!(rm.internal.y1 < rm.internal.y2);
    }

    #[test]
    fn rectroom_stays_valid_after_left_rotation() {
        let mut room = RectRoom::new(5, 5);
        for _ in 0..4 {
            room.rotate_left();
            rectroom_is_valid(&room);
        }
        // 4 rotations should always return to the original
        assert_eq!(room, RectRoom::new(5, 5));
    }

    #[test]
    fn rectroom_stays_valid_after_right_rotation() {
        let mut room = RectRoom::new(5, 5);
        for _ in 0..4 {
            room.rotate_right();
            rectroom_is_valid(&room);
        }
        // 4 rotations should always return to the original
        assert_eq!(room, RectRoom::new(5, 5));
    }

    #[test]
    fn rectroom_access_inside_borders() {
        let room = RectRoom::new(5, 5);

        assert!(room.walls().is_subset(&room.borders()));
    }
}
