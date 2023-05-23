use raylib::prelude::*;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .title("rustsnake")
        .build();
    rl.set_target_fps(60);

    while !rl.window_should_close() {
        // Start Drawing
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);
        d.draw_text("Score: ", 12, 12, 20, Color::BLACK);
        // d.draw_fps(12,12);
        d.draw_rectangle(12, 30, d.get_screen_width()-24, d.get_screen_height()-50, Color::RED);
        d.draw_rectangle(16, 34, d.get_screen_width()-32, d.get_screen_height()-58, Color::WHITE);
    }
}
