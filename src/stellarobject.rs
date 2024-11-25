//! Module pour la gestion des objets stellaires dans le jeu Asteroids.
//!
//! Ce module définit le trait `StellarObject` qui représente tout objet stellaire dans le jeu, 
//! ainsi que son implémentation pour les astéroïdes (`Asteroid`), les missiles (`Missile`) et les vaisseaux spatiaux (`Spaceship`).

use macroquad::prelude::*;

use crate::{asteroid::Asteroid, missile::Missile, spaceship::Spaceship};

/// Trait représentant un objet stellaire dans le jeu.
///
/// Un objet stellaire est défini par sa position, sa taille et sa capacité à détecter les collisions avec d'autres objets stellaires.
pub trait StellarObject {
    /// Retourne la position de l'objet stellaire.
    fn get_position(&self) -> Vec2;
    /// Retourne la taille de l'objet stellaire.
    fn get_size(&self) -> f32;
    /// Détecte une collision avec un autre objet stellaire.
    ///
    /// # Paramètres
    /// - `other`: Un autre objet stellaire à vérifier.
    ///
    /// # Retour
    /// `true` si une collision est détectée, sinon `false`.
    fn collide(&self, other: &dyn StellarObject) -> bool;
}

/// Implémentation du trait `StellarObject` pour les astéroïdes.
impl StellarObject for Asteroid {
    fn get_position(&self) -> Vec2 {
        self.get_position()
    }

    fn get_size(&self) -> f32 {
        self.get_size()
    }

    /// Vérifie si l'astéroïde entre en collision avec un autre objet stellaire.
    fn collide(&self, other: &dyn StellarObject) -> bool {
        let other_position = other.get_position();
        let other_size = other.get_size();
        let distance = self.get_position().distance(other_position);

        distance < self.get_size() / 2.0 + other_size / 2.0
    }
}

/// Implémentation du trait `StellarObject` pour les missiles.
impl StellarObject for Missile {
    fn get_position(&self) -> Vec2 {
        self.get_position()
    }

    fn get_size(&self) -> f32 {
        Self::SIZE
    }

    /// Vérifie si le missile entre en collision avec un autre objet stellaire.
    fn collide(&self, other: &dyn StellarObject) -> bool {
        let other_position = other.get_position();
        let other_size = other.get_size();
        let distance = self.get_position().distance(other_position);

        distance < self.get_size() / 2.0 + other_size / 2.0
    }
}

/// Implémentation du trait `StellarObject` pour les vaisseaux spatiaux.
impl StellarObject for Spaceship {
    fn get_position(&self) -> Vec2 {
        self.get_position()
    }

    fn get_size(&self) -> f32 {
        Self::SIZE
    }

    /// Vérifie si le vaisseau spatial entre en collision avec un autre objet stellaire.
    fn collide(&self, other: &dyn StellarObject) -> bool {
        let other_position = other.get_position();
        let other_size = other.get_size();
        let distance = self.get_position().distance(other_position);

        distance < self.get_size() / 2.0 + other_size / 2.0
    }
}

