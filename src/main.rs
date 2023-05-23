use raylib::prelude::*;
use rand::Rng;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .title("rustsnake")
        .build();

    // Add FPS if desired
    rl.set_target_fps(60);

    // Random food number
    let mut rng = rand::thread_rng();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);
        
        let food_y: i32 = rng.gen_range(0..640);
        let food_x: i32 = rng.gen_range(0..480);
        d.draw_circle(food_y, food_x, 10.0, Color::BLUE);

        // // Draw a blue line
        // if d.is_mouse_button_down(MouseButton::MOUSE_LEFT_BUTTON)
        // {
        //     d.draw_circle(d.get_mouse_x(), d.get_mouse_y(), 10.0, Color::BLUE)
        // }

        // // Draw
        // if d.is_mouse_button_down(MouseButton::MOUSE_RIGHT_BUTTON)
        // {
        //     d.draw_circle(d.get_mouse_x(), d.get_mouse_y(), 10.0, Color::WHITE)
        // }
    }
}
