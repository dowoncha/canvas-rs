use types::PointF;
use std::cmp;

#[derive(Debug)]
pub struct Edge {
    pub slope: f32,
    pub x: f32,
    pub top: i32,
    pub bottom: i32
}

impl Edge {
    pub fn new(p1: PointF, p2: PointF) -> Edge {
        // Figure out the higher point
        let ref top_point = if p1.y < p2.y { p1 } else { p2 };
        let ref bot_point = if p1.y < p2.y { p2 } else { p1 };

        let slope = (top_point.x - bot_point.x) / (top_point.y - bot_point.y);
        
        let top = top_point.y.round();
        let bot = bot_point.y.round();

        let x = top_point.x + slope * (top as f32 + 0.5 - top_point.y);

        Edge {
            slope: slope,
            x: x,
            top: top as i32,
            bottom: bot as i32
        }
    }

    pub fn bottom_x(&self) -> f32 {
        self.x + (self.bottom - self.top) as f32
    }
    
    pub fn move_x(&mut self, dy: f32) {
        self.x += self.slope * dy;
    }

    pub fn pin_y(&mut self, height: i32) -> bool {
        if self.bottom < 0 || self.top >= height {
            return false;
        }

        if self.top < 0 {
            self.x = self.slope * self.top as f32;
            self.top = 0;
        }

        if self.bottom > height {
            self.bottom = height;
        }

        if self.slope.is_infinite() || self.slope.is_nan() || self.top == self.bottom {
            return false;
        }

        true
    }
}
