use raylib::prelude::*;
// use rand::Rng;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .title("rustsnake")
        .build();

    // Add FPS if desired
    rl.set_target_fps(60);

    // Random food number
    // let mut rng = rand::thread_rng();
    
    // Snake position
    let mut x: i32 = 16;
    let mut y: i32 = 34;

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        // Clear background 
        d.clear_background(Color::WHITE);

        // Draw score 
        d.draw_text("Score: ", 12, 12, 20, Color::BLACK);

        // Add screen limits
        let width = d.get_screen_width();
        let height = d.get_screen_height();

        // Draw limit frame
        d.draw_rectangle(12, 30, width-24, height-50, Color::RED);
        d.draw_rectangle(16, 34, width-32, height-58, Color::WHITE);

        // // Draw food
        // let food_y: i32 = rng.gen_range(21..width-37);
        // let food_x: i32 = rng.gen_range(39..height-63);
        // d.draw_circle(food_y, food_x, 10.0, Color::BLUE);

        if d.is_key_pressed(KeyboardKey::KEY_S) {
            y = y + 10;
        };
        if d.is_key_pressed(KeyboardKey::KEY_W) {
            y = y - 10;
        };
        if d.is_key_pressed(KeyboardKey::KEY_A) {
            x = x - 10;
        };
        if d.is_key_pressed(KeyboardKey::KEY_D) {
            x = x + 10;
        };
        d.draw_rectangle(x, y, 10, 10, Color::BLUE);
    }
}
