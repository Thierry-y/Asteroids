use std::f32::consts::PI;

use macroquad::prelude::*;
use ::rand::{thread_rng, Rng};

pub struct Asteroid {
    position : Vec2,
    speed : Vec2,
}

impl Asteroid {
    pub const ASTEROID_INIT_SIZE : f32 = 60.0;

    pub fn new() -> Self {
        Self {
            position: Self::new_alea_pos(),
            speed : Self::new_alea_speed(),
        }
    }

    pub fn get_position(&self) -> Vec2 {
        self.position
    }

    pub fn move_object(&mut self) -> Vec2{
        self.position += self.speed;
        self.position = Self::bound_pos(self.position);
        self.position
    }

    /// Génère une position aléatoire près de l'un des bords.
    fn new_alea_pos() -> Vec2 {
        let mut rng = thread_rng();

        let nearpos : f32 = rng.gen_range(Self::ASTEROID_INIT_SIZE/2.0..=Self::ASTEROID_INIT_SIZE);
        let nearside = rng.gen_range(1..=4); // 1 = top, 2 = right, 3 = down, 4 = left
        let xpos : f32 = match nearside {
            2 => screen_width() - nearpos,
            4 => nearpos,
            _ => rng.gen_range(0.0..=screen_width()),
        };
        let ypos : f32 = match nearside {
            1 => nearpos,
            3 => screen_height() - nearpos,
            _ => rng.gen_range(0.0..=screen_height()),
        };
        vec2(xpos, ypos)
    }

    fn new_alea_speed() -> Vec2 {
        let mut rng = thread_rng();

        let angle : f32 = rng.gen_range(0.0..=(2.0 * PI));
        Vec2::from_angle(angle)
    }

    fn bound_pos(mut pos : Vec2) -> Vec2 {
        pos.x = Self::bound_to(pos.x, screen_width());
        pos.y = Self::bound_to(pos.y, screen_height());
        pos
    }

    fn bound_to(coord : f32, max : f32) -> f32 {
        if coord < 0.0 { max - coord }
        else if coord > max { coord - max }
        else { coord }
    }
}
