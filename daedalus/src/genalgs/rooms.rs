use bracket_geometry::prelude::*;

use std::collections::HashSet;

use super::shapes;

pub trait Room {
    fn floor(&self) -> HashSet<Point>;
    fn borders(&self) -> HashSet<Point>;
    fn points(&self) -> HashSet<Point>;
    fn point_in_room(&self, pt: Point) -> bool;
    fn shift(&mut self, offset: Point);
    fn rotate_left(&mut self);
    fn rotate_right(&mut self);
    fn mirror(&mut self);
}

#[derive(Debug, PartialEq)]
struct RectRoom {
    internal: Rect,
}

impl RectRoom {
    fn new(w: i32, h: i32) -> RectRoom {
        RectRoom {
            internal: Rect::with_size(0, 0, w, h),
        }
    }
}

impl Room for RectRoom {
    fn floor(&self) -> HashSet<Point> {
        self.internal.point_set()
    }

    fn borders(&self) -> HashSet<Point> {
        let mut border = HashSet::new();

        // add walls and corners
        for x in self.internal.x1 - 1..=self.internal.x2 + 1 {
            border.insert(Point::new(x, self.internal.y1 - 1));
            border.insert(Point::new(x, self.internal.y2 + 1));
        }
        for y in self.internal.y1 - 1..=self.internal.y2 + 1 {
            border.insert(Point::new(self.internal.x1 - 1, y));
            border.insert(Point::new(self.internal.x2 + 1, y));
        }

        // remove door spaces?
        border
    }

    fn points(&self) -> HashSet<Point> {
        let mut points = self.floor();
        points.extend(&self.borders());
        points
    }

    fn point_in_room(&self, pt: Point) -> bool {
        self.internal.point_in_rect(pt)
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
        self.internal = Rect::with_exact(
            old.x1 + offset.x,
            old.y1 + offset.y,
            old.x2 + offset.x,
            old.y2 + offset.y,
        );
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
}
