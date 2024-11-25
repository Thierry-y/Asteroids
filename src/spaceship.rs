//! Module de gestion du vaisseau spatial pour un jeu Asteroids.
//!
//! Ce module définit la structure et le comportement d'un vaisseau spatial, 
//! incluant sa position, sa vitesse, sa rotation, et sa gestion des bordures d'écran. 
//! Le vaisseau peut être contrôlé pour avancer, tourner à gauche ou à droite.

use macroquad::prelude::*;

/// Représente un vaisseau spatial dans le jeu.
pub struct Spaceship {
    /// Position actuelle du vaisseau spatial.
    position: Vec2,
    /// Vecteur de vitesse du vaisseau spatial.
    velocity: Vec2,
    /// Rotation actuelle du vaisseau spatial (en radians).
    rotation: f32,
    /// Indique si le vaisseau spatial est en poussée (propulsion).
    push: bool,
    /// Texture utilisée pour dessiner le vaisseau spatial.
    texture: Texture2D,
}

impl Spaceship {
    /// Taille du vaisseau spatial.
    pub const SIZE: f32 = 60.0;
    /// Vitesse de rotation du vaisseau spatial.
    pub const ROTATION_SPEED: f32 = 0.05;
    /// Intensité de la poussée appliquée au vaisseau spatial.
    pub const SPEED: f32 = 0.1;

    /// Crée un nouveau vaisseau spatial centré sur l'écran avec une texture donnée.
    ///
    /// # Paramètres
    /// - `texture`: La texture à utiliser pour dessiner le vaisseau spatial.
    ///
    /// # Retour
    /// Une nouvelle instance de `Spaceship`.
    pub fn new(texture: Texture2D) -> Self {
        Self {
            position: vec2(screen_width() / 2.0, screen_height() / 2.0),
            velocity: vec2(0.0, 0.0),
            rotation: 0.0,
            push: false,
            texture,
        }
    }

    /// Met à jour la position et la vitesse du vaisseau spatial.
    ///
    /// - Si la propulsion est activée (`push` est `true`), une nouvelle vitesse 
    ///   est ajoutée dans la direction de la rotation actuelle.
    /// - La position est ajustée pour rester dans les limites de l'écran.
    /// - Si aucune propulsion n'est appliquée, la vitesse diminue progressivement 
    ///   pour simuler l'inertie.
    pub fn update(&mut self) {
        if self.push {
            let velocity_vector = Vec2::from_angle(self.rotation) * Self::SPEED;
            self.velocity += velocity_vector;
        }

        self.position += self.velocity;
        self.position = Self::bound_pos(self.position);

        // Si aucun nouveau vecteur de vitesse n'est ajouté,
        // la vitesse est multipliée par 0,99 pour simuler un ralentissement progressif.
        self.velocity *= 0.99;
    }

    /// Retourne la position actuelle du vaisseau spatial.
    pub fn get_position(&self) -> Vec2 {
        self.position
    }

    /// Retourne la rotation actuelle du vaisseau spatial (en radians).
    pub fn get_direction(&self) -> f32 {
        self.rotation
    }

    /// Active ou désactive la propulsion du vaisseau spatial.
    ///
    /// # Paramètres
    /// - `push`: `true` pour activer la propulsion, `false` pour la désactiver.
    pub fn set_push(&mut self, push: bool) {
        self.push = push;
    }

    /// Tourne le vaisseau spatial vers la gauche.
    pub fn rotate_left(&mut self) {
        self.rotation -= Self::ROTATION_SPEED;
    }

    /// Tourne le vaisseau spatial vers la droite.
    pub fn rotate_right(&mut self) {
        self.rotation += Self::ROTATION_SPEED;
    }

    /// Dessine le vaisseau spatial à sa position actuelle avec la rotation et la texture définies.
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
    }

    /// Gère le retour de la position dans les limites de l'écran.
    ///
    /// # Paramètres
    /// - `pos`: La position à ajuster.
    ///
    /// # Retour
    /// La position ajustée pour qu'elle reste dans les limites de l'écran.
    fn bound_pos(mut pos: Vec2) -> Vec2 {
        pos.x = Self::bound_to(pos.x, screen_width());
        pos.y = Self::bound_to(pos.y, screen_height());
        pos
    }

    /// Vérifie et ajuste une coordonnée pour rester dans une limite donnée.
    ///
    /// # Paramètres
    /// - `coord`: La coordonnée à vérifier.
    /// - `max`: La limite maximale.
    ///
    /// # Retour
    /// La coordonnée ajustée pour rester dans les limites.
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
