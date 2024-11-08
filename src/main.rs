use asteroid::Asteroid;
use spaceship::Spaceship;
use macroquad::prelude::*;

mod asteroid;
mod spaceship; 


fn draw(asteroids: &[Asteroid], spaceship: &Spaceship) {
    draw_background();
    draw_asteroids(asteroids);
    spaceship.draw(); 
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

fn handle_input(spaceship: &mut Spaceship) -> bool {
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

    false
}

fn update_model(asteroids: &mut [Asteroid], spaceship: &mut Spaceship) {
    for asteroid in asteroids {
        asteroid.move_object();
    }
    spaceship.update();
}

//La logique de collision a été modifiée : si deux astéroïdes entrent en collision, 
//ceux de même taille se divisent, tandis que si leur taille est différente, 
//c'est le plus petit qui se divise.
fn handle_collisions(asteroids: &mut Vec<Asteroid>, spaceship: &Spaceship) -> bool {
    let mut new_asteroids = vec![];
    let mut to_remove = vec![];

    for i in 0..asteroids.len() {
        for j in (i + 1)..asteroids.len() {
            let asteroid_a = &asteroids[i];
            let asteroid_b = &asteroids[j];

            let distance = asteroid_a.get_position().distance(asteroid_b.get_position());
            if distance < (asteroid_a.get_size() + asteroid_b.get_size()) / 2.0 {
                if (asteroid_a.get_size() - asteroid_b.get_size()).abs() < f32::EPSILON {
                    to_remove.push(i);
                    to_remove.push(j);
                    new_asteroids.extend(asteroid_a.split());
                    new_asteroids.extend(asteroid_b.split());
                } else {
                    let (small_idx, small_asteroid) = if asteroid_a.get_size() < asteroid_b.get_size() {
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

    for (_i, asteroid) in asteroids.iter().enumerate() {
        let distance = asteroid.get_position().distance(spaceship.get_position());
        if distance < asteroid.get_size() / 2.0 + Spaceship::SIZE / 2.0 {
            println!("Game Over!"); 
            return false;      //Déboguer le programme en désactivant temporairement la fin du jeu en cas de collision.
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
    false
}



#[macroquad::main("BasicShapes")]
async fn main() {
    let mut asteroids = Vec::new();
    let mut spaceship = Spaceship::new();

    for _ in 0..100 {
        asteroids.push(asteroid::Asteroid::new());
    }

    loop {
        draw(&asteroids, &spaceship);

        if handle_input(&mut spaceship) { break; }

        update_model(&mut asteroids, &mut spaceship);

        if handle_collisions(&mut asteroids, &spaceship) {
            break;
        }

        next_frame().await
    }
}
