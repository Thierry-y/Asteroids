use std::f32::consts::PI;

use ::rand::{thread_rng, Rng};
use macroquad::prelude::*;

pub struct Asteroid {
    position: Vec2,
    speed: Vec2,
    size: f32,
}

impl Asteroid {
    pub const LARGE: f32 = 60.0;
    pub const MEDIUM: f32 = 30.0;
    pub const SMALL: f32 = 10.0;

    pub fn new() -> Self {
        Self {
            position: Self::new_alea_pos(),
            speed: Self::new_alea_speed(),
            size: Self::LARGE,
        }
    }

    //Créer des astéroïdes de taille connue à l'endroit souhaité.
    pub fn with_size(size: f32, position: Vec2) -> Self {
        Self {
            position,
            speed: Self::new_alea_speed(),
            size,
        }
    }

    pub fn get_position(&self) -> Vec2 {
        self.position
    }

    pub fn get_size(&self) -> f32 {
        self.size
    }

    pub fn move_object(&mut self) -> Vec2 {
        self.position += self.speed;
        self.position = Self::bound_pos(self.position);
        self.position
    }

    pub fn split(&self) -> Vec<Asteroid> {
        let new_size = match self.size {
            Self::LARGE => Self::MEDIUM,
            Self::MEDIUM => Self::SMALL,
            _ => return vec![],
        };

        vec![
            Asteroid::with_size(new_size, self.position + vec2(20.0, 20.0)),
            Asteroid::with_size(new_size, self.position + vec2(-20.0, -20.0)),
        ]
    }

    /// Génère une position aléatoire près de l'un des bords.
    fn new_alea_pos() -> Vec2 {
        let mut rng = thread_rng();

        let nearpos: f32 = rng.gen_range(Self::LARGE / 2.0..=Self::LARGE);
        let nearside = rng.gen_range(1..=4); // 1 = top, 2 = right, 3 = down, 4 = left
        let xpos: f32 = match nearside {
            2 => screen_width() - nearpos,
            4 => nearpos,
            _ => rng.gen_range(0.0..=screen_width()),
        };
        let ypos: f32 = match nearside {
            1 => nearpos,
            3 => screen_height() - nearpos,
            _ => rng.gen_range(0.0..=screen_height()),
        };
        vec2(xpos, ypos)
    }

    fn new_alea_speed() -> Vec2 {
        let mut rng = thread_rng();

        let angle: f32 = rng.gen_range(0.0..=(2.0 * PI));
        Vec2::from_angle(angle)
    }

    fn bound_pos(mut pos: Vec2) -> Vec2 {
        pos.x = Self::bound_to(pos.x, screen_width());
        pos.y = Self::bound_to(pos.y, screen_height());
        pos
    }

    fn bound_to(coord: f32, max: f32) -> f32 {
        if coord < 0.0 {
            max - coord
        } else if coord > max {
            coord - max
        } else {
            coord
        }
    }
}
