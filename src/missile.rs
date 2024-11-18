use macroquad::prelude::*;

pub struct Missile {
    position: Vec2,
    velocity: Vec2,
    active: bool,
}

impl Missile {
    pub const SPEED: f32 = 5.0;
    pub const SIZE: f32 = 5.0;

    pub fn new(spaceship_position: Vec2, spaceship_rotation: f32) -> Self {
        let direction = Vec2::from_angle(spaceship_rotation);

        Self {
            position: spaceship_position,
            velocity: direction * Self::SPEED,
            active: true,
        }
    }

    pub fn update(&mut self) {
        if self.active {
            self.position += self.velocity;
            self.active = Self::bound_pos(self.position);
        }
    }

    pub fn draw(&self) {
        if self.active {
            draw_circle(self.position.x, self.position.y, Self::SIZE, RED);
        }
    }

    pub fn deactivate(&mut self) {
        self.active = false;
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn get_position(&self) -> Vec2 {
        self.position
    }

    fn bound_pos(pos: Vec2) -> bool {
        let x = if pos.x < 0.0 {
            false
        } else if pos.x > screen_width() {
            false
        } else {
            true
        };
        let y = if pos.y < 0.0 {
            false
        } else if pos.y > screen_height() {
            false
        } else {
            true
        };
        x & y
    }
}
