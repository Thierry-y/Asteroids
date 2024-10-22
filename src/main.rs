use asteroid::Asteroid;
use macroquad::prelude::*;

mod asteroid;

fn draw(asteroid : &Asteroid) {
    draw_background();
    draw_asteroids(asteroid);
}

fn draw_background() {
    clear_background(BLACK);
}

fn draw_asteroids(asteroid : &Asteroid) {
    draw_circle_lines(asteroid.get_position().x, asteroid.get_position().y, 60.0, 1.0, YELLOW);
}

fn handle_input() -> bool {
    if is_key_down(KeyCode::Escape) {
        return true
    }

    false
}

fn update_model(asteroid : &mut Asteroid) {
    asteroid.move_object();
}

#[macroquad::main("BasicShapes")]
async fn main() {
    let mut asteroid = asteroid::Asteroid::new();

    loop {
        draw(&asteroid);

        if handle_input() { break; }

        update_model(&mut asteroid);

        next_frame().await
    }
}
