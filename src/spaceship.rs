use macroquad::prelude::*;
// use std::f32::consts::PI;

pub struct Spaceship {
    position: Vec2,
    velocity: Vec2,
    rotation: f32,
    push: bool,
    texture: Texture2D,
}

impl Spaceship {
    pub const SIZE: f32 = 60.0;
    pub const ROTATION_SPEED: f32 = 0.05;
    pub const SPEED: f32 = 0.1;

    pub fn new(texture: Texture2D) -> Self {
        Self {
            position: vec2(screen_width() / 2.0, screen_height() / 2.0),
            velocity: vec2(0.0, 0.0),
            rotation: 0.0,
            push: false,
            texture,
        }
    }

    pub fn update(&mut self) {
        if self.push {
            let velocity_vector = Vec2::from_angle(self.rotation) * Self::SPEED;
            self.velocity += velocity_vector;
        }

        self.position += self.velocity;
        self.position = Self::bound_pos(self.position);

        // Si aucun nouveau vecteur de vitesse n'est ajouté,
        // la vitesse est multipliée par 0,99 à chaque fois pour provoquer un ralentissement progressif, simulant l'inertie du vaisseau spatial.
        self.velocity *= 0.99;
    }

    pub fn get_position(&self) -> Vec2 {
        self.position
    }

    pub fn get_direction(&self) -> f32 {
        self.rotation
    }

    pub fn set_push(&mut self, push: bool) {
        self.push = push;
    }

    pub fn rotate_left(&mut self) {
        self.rotation -= Self::ROTATION_SPEED;
    }

    pub fn rotate_right(&mut self) {
        self.rotation += Self::ROTATION_SPEED;
    }

    pub fn draw(&self) {
        draw_texture_ex(
            &self.texture,
            self.position.x - Self::SIZE / 2.0,
            self.position.y - Self::SIZE / 2.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(Self::SIZE, Self::SIZE)), 
                rotation: self.rotation, 
                pivot: Some(self.position),
                ..Default::default()
            },
        );
        
        // let direction = Vec2::from_angle(self.rotation);
        // let left_wing = Vec2::from_angle(self.rotation + 3.0 * PI / 4.0) * Self::SIZE / 2.0;
        // let right_wing = Vec2::from_angle(self.rotation - 3.0 * PI / 4.0) * Self::SIZE / 2.0;

        // draw_triangle_lines(
        //     self.position + direction * Self::SIZE,
        //     self.position + left_wing,
        //     self.position + right_wing,
        //     2.0,
        //     WHITE,
        // );
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
