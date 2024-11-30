//! Module des astéroïdes pour un jeu Asteroids.
//!
//! Ce module gère la création, le déplacement et la division des astéroïdes.
//! Les astéroïdes apparaissent avec des tailles différentes et se déplacent
//! de manière aléatoire dans la fenêtre.

use ::rand::{thread_rng, Rng};
use macroquad::prelude::*;
use std::f32::consts::PI;

/// Représente un astéroïde dans le jeu.
pub struct Asteroid {
    /// Position actuelle de l'astéroïde.
    position: Vec2,
    /// Vitesse et direction de déplacement de l'astéroïde.
    speed: Vec2,
    /// Taille de l'astéroïde.
    size: f32,
}

impl Asteroid {
    /// Taille des astéroïdes grands.
    pub const LARGE: f32 = 60.0;
    /// Taille des astéroïdes moyens.
    pub const MEDIUM: f32 = 30.0;
    /// Taille des astéroïdes petits.
    pub const SMALL: f32 = 10.0;

    /// Crée un nouvel astéroïde avec une position et une vitesse aléatoires.
    ///
    /// # Retour
    /// Un nouveau `Asteroid` de taille `LARGE`.
    pub fn new() -> Self {
        Self {
            position: Self::new_alea_pos(),
            speed: Self::new_alea_speed(),
            size: Self::LARGE,
        }
    }

    /// Crée un astéroïde de taille spécifiée à une position donnée.
    ///
    /// # Paramètres
    /// - `size`: La taille de l'astéroïde (`LARGE`, `MEDIUM` ou `SMALL`).
    /// - `position`: La position initiale de l'astéroïde.
    ///
    /// # Retour
    /// Un nouveau `Asteroid` avec les attributs spécifiés.
    pub fn with_size(size: f32, position: Vec2) -> Self {
        Self {
            position,
            speed: Self::new_alea_speed(),
            size,
        }
    }

    /// Retourne la position actuelle de l'astéroïde.
    pub fn get_position(&self) -> Vec2 {
        self.position
    }

    /// Retourne la taille actuelle de l'astéroïde.
    pub fn get_size(&self) -> f32 {
        self.size
    }

    /// Déplace l'astéroïde en fonction de sa vitesse.
    ///
    /// # Retour
    /// La nouvelle position de l'astéroïde après déplacement.
    pub fn move_object(&mut self) -> Vec2 {
        self.position += self.speed;
        self.position = Self::bound_pos(self.position);
        self.position
    }

    /// Divise l'astéroïde en deux plus petits, s'il est possible de le diviser.
    ///
    /// # Retour
    /// Un `Vec` contenant deux nouveaux astéroïdes plus petits,
    /// ou un vecteur vide si l'astéroïde est déjà de la plus petite taille.
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

    /// Génère une position aléatoire proche d'un bord de l'écran.
    ///
    /// # Retour
    /// Un vecteur `Vec2` représentant la position aléatoire.
    fn new_alea_pos() -> Vec2 {
        let mut rng = thread_rng();

        let nearpos: f32 = rng.gen_range(Self::LARGE / 2.0..=Self::LARGE);
        let nearside = rng.gen_range(1..=4); // 1 = haut, 2 = droite, 3 = bas, 4 = gauche
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

    /// Génère une vitesse aléatoire basée sur un angle.
    ///
    /// # Retour
    /// Un vecteur `Vec2` représentant la vitesse.
    fn new_alea_speed() -> Vec2 {
        let mut rng = thread_rng();

        let angle: f32 = rng.gen_range(0.0..=(2.0 * PI));
        Vec2::from_angle(angle)
    }

    /// Assure que la position reste dans les limites de l'écran.
    ///
    /// # Paramètre
    /// - `pos`: La position à limiter.
    ///
    /// # Retour
    /// Une position ajustée pour rester dans les limites de l'écran.
    fn bound_pos(mut pos: Vec2) -> Vec2 {
        pos.x = Self::bound_to(pos.x, screen_width());
        pos.y = Self::bound_to(pos.y, screen_height());
        pos
    }

    /// Limite une coordonnée à une plage maximale.
    ///
    /// # Paramètres
    /// - `coord`: La coordonnée à limiter.
    /// - `max`: La valeur maximale permise.
    ///
    /// # Retour
    /// La coordonnée ajustée.
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

#[cfg(test)]
mod tests {
    use super::*;

    // Les fonctions test_asteroid_new() et test_asteroid_move() ne passent pas les tests car elles utilisent les fonctions
    // new_alea_pos() et bound_pos() qui appellent screen_width() et screen_height() de macroquad window,
    // mais ces deux valeurs ne peuvent pas être récupérées dans le test. Une solution pour que les tests passent est de remplacer
    // toutes les occurrences de screen_width() par 1920.0 et screen_height() par 1080.0 dans le code.

    // #[test]
    // fn test_asteroid_new() {
    //     let asteroid = Asteroid::new();
    //     println!("size: {:?}", asteroid.size);
    //     assert_eq!(asteroid.size, Asteroid::LARGE);
    //     assert!(asteroid.position.x >= 0.0 && asteroid.position.x <= screen_width());
    //     assert!(asteroid.position.y >= 0.0 && asteroid.position.y <= screen_height());
    // }

    #[test]
    fn test_asteroid_split_large() {
        let asteroid = Asteroid::with_size(Asteroid::LARGE, vec2(50.0, 50.0));
        let children = asteroid.split();
        assert_eq!(children.len(), 2);
        assert_eq!(children[0].size, Asteroid::MEDIUM);
        assert_eq!(children[1].size, Asteroid::MEDIUM);
    }

    #[test]
    fn test_asteroid_split_small() {
        let asteroid = Asteroid::with_size(Asteroid::SMALL, vec2(50.0, 50.0));
        let children = asteroid.split();
        assert!(children.is_empty());
    }

    // #[test]
    // fn test_asteroid_move() {
    //     let mut asteroid = Asteroid::new();
    //     let initial_position = asteroid.position;
    //     asteroid.move_object();
    //     assert_ne!(asteroid.position, initial_position);
    // }

    #[test]
    fn test_bound_to() {
        assert_eq!(Asteroid::bound_to(-10.0, 100.0), 110.0);
        assert_eq!(Asteroid::bound_to(110.0, 100.0), 10.0);
        assert_eq!(Asteroid::bound_to(50.0, 100.0), 50.0);
    }
}
