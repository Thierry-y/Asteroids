use asteroid::Asteroid;
use macroquad::prelude::*;
use missile::Missile;
use spaceship::Spaceship;
use std::io;

mod asteroid;
mod missile;
mod spaceship;

fn draw(asteroids: &[Asteroid], spaceship: &Spaceship, missiles: &[Missile]) {
    draw_background();
    draw_asteroids(asteroids);
    spaceship.draw();
    for missile in missiles {
        missile.draw();
    }
}

fn draw_background() {
    clear_background(BLACK);
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

fn update_model(asteroids: &mut [Asteroid], spaceship: &mut Spaceship, missiles: &mut Vec<Missile>) {
    for asteroid in asteroids {
        asteroid.move_object();
    }
    spaceship.update();
    missiles.retain(|missile| missile.is_active());
    for missile in missiles {
        missile.update();
    }
}

//La logique de collision a été modifiée : si deux astéroïdes entrent en collision,
//ceux de même taille se divisent, tandis que si leur taille est différente,
//c'est le plus petit qui se divise.
fn handle_collisions(asteroids: &mut Vec<Asteroid>, spaceship: &Spaceship, missiles: &mut Vec<Missile>) -> bool {
    let mut new_asteroids = vec![];
    let mut to_remove = vec![];

    //collision asteroides
    for i in 0..asteroids.len() {
        for j in (i + 1)..asteroids.len() {
            let asteroid_a = &asteroids[i];
            let asteroid_b = &asteroids[j];

            let distance = asteroid_a
                .get_position()
                .distance(asteroid_b.get_position());
            if distance < (asteroid_a.get_size() + asteroid_b.get_size()) / 2.0 {
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


    //collision spacenship asteroides
    for asteroid in asteroids.iter() {
        let distance = asteroid.get_position().distance(spaceship.get_position());
        if distance < asteroid.get_size() / 2.0 + Spaceship::SIZE / 2.0 {
            println!("Game Over!");
            return true;
        }
    }


    //collision missiles asteroides
    for missile in missiles.iter_mut() {
        if !missile.is_active() {
            continue;
        }

        for (asteroid_index, asteroid) in asteroids.iter_mut().enumerate() {
            let distance = missile.get_position().distance(asteroid.get_position());
            if distance < asteroid.get_size() / 2.0 + Missile::SIZE / 2.0 {
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

    to_remove.sort_unstable();
    to_remove.dedup();
    for &index in to_remove.iter().rev() {
        if index < asteroids.len() {
            asteroids.remove(index);
        }
    }

    asteroids.extend(new_asteroids);

    //game win
    if asteroids.is_empty() {
        println!("You win!");
        return true;
    }

    false
}

#[macroquad::main("BasicShapes")]
async fn main() {
    let mut asteroids = Vec::new();
    let mut spaceship = Spaceship::new();
    let mut missiles = Vec::new();

    let mut difficulty = 0;

    println!("Please select a difficulty level:");
    println!("1. Easy");
    println!("2. Medium");
    println!("3. Hard");

    println!("Enter your choice (1, 2, or 3): ");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let choice = input.trim();

    match choice {
        "1" => {
            println!("You selected Easy difficulty.");
            difficulty = 5;
        }
        "2" => {
            println!("You selected Medium difficulty.");
            difficulty = 30;
        }
        "3" => {
            println!("You selected Hard difficulty.");
            difficulty = 100;
        }
        _ => println!("Invalid choice, please enter 1, 2, or 3."),
    }

    for _ in 0..difficulty {
        asteroids.push(asteroid::Asteroid::new());
    }

    loop {
        draw(&asteroids, &spaceship, &missiles);

        if handle_input(&mut spaceship, &mut missiles) {
            break;
        }

        update_model(&mut asteroids, &mut spaceship, &mut missiles);

        if handle_collisions(&mut asteroids, &spaceship, &mut missiles) {
            break;
        }

        next_frame().await
    }
}
