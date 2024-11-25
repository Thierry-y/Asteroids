//! Module des missiles pour un jeu Asteroids.
//!
//! Ce module gère les missiles tirés par le vaisseau spatial. Les missiles ont une position,
//! une vitesse, et un état (actif ou inactif). Ils se déplacent dans la direction où le vaisseau pointe
//! et sont désactivés lorsqu'ils sortent des limites de l'écran.

use macroquad::prelude::*;

/// Représente un missile dans le jeu.
pub struct Missile {
    /// Position actuelle du missile.
    position: Vec2,
    /// Vitesse et direction de déplacement du missile.
    velocity: Vec2,
    /// État du missile (actif ou inactif).
    active: bool,
}

impl Missile {
    /// Vitesse du missile.
    pub const SPEED: f32 = 5.0;
    /// Taille visuelle du missile (rayon).
    pub const SIZE: f32 = 5.0;

    /// Crée un nouveau missile à la position et rotation actuelles du vaisseau spatial.
    ///
    /// # Paramètres
    /// - `spaceship_position`: La position actuelle du vaisseau spatial.
    /// - `spaceship_rotation`: L'angle de rotation du vaisseau (en radians).
    ///
    /// # Retour
    /// Un nouveau `Missile` initialisé avec la position et la direction du vaisseau spatial.
    pub fn new(spaceship_position: Vec2, spaceship_rotation: f32) -> Self {
        let direction = Vec2::from_angle(spaceship_rotation);

        Self {
            position: spaceship_position,
            velocity: direction * Self::SPEED,
            active: true,
        }
    }

    /// Met à jour la position du missile en fonction de sa vitesse.
    ///
    /// Si le missile dépasse les limites de l'écran, il est désactivé.
    pub fn update(&mut self) {
        if self.active {
            self.position += self.velocity;
            self.active = Self::bound_pos(self.position);
        }
    }

    /// Dessine le missile à sa position actuelle, si celui-ci est actif.
    pub fn draw(&self) {
        if self.active {
            draw_circle(self.position.x, self.position.y, Self::SIZE, RED);
        }
    }

    /// Désactive le missile (par exemple lorsqu'il touche une cible ou sort de l'écran).
    pub fn deactivate(&mut self) {
        self.active = false;
    }

    /// Vérifie si le missile est toujours actif.
    ///
    /// # Retour
    /// - `true` si le missile est actif.
    /// - `false` si le missile est inactif.
    pub fn is_active(&self) -> bool {
        self.active
    }

    /// Retourne la position actuelle du missile.
    pub fn get_position(&self) -> Vec2 {
        self.position
    }

    /// Vérifie si une position donnée est dans les limites de l'écran.
    ///
    /// # Paramètre
    /// - `pos`: La position à vérifier.
    ///
    /// # Retour
    /// - `true` si la position est dans les limites de l'écran.
    /// - `false` si la position est hors des limites.
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
