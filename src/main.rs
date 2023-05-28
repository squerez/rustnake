use raylib::prelude::*;
use std::time::Duration;
use std::thread;

mod engine;
use engine::game::Game;
use engine::configs::*;


fn main() {
    // Initialize raylib and its components
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("rustnake")
        .build();
    rl.set_target_fps(60);

    // Create a new game
    let mut game = Game::new();

    // Start drawing the game
    while !rl.window_should_close() {
        let mut d: RaylibDrawHandle = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);

        if !game.game_over {
            if d.is_key_pressed(KeyboardKey::KEY_UP) && game.direction != Direction::Down {
                game.direction = Direction::Up;
            }
            if d.is_key_pressed(KeyboardKey::KEY_DOWN) && game.direction != Direction::Up {
                game.direction = Direction::Down;
            }
            if d.is_key_pressed(KeyboardKey::KEY_LEFT) && game.direction != Direction::Right {
                game.direction = Direction::Left;
            }
            if d.is_key_pressed(KeyboardKey::KEY_RIGHT) && game.direction != Direction::Left {
                game.direction = Direction::Right;
            }
            game.update();
            game.render(&mut d);
        }
        else {
            if d.is_key_pressed(KeyboardKey::KEY_ENTER) {
                game = Game::new();
            } 
            else {
                let score_message = format!("Score: {}", game.score);
                d.draw_text(
                    "Game Over",
                    SCREEN_WIDTH / 2 - 70,
                    SCREEN_HEIGHT / 2 - 10,
                    40,
                    Color::BLACK,
                );
                d.draw_text(
                    &score_message,
                    SCREEN_WIDTH / 2 - 70,
                    SCREEN_HEIGHT / 2 + 40,
                    20,
                    Color::BLACK,
                );
            }

        }
        thread::sleep(Duration::from_millis(75));
    }
}

