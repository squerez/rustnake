use raylib::prelude::*;
// use rand::Rng;

// fn movement(raylib::drawing::RaylibDrawHandle a) {

// }
fn draw_box(x:i32, y:i32, line_width:i32, box_width:i32, box_height:i32, draw_handle:&mut RaylibDrawHandle){
    draw_handle.draw_rectangle(x, y, box_width, box_height, Color::RED);
    draw_handle.draw_rectangle(x+line_width, y+line_width, box_width-(line_width*2), 
                               box_height-(line_width*2), Color::WHITE);
}

// fn collision_box(bbox:[4]i32){

// }

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .title("rustsnake")
        .build();

    // Add FPS if desired
    rl.set_target_fps(60);
    let box_x:i32 = 12;
    let box_y:i32 = 30;
    let box_line_width:i32 = 4;
    let box_width:i32 = rl.get_screen_width()-box_x-12;
    let box_height:i32 = rl.get_screen_height()-box_y-12;
    let boundary_box: [i32; 4] = [box_x+box_line_width, box_y+box_line_width, // xmin, ymin
                                  rl.get_screen_width()-12-(box_line_width*2), // xmax
                                  rl.get_screen_height()-12-(box_line_width*2)]; // ymax
                                  
    let _pixel_size_width: i32 = 10;
    let _pixel_size_height: i32 = 10;

    // Random food number
    // let mut rng = rand::thread_rng();
    
    // Snake position
    let mut x: i32 = 16;
    let mut y: i32 = 34;

    while !rl.window_should_close() {
        // Start Drawing
        let mut d: RaylibDrawHandle = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);

        // Draw score 
        d.draw_text("Score: ", 12, 12, 20, Color::BLACK);

        // d.draw_fps(12,12);
        draw_box(box_x, box_y, box_line_width, box_width, box_height, &mut d);

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
