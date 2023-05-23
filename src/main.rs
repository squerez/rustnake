use raylib::prelude::*;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .title("rustsnake")
        .build();


    while !rl.window_should_close() {
        {
            // Start Drawing
            let mut d = rl.begin_drawing(&thread);
            d.clear_background(Color::WHITE);
            d.draw_text("Hello, world!", 12, 12, 20, Color::BLACK);

            if (d.is_mouse_button_down(MouseButton::MOUSE_LEFT_BUTTON))
            {
                d.draw_circle(d.get_mouse_x(), d.get_mouse_y(), 10.0, Color::BLUE)
            }
            if (d.is_mouse_button_down(MouseButton::MOUSE_RIGHT_BUTTON))
            {
                d.draw_circle(d.get_mouse_x(), d.get_mouse_y(), 10.0, Color::WHITE)
            }
        }
    }
}
