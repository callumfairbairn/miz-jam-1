use crate::level::Side;
use crate::level::Side::{TOP, BOTTOM, LEFT, RIGHT};

pub struct Rect {
    pub pos: (f32, f32),
    pub size: (f32, f32)
}

impl Rect {
    pub fn collides_with(&self, other: &Rect) -> bool {
        self.test_collision(other) || other.test_collision(self)
    }

    pub fn get_nearest_wall(&self, other: &Rect) -> Side {
        let self_centre = self.centre();

        let distance_to_top = distance(self_centre, other.top_wall());
        let distance_to_bottom = distance(self_centre, other.bottom_wall());
        let distance_to_left = distance(self_centre, other.left_wall());
        let distance_to_right = distance(self_centre, other.right_wall());

        if distance_to_top <= distance_to_bottom && distance_to_top <= distance_to_left && distance_to_top <= distance_to_right { return TOP } ;
        if distance_to_bottom <= distance_to_top && distance_to_bottom <= distance_to_left && distance_to_bottom <= distance_to_right { return BOTTOM } ;
        if distance_to_left <= distance_to_top && distance_to_left <= distance_to_bottom && distance_to_left <= distance_to_right { return LEFT } ;
        RIGHT
    }

    fn centre(&self) -> (f32, f32) {
        (self.pos.0 + self.size.0 / 2.0, self.pos.1 + self.size.1 / 2.0)
    }

    fn top_wall(&self) -> (f32, f32) {
        (self.pos.0 + self.size.0 / 2.0, self.pos.1 + self.size.1)
    }

    fn bottom_wall(&self) -> (f32, f32) {
        (self.pos.0 + self.size.0 / 2.0, self.pos.1)
    }

    fn left_wall(&self) -> (f32, f32) {
        (self.pos.0, self.pos.1 + self.size.1 / 2.0)
    }

    fn right_wall(&self) -> (f32, f32) {
        (self.pos.0 + self.size.0, self.pos.1 + self.size.1 / 2.0)
    }

    // Internal use only.
    fn test_collision(&self, other: &Rect) -> bool {
        let between_x = (self.left() < other.left() && self.right() > other.left()) ||
            (self.left() < other.right() && self.right() > other.right());

        let between_y = (self.top() < other.top() && self.bottom() > other.top()) ||
            (self.top() < other.bottom() && self.bottom() > other.bottom());

        (between_x && between_y) || (between_x && self.top() == other.top() && self.bottom() == other.bottom())
            || (between_y && self.left() == other.left() && self.left() == other.left())
    }

    fn left(&self) -> f32 {
        self.pos.0
    }

    fn right(&self) -> f32 {
        self.pos.0 + self.size.0
    }

    fn top(&self) -> f32 {
        self.pos.1
    }

    fn bottom(&self) -> f32 {
        self.pos.1 + self.size.1
    }
}

fn distance(coord1: (f32, f32), coord2: (f32, f32)) -> f32 {
    ((coord1.0 - coord2.0).powf(2.0) + (coord1.1 - coord2.1).powf(2.0)).sqrt()
}
