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
        let x = !(pos.x < 0.0 || pos.x > screen_width());
        let y = !(pos.y < 0.0 || pos.y > screen_height());
        x & y
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_missile_new() {
        let spaceship_position = Vec2::new(100.0, 200.0);
        let spaceship_rotation = std::f32::consts::PI / 4.0;
        let missile = Missile::new(spaceship_position, spaceship_rotation);
        assert_eq!(missile.position, spaceship_position);

        let expected_velocity = Vec2::from_angle(spaceship_rotation) * Missile::SPEED;
        assert!((missile.velocity.x - expected_velocity.x).abs() < f32::EPSILON);
        assert!((missile.velocity.y - expected_velocity.y).abs() < f32::EPSILON);

        assert!(missile.active);
    }

    // Les tests macroquad::test dans missile et spaceship ne peuvent pas coexister en même temps. Si l'on commente l'un des deux,
    // les tests se déroulent correctement, mais si les deux sont actifs, l'un d'entre eux échoue. Cela pourrait être dû à
    // une erreur causée par l'asynchronisme de macroquad.

    // #[macroquad::test]
    // async fn test_missile_update() {
    //     let spaceship1_position = Vec2::new(100.0,100.0);
    //     let spaceship1_rotation = 0.0;
    //     let mut missile = Missile::new(spaceship1_position, spaceship1_rotation);
    //     missile.update();
    //     let expected1_position = spaceship1_position + Vec2::from_angle(spaceship1_rotation) * Missile::SPEED;
    //     assert_eq!(missile.position, expected1_position);

    //     assert!(missile.is_active());

    //     missile.position = Vec2::new(-10.0, -10.0);
    //     missile.update();
    //     assert!(!missile.is_active());

    //     missile.position = Vec2::new(screen_width() - 1.0, screen_height() - 1.0);
    //     missile.update();
    //     assert!(!missile.is_active());
    // }
}
