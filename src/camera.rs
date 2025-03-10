use crate::prelude::*;

pub struct Camera {
    pub left_x: i32,
    pub right_x: i32,
    pub top_y: i32,
    pub bottom_y: i32,
}

impl Camera {
    pub fn new(point: Point) -> Self {
        Self {
            left_x: point.x - DISPLAY_WIDTH / 2,
            right_x: point.x + DISPLAY_WIDTH / 2,
            top_y: point.y - DISPLAY_HEIGHT / 2,
            bottom_y: point.y + DISPLAY_HEIGHT / 2,
        }
    }

    pub fn on_player_move(&mut self, point: Point) {
        self.left_x = point.x - DISPLAY_WIDTH / 2;
        self.right_x = point.x + DISPLAY_WIDTH / 2;
        self.top_y = point.y - DISPLAY_HEIGHT / 2;
        self.bottom_y = point.y + DISPLAY_HEIGHT / 2;
    }
}
