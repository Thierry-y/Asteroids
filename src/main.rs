use asteroid::Asteroid;
use macroquad::prelude::*;
use missile::Missile;
use spaceship::Spaceship;
use std::thread;
use std::time::Duration;
use stellarobject::StellarObject;
//use std::io;

mod asteroid;
mod missile;
mod spaceship;
mod stellarobject;

fn draw(
    asteroids: &[Asteroid],
    spaceship: &Spaceship,
    missiles: &[Missile],
    texture: &Texture2D,
) {
    draw_background(texture);
    draw_asteroids(asteroids);
    spaceship.draw();
    for missile in missiles {
        missile.draw();
    }
}

fn draw_background(background_texture: &Texture2D) {
    draw_texture(background_texture, 0.0, 0.0, WHITE);
}

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

fn draw_asteroids(asteroids: &[Asteroid]) {
    for asteroid in asteroids {
        draw_circle_lines(
            asteroid.get_position().x,
            asteroid.get_position().y,
            asteroid.get_size() / 2.0,
            1.0,
            YELLOW,
        );
    }
}

fn handle_input(spaceship: &mut Spaceship, missiles: &mut Vec<Missile>) -> bool {
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

    false
}

fn update_model(
    asteroids: &mut [Asteroid],
    spaceship: &mut Spaceship,
    missiles: &mut Vec<Missile>,
) {
    for asteroid in asteroids {
        asteroid.move_object();
    }
    spaceship.update();
    missiles.retain(|missile| missile.is_active());
    for missile in missiles {
        missile.update();
    }
}

fn handle_collisions(
    asteroids: &mut Vec<Asteroid>,
    spaceship: &Spaceship,
    missiles: &mut Vec<Missile>,
) -> bool {
    let mut new_asteroids = vec![];
    let mut to_remove = vec![];
    handle_asteroid_collisions(asteroids, &mut new_asteroids, &mut to_remove);

    if handle_spaceship_asteroid_collision(asteroids, spaceship) {
        draw_game_over();
        return true;
    }

    handle_missile_asteroid_collisions(missiles, asteroids, &mut new_asteroids, &mut to_remove);

    remove_collided_asteroids(asteroids, &to_remove);

    asteroids.extend(new_asteroids);

    if asteroids.is_empty() {
        draw_you_win();
        return true;
    }

    false
}

//si deux astéroïdes entrent en collision,ceux de même taille se divisent,
//tandis que si leur taille est différente,c'est le plus petit qui se divise.
fn handle_asteroid_collisions(
    asteroids: &mut Vec<Asteroid>,
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

fn handle_spaceship_asteroid_collision(asteroids: &[Asteroid], spaceship: &Spaceship) -> bool {
    for asteroid in asteroids.iter() {
        if spaceship.collide(asteroid) {
            return true;
        }
    }
    false
}

fn handle_missile_asteroid_collisions(
    missiles: &mut Vec<Missile>,
    asteroids: &mut Vec<Asteroid>,
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

#[macroquad::main("Asteroids game")]
async fn main() {
    let background_texture = load_texture("../img/asteroide.png").await.unwrap();
    background_texture.set_filter(FilterMode::Nearest);
    let texture_spaceship = load_texture("../img/spaceship.png").await.unwrap();
    background_texture.set_filter(FilterMode::Nearest);

    let mut difficulty = 0;

    let mut selected_difficulty = false;

    while !selected_difficulty {
        clear_background(BLACK);

        //Les paramètres sont créés dans la boucle while afin de pouvoir
        //modifier la taille de la fenêtre de jeu en fonction des besoins
        //de l'utilisateur, sans affecter la jouabilité.
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

    let mut asteroids = Vec::new();
    let mut spaceship = Spaceship::new(texture_spaceship);
    let mut missiles = Vec::new();

    for _ in 0..difficulty {
        asteroids.push(asteroid::Asteroid::new());
    }

    loop {
        //clear_background(BLACK);

        draw(&asteroids, &spaceship, &missiles, &background_texture);

        if handle_input(&mut spaceship, &mut missiles) {
            break;
        }

        update_model(&mut asteroids, &mut spaceship, &mut missiles);

        if handle_collisions(&mut asteroids, &spaceship, &mut missiles) {
            next_frame().await;
            thread::sleep(Duration::from_secs(3));
            break;
        }

        next_frame().await
    }
}
