
pub struct Rect {
    pub pos: (f32, f32),
    pub size: (f32, f32)
}

impl Rect {
    pub fn collides_with(&self, other: &Rect) -> bool {
        self.test_collision(other) || other.test_collision(self)
    }

    // Internal use only.
    fn test_collision(&self, other: &Rect) -> bool {
        let between_x = (self.left() < other.left() && self.right() > other.left()) ||
            (self.left() < other.right() && self.right() > other.right());

        let between_y = (self.top() < other.top() && self.bottom() > other.top()) ||
            (self.top() < other.bottom() && self.bottom() > other.bottom());

        between_x && between_y
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
