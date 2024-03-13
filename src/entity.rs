use sdl2::rect::Point;
use sdl2::rect::Rect;

const MULT: f64 = 180.0;
const GRAVITY_ACC: f64 = 9.81;
const JUMP_FORCE: f64 = -3.0;
const MAX_VEL: f64 = 3.5;

#[derive(Clone, Copy)]
pub struct Entity {
    pub rect: Rect,
    pub x: f64,
    pub y: f64,
    x_vel: f64,
    y_vel: f64,
    gravity: bool,
}

impl Entity {
    pub fn new(
        x: f64,
        y: f64,
        width: u32,
        height: u32,
        x_vel: f64,
        y_vel: f64,
        gravity: bool,
    ) -> Self {
        Self {
            rect: Rect::from_center(Point::new(x as i32, y as i32), width, height),
            x,
            y,
            x_vel,
            y_vel,
            gravity,
        }
    }

    fn update_rect(&mut self) {
        let center = Point::new(self.x.floor() as i32, self.y.floor() as i32);
        self.rect = Rect::from_center(center, self.rect.width(), self.rect.height())
    }

    pub fn update_position(&mut self, delta_time: f64) {
        if self.gravity {
            self.y_vel += GRAVITY_ACC * delta_time;
        }
        if self.y_vel > MAX_VEL {
            self.y_vel = MAX_VEL;
        }
        self.y += self.y_vel * delta_time * MULT;
        self.x += self.x_vel * delta_time * MULT;

        self.update_rect();
    }

    pub fn jump(&mut self) {
        self.y_vel = JUMP_FORCE;
    }

    pub fn stop(&mut self) {
        self.x_vel = 0.0;
        self.y_vel = 0.0;
    }
}
