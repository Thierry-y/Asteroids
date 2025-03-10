//! Jeu Asteroids développé avec Macroquad et Gamepads
//!
//! Ce programme implémente du jeu "Asteroids". Le joueur contrôle un vaisseau spatial pour éviter et détruire des astéroïdes en utilisant des missiles. Le jeu offre plusieurs niveaux de difficulté et détecte les collisions entre les objets (vaisseau, missiles, et astéroïdes).
//!
//! ## Contrôles clavier
//! - **Flèche gauche** : Tourner à gauche
//! - **Flèche droite** : Tourner à droite
//! - **Flèche haut** : Accélérer
//! - **Espace** : Tirer un missile
//! - **Échap** : Quitter le jeu
//!
//! ## Contrôles controller
//! - **Joystick gauche vers la gauche** : Tourner à gauche
//! - **Joystick gauche vers la droite** : Tourner à droite
//! - **Joystick gauche vers le haut** : Accélérer
//! - **R1** : Tirer un missile
//! - **B** : Quitter le jeu

use asteroid::Asteroid;
use gamepads::Gamepads;
use macroquad::prelude::*;
use missile::Missile;
use spaceship::Spaceship;
use std::thread;
use std::time::Duration;
use stellarobject::StellarObject; //sudo apt-get install libudev-dev

mod asteroid;
mod missile;
mod spaceship;
mod stellarobject;

/// Dessine l'état actuel du jeu.
///
/// Cela inclut le fond d'écran, les astéroïdes, le vaisseau spatial et les missiles.
///
/// # Paramètres
/// - `asteroids`: Liste des astéroïdes à dessiner.
/// - `spaceship`: Instance du vaisseau spatial.
/// - `missiles`: Liste des missiles actifs.
/// - `texture`: Texture de fond à utiliser.
fn draw(asteroids: &[Asteroid], spaceship: &Spaceship, missiles: &[Missile], texture: &Texture2D) {
    draw_background(texture);
    draw_asteroids(asteroids);
    spaceship.draw();
    for missile in missiles {
        missile.draw();
    }
}

/// Dessine le fond d'écran.
///
/// # Paramètres
/// - `background_texture`: La texture du fond.
fn draw_background(background_texture: &Texture2D) {
    draw_texture(background_texture, 0.0, 0.0, WHITE);
}

/// Affiche l'écran de fin de partie avec le texte "Game Over".
fn draw_game_over() {
    let screen_width = screen_width();
    let screen_height = screen_height();
    let font_size = screen_height * 0.1;
    draw_text(
        "Game Over",
        screen_width * 0.4,
        screen_height * 0.5,
        font_size,
        RED,
    );
}

/// Affiche l'écran de victoire avec le texte "You Win!".
fn draw_you_win() {
    let screen_width = screen_width();
    let screen_height = screen_height();
    let font_size = screen_height * 0.1;
    draw_text(
        "You Win!",
        screen_width * 0.4,
        screen_height * 0.5,
        font_size,
        GREEN,
    );
}

/// Affiche l'écran un affichage visuel bouclier.
fn draw_health_bar(health: f32) {
    let bar_width = 100.0;
    let bar_height = 20.0;
    let bar_x = 20.0;
    let bar_y = screen_height() - bar_height - 20.0;

    draw_rectangle(
        bar_x,
        bar_y,
        bar_width,
        bar_height,
        Color::from_rgba(50, 50, 50, 200),
    );

    let current_health_width = bar_width * health;
    draw_rectangle(bar_x, bar_y, current_health_width, bar_height, GREEN);
}

/// Dessine tous les astéroïdes de la liste.
///
/// # Paramètres
/// - `asteroids`: Liste des astéroïdes à dessiner.
fn draw_asteroids(asteroids: &[Asteroid]) {
    for asteroid in asteroids {
        draw_circle(
            asteroid.get_position().x,
            asteroid.get_position().y,
            asteroid.get_size() / 2.0,
            YELLOW,
        );
    }
}

/// Gère les entrées utilisateur pour contrôler le vaisseau et tirer des missiles.
///
/// # Paramètres
/// - `spaceship`: Référence mutable au vaisseau spatial.
/// - `missiles`: Liste des missiles actifs.
/// - `gamepads`: Référence mutable aux manettes pour gérer les entrées.
///
/// # Retour
/// `true` si l'utilisateur souhaite quitter le jeu, sinon `false`.
fn handle_input(
    spaceship: &mut Spaceship,
    missiles: &mut Vec<Missile>,
    gamepads: &mut Gamepads,
) -> bool {
    if is_key_down(KeyCode::Escape) {
        return true;
    }

    if is_key_down(KeyCode::Left) {
        spaceship.rotate_left();
    }
    if is_key_down(KeyCode::Right) {
        spaceship.rotate_right();
    }
    spaceship.set_push(is_key_down(KeyCode::Up));

    if is_key_pressed(KeyCode::Space) {
        let missile = Missile::new(spaceship.get_position(), spaceship.get_direction());
        missiles.push(missile);
    }

    gamepads.poll();

    for gamepad in gamepads.all() {
        let left_stick = gamepad.left_stick();
        if gamepad.is_just_pressed(gamepads::Button::ActionRight) {
            return true;
        }

        if left_stick.0 < -0.5 {
            spaceship.rotate_left();
        } else if left_stick.0 > 0.5 {
            spaceship.rotate_right();
        }

        spaceship.set_push(left_stick.1 > 0.5);

        if gamepad.is_just_pressed(gamepads::Button::FrontRightLower) {
            let missile = Missile::new(spaceship.get_position(), spaceship.get_direction());
            missiles.push(missile);
        }
    }

    false
}

/// Met à jour les positions et les états des objets dans le jeu.
///
/// # Paramètres
/// - `asteroids`: Liste des astéroïdes.
/// - `spaceship`: Référence mutable au vaisseau spatial.
/// - `missiles`: Liste des missiles actifs.
fn update_model(
    asteroids: &mut [Asteroid],
    spaceship: &mut Spaceship,
    missiles: &mut Vec<Missile>,
) {
    for asteroid in asteroids {
        asteroid.move_object();
    }
    spaceship.update();
    missiles.retain(|missile| missile.is_active()); // Recevoir uniquement les missiles en état d'activation.
    for missile in missiles {
        missile.update();
    }
}

/// Gère les collisions entre objets stellaires (vaisseau, missiles, astéroïdes).
///
/// # Paramètres
/// - `asteroids`: Liste des astéroïdes.
/// - `spaceship`: Référence au vaisseau spatial.
/// - `missiles`: Liste des missiles actifs.
///
/// # Retour
/// `true` si le jeu doit se terminer en raison d'une collision ou de la victoire, sinon `false`.
fn handle_collisions(
    asteroids: &mut Vec<Asteroid>,
    spaceship: &Spaceship,
    missiles: &mut [Missile],
) -> bool {
    let mut new_asteroids = vec![];
    let mut to_remove = vec![];
    handle_asteroid_collisions(asteroids, &mut new_asteroids, &mut to_remove);

    let spaceship_collision = handle_spaceship_asteroid_collision(
        asteroids,
        spaceship,
        &mut new_asteroids,
        &mut to_remove,
    );

    handle_missile_asteroid_collisions(missiles, asteroids, &mut new_asteroids, &mut to_remove);

    remove_collided_asteroids(asteroids, &to_remove);

    asteroids.extend(new_asteroids);

    if spaceship_collision {
        return true;
    }

    false
}

/// Gère la condition de victoire en vérifiant si la liste des astéroïdes est vide.
///
/// # Paramètres
/// * `asteroids` - Une slice à un vecteur d'astéroïdes, représentant les objets restants dans le jeu.
///
/// # Retour
/// Retourne `true` si tous les astéroïdes ont été détruits et le joueur a gagné, sinon retourne `false`.
fn handle_win(asteroids: &[Asteroid]) -> bool {
    if asteroids.is_empty() {
        draw_you_win();
        return true;
    }

    false
}

/// Détecte et gère les collisions entre astéroïdes.
///
/// Si deux astéroïdes entrent en collision, ils se divisent si leur taille est égale.
/// Si leurs tailles sont différentes, le plus petit se divise.
///
/// # Paramètres
/// - `asteroids`: Liste des astéroïdes.
/// - `new_asteroids`: Liste des nouveaux fragments d'astéroïdes.
/// - `to_remove`: Liste des indices des astéroïdes à supprimer.
fn handle_asteroid_collisions(
    asteroids: &mut [Asteroid],
    new_asteroids: &mut Vec<Asteroid>,
    to_remove: &mut Vec<usize>,
) {
    for i in 0..asteroids.len() {
        for j in (i + 1)..asteroids.len() {
            let asteroid_a = &asteroids[i];
            let asteroid_b = &asteroids[j];

            if asteroid_a.collide(asteroid_b) {
                if (asteroid_a.get_size() - asteroid_b.get_size()).abs() < f32::EPSILON {
                    to_remove.push(i);
                    to_remove.push(j);
                    new_asteroids.extend(asteroid_a.split());
                    new_asteroids.extend(asteroid_b.split());
                } else {
                    let (small_idx, small_asteroid) =
                        if asteroid_a.get_size() < asteroid_b.get_size() {
                            (i, asteroid_a)
                        } else {
                            (j, asteroid_b)
                        };
                    to_remove.push(small_idx);
                    new_asteroids.extend(small_asteroid.split());
                }
                break;
            }
        }
    }
}

/// Détecte si le vaisseau spatial est entré en collision avec un astéroïde.
///
/// # Paramètres
/// - `asteroids`: Liste des astéroïdes.
/// - `spaceship`: Référence au vaisseau spatial.
/// - `new_asteroids`: Liste des nouveaux fragments d'astéroïdes.
/// - `to_remove`: Liste des indices des astéroïdes à supprimer.
///
/// # Retour
/// `true` si une collision est détectée, sinon `false`.
fn handle_spaceship_asteroid_collision(
    asteroids: &mut [Asteroid],
    spaceship: &Spaceship,
    new_asteroids: &mut Vec<Asteroid>,
    to_remove: &mut Vec<usize>,
) -> bool {
    for (asteroid_index, asteroid) in asteroids.iter_mut().enumerate() {
        if asteroid.collide(spaceship) {
            match asteroid.get_size() {
                Asteroid::LARGE => {
                    new_asteroids.extend(asteroid.split());
                    to_remove.push(asteroid_index);
                }
                Asteroid::MEDIUM => {
                    new_asteroids.extend(asteroid.split());
                    to_remove.push(asteroid_index);
                }
                Asteroid::SMALL => {
                    to_remove.push(asteroid_index);
                }
                _ => {}
            }
            return true;
        }
    }
    false
}

/// Détecte et gère les collisions entre missiles et astéroïdes.
///
/// # Paramètres
/// - `missiles`: Liste des missiles.
/// - `asteroids`: Liste des astéroïdes.
/// - `new_asteroids`: Liste des nouveaux fragments d'astéroïdes.
/// - `to_remove`: Liste des indices des astéroïdes à supprimer.
fn handle_missile_asteroid_collisions(
    missiles: &mut [Missile],
    asteroids: &mut [Asteroid],
    new_asteroids: &mut Vec<Asteroid>,
    to_remove: &mut Vec<usize>,
) {
    for missile in missiles.iter_mut() {
        if !missile.is_active() {
            continue;
        }

        for (asteroid_index, asteroid) in asteroids.iter_mut().enumerate() {
            if missile.collide(asteroid) {
                missile.deactivate();

                match asteroid.get_size() {
                    Asteroid::LARGE => {
                        new_asteroids.extend(asteroid.split());
                        to_remove.push(asteroid_index);
                    }
                    Asteroid::MEDIUM => {
                        new_asteroids.extend(asteroid.split());
                        to_remove.push(asteroid_index);
                    }
                    Asteroid::SMALL => {
                        to_remove.push(asteroid_index);
                    }
                    _ => {}
                }
                break;
            }
        }
    }
}

/// Supprime les astéroïdes qui ont été détruits suite à des collisions.
///
/// # Paramètres
/// - `asteroids`: Liste des astéroïdes à modifier.
/// - `to_remove`: Liste des indices des astéroïdes à supprimer.
fn remove_collided_asteroids(asteroids: &mut Vec<Asteroid>, to_remove: &[usize]) {
    let mut to_remove_sorted = to_remove.to_vec();
    to_remove_sorted.sort_unstable();
    to_remove_sorted.dedup();

    for &index in to_remove_sorted.iter().rev() {
        if index < asteroids.len() {
            asteroids.remove(index);
        }
    }
}

/// Fonction principale qui initialise et exécute le jeu.
///
/// Cette fonction initialise le jeu, gère l'affichage des menus, les choix de difficulté,
/// et lance la boucle principale du jeu.
#[macroquad::main("Asteroids game")]
async fn main() {
    let mut gamepads = Gamepads::new();
    let background_texture = load_texture("../img/asteroide.png").await.unwrap();
    background_texture.set_filter(FilterMode::Nearest);
    let texture_spaceship = load_texture("../img/spaceship.png").await.unwrap();
    background_texture.set_filter(FilterMode::Nearest);

    let mut difficulty = 0;

    let mut selected_difficulty = false;

    // Sélection de la difficulté dans le menu
    while !selected_difficulty {
        clear_background(BLACK);

        // Paramètres pour la taille des boutons et du texte dans le menu
        let screen_width = screen_width();
        let screen_height = screen_height();

        let button_width = screen_width * 0.4;
        let button_height = screen_height * 0.1;
        let font_size = screen_height * 0.05;
        let title_font_size = screen_height * 0.08;

        let button_x = (screen_width - button_width) / 2.0;
        let easy_y = screen_height * 0.3;
        let medium_y = easy_y + button_height + screen_height * 0.05;
        let hard_y = medium_y + button_height + screen_height * 0.05;

        draw_text(
            "Select Difficulty",
            screen_width * 0.3,
            screen_height * 0.2,
            title_font_size,
            WHITE,
        );

        // Sélection de la difficulté en fonction de la position du clic de la souris
        if is_mouse_button_pressed(MouseButton::Left) {
            let (mx, my) = mouse_position();

            if mx >= button_x
                && mx <= button_x + button_width
                && my >= easy_y
                && my <= easy_y + button_height
            {
                difficulty = 5;
                selected_difficulty = true;
            } else if mx >= button_x
                && mx <= button_x + button_width
                && my >= medium_y
                && my <= medium_y + button_height
            {
                difficulty = 30;
                selected_difficulty = true;
            } else if mx >= button_x
                && mx <= button_x + button_width
                && my >= hard_y
                && my <= hard_y + button_height
            {
                difficulty = 100;
                selected_difficulty = true;
            }
        }

        // Affichage des boutons de sélection de difficulté
        draw_rectangle(button_x, easy_y, button_width, button_height, DARKGRAY);
        draw_text(
            "Easy",
            button_x + button_width * 0.4,
            easy_y + button_height * 0.6,
            font_size,
            WHITE,
        );

        draw_rectangle(button_x, medium_y, button_width, button_height, DARKGRAY);
        draw_text(
            "Medium",
            button_x + button_width * 0.35,
            medium_y + button_height * 0.6,
            font_size,
            WHITE,
        );

        draw_rectangle(button_x, hard_y, button_width, button_height, DARKGRAY);
        draw_text(
            "Hard",
            button_x + button_width * 0.4,
            hard_y + button_height * 0.6,
            font_size,
            WHITE,
        );

        next_frame().await;
    }

    // Initialisation du jeu avec la difficulté sélectionnée
    let mut asteroids = Vec::new();
    let mut spaceship = Spaceship::new(texture_spaceship);
    let mut missiles = Vec::new();
    let mut health = 3.0;

    // Création des astéroïdes en fonction de la difficulté
    for _ in 0..difficulty {
        asteroids.push(asteroid::Asteroid::new());
    }

    // Boucle principale du jeu
    loop {
        // Dessin du jeu
        draw(&asteroids, &spaceship, &missiles, &background_texture);
        draw_health_bar(health);

        // Vérification des entrées du joueur
        if handle_input(&mut spaceship, &mut missiles, &mut gamepads) {
            break;
        }

        // Mise à jour des objets dans le jeu
        update_model(&mut asteroids, &mut spaceship, &mut missiles);

        // Gestion des collisions
        if handle_collisions(&mut asteroids, &spaceship, &mut missiles) {
            health -= 1.0;
        }

        // Gestion de fin du jeu
        if health < 0.0 {
            draw_game_over();
            next_frame().await;
            thread::sleep(Duration::from_secs(3));
            break;
        }

        // Gestion de la condition de victoire
        if handle_win(&asteroids) {
            next_frame().await;
            thread::sleep(Duration::from_secs(3));
            break;
        }

        next_frame().await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_asteroid_collision_handling() {
        let mut asteroids = vec![
            Asteroid::with_size(Asteroid::MEDIUM, Vec2::new(100.0, 100.0)),
            Asteroid::with_size(Asteroid::MEDIUM, Vec2::new(105.0, 100.0)),
        ];
        let mut new_asteroids = Vec::new();
        let mut to_remove = Vec::new();

        handle_asteroid_collisions(&mut asteroids, &mut new_asteroids, &mut to_remove);

        assert_eq!(to_remove.len(), 2);
        assert!(new_asteroids.len() > 0);
    }

    #[test]
    fn test_missile_asteroid_collision() {
        let mut asteroids = vec![Asteroid::with_size(
            Asteroid::MEDIUM,
            Vec2::new(100.0, 100.0),
        )];

        let missile_position = Vec2::new(100.0, 100.0);
        let missile_angle = 0.0;
        let mut missiles = vec![Missile::new(missile_position, missile_angle)];

        let mut new_asteroids = Vec::new();
        let mut to_remove = Vec::new();

        handle_missile_asteroid_collisions(
            &mut missiles,
            &mut asteroids,
            &mut new_asteroids,
            &mut to_remove,
        );

        assert_eq!(to_remove.len(), 1);
        assert!(new_asteroids.len() > 0 || asteroids.is_empty());
    }

    #[test]
    fn test_remove_collided_asteroids() {
        let mut asteroids = vec![
            Asteroid::with_size(Asteroid::MEDIUM, Vec2::new(100.0, 100.0)),
            Asteroid::with_size(Asteroid::MEDIUM, Vec2::new(200.0, 200.0)),
        ];
        let to_remove = vec![0];

        remove_collided_asteroids(&mut asteroids, &to_remove);

        assert_eq!(asteroids.len(), 1);
        assert_eq!(asteroids[0].get_position(), Vec2::new(200.0, 200.0));
    }
}
