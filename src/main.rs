use raylib::prelude::*;
use rand::Rng;


fn main() {
    let screen_width = 1920;
    let screen_height = 1080;

    let (mut rl, thread) = raylib::init()
        .size(screen_width, screen_height)
        .title("Hello, World")
        .build();

     
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        let mut iteration = 1
        
         
        d.clear_background(Color::WHITE);

        while iteration < 101 {

        }

        d.draw_circle(0, 0, 50.0, Color::DARKBLUE);
    }
}
