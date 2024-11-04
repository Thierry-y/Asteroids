use asteroid::Asteroid;
use macroquad::prelude::*;

mod asteroid;

fn draw(asteroids: &[Asteroid]) {
    draw_background();
    draw_asteroids(asteroids);
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

fn handle_input() -> bool {
    if is_key_down(KeyCode::Escape) {
        return true
    }

    false
}

fn update_model(asteroids: &mut [Asteroid]) {
    for asteroid in asteroids {
        asteroid.move_object();
    }
}

//La logique de collision a été modifiée : si deux astéroïdes entrent en collision, 
//ceux de même taille se divisent, tandis que si leur taille est différente, 
//c'est le plus petit qui se divise.
fn handle_collisions(asteroids: &mut Vec<Asteroid>) {
    let mut new_asteroids = vec![];
    let mut to_remove = vec![]; 

    for i in 0..asteroids.len() {
        for j in (i + 1)..asteroids.len() {
            let asteroid_a = &asteroids[i];
            let asteroid_b = &asteroids[j];

            let distance = asteroid_a.get_position().distance(asteroid_b.get_position());

            if distance < (asteroid_a.get_size() + asteroid_b.get_size()) / 2.0 {
                if (asteroid_a.get_size() - asteroid_b.get_size()).abs() < f32::EPSILON {  // =0 possible pose problem float
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

    to_remove.sort_unstable();
    to_remove.dedup();
    for &index in to_remove.iter().rev() {
        asteroids.remove(index);
    }

    asteroids.extend(new_asteroids);
}



#[macroquad::main("BasicShapes")]
async fn main() {
    let mut asteroids = Vec::new();

    for _ in 0..10 {
        asteroids.push(asteroid::Asteroid::new());
    }

    loop {
        draw(&asteroids);

        if handle_input() { break; }

        update_model(&mut asteroids);

        handle_collisions(&mut asteroids);

        next_frame().await
    }
}
