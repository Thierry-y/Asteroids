use macroquad::prelude::*;

use crate::{asteroid::Asteroid, missile::Missile, spaceship::Spaceship};

pub trait StellarObject {
    fn get_position(&self) -> Vec2;
    fn get_size(&self) -> f32;
    fn collide(&self, other: &dyn StellarObject) -> bool;
}

impl StellarObject for Asteroid {
    fn get_position(&self) -> Vec2 {
        self.get_position()
    }

    fn get_size(&self) -> f32 {
        self.get_size()
    }

    fn collide(&self, other: &dyn StellarObject) -> bool {
        let other_position = other.get_position();
        let other_size = other.get_size();
        let distance = self.get_position().distance(other_position);

        distance < self.get_size() / 2.0 + other_size / 2.0
    }
}

impl StellarObject for Missile {
    fn get_position(&self) -> Vec2 {
        self.get_position()
    }

    fn get_size(&self) -> f32 {
        Self::SIZE
    }

    fn collide(&self, other: &dyn StellarObject) -> bool {
        let other_position = other.get_position();
        let other_size = other.get_size();
        let distance = self.get_position().distance(other_position);

        distance < self.get_size() / 2.0 + other_size / 2.0
    }
}

impl StellarObject for Spaceship {
    fn get_position(&self) -> Vec2 {
        self.get_position()
    }

    fn get_size(&self) -> f32 {
        Self::SIZE
    }

    fn collide(&self, other: &dyn StellarObject) -> bool {
        let other_position = other.get_position();
        let other_size = other.get_size();
        let distance = self.get_position().distance(other_position);

        distance < self.get_size() / 2.0 + other_size / 2.0
    }
}
