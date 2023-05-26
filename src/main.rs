use raylib::prelude::*;
use rand::Rng;
use std::time;

// fn movement(raylib::drawing::RaylibDrawHandle a) {

// }

struct BBOX{
    xmin: i32,
    ymin: i32,
    xmax: i32,
    ymax: i32,
}

fn draw_box(x:i32, y:i32, line_width:i32, box_width:i32, box_height:i32, draw_handle:&mut RaylibDrawHandle){
    draw_handle.draw_rectangle(x, y, box_width, box_height, Color::RED);
    draw_handle.draw_rectangle(x+line_width, y+line_width, box_width-(line_width*2), 
                               box_height-(line_width*2), Color::WHITE);
}

fn check_collision_box(bounding_box: &BBOX, mut x: i32, mut y: i32)->(i32, i32){

    if x<bounding_box.xmin {x = bounding_box.xmax};
    if x>bounding_box.xmax {x = bounding_box.xmin};
    if y<bounding_box.ymin {y = bounding_box.ymax};
    if y>bounding_box.ymax {y = bounding_box.ymin};
    return (x,y);
}

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
    let bounding_box: BBOX = BBOX{xmin: (box_x+box_line_width), 
                                  ymin: (box_y+box_line_width),
                                  xmax: rl.get_screen_width()-12-(box_line_width*2), 
                                  ymax: rl.get_screen_height()-12-(box_line_width*2)};
                                  
    let pixel_size: i32 = 10;

    // Random food number
    let mut rng = rand::thread_rng();
    
    // Snake position
    let mut x: i32 = 16;
    let mut y: i32 = 34;
    let mut food_x: i32 = 0;
    let mut food_y: i32 = 0;
    let mut food: bool = false;
    
    // Snake speed
    let snake_speed: i64 = 100;
    let _time: time::Duration = time::Duration::new(0, 100_000_000);
    let mut _tic = time::Instant::now();
    let mut _toc = time::Instant::now();

    while !rl.window_should_close() {
        // Start Drawing
        let mut d: RaylibDrawHandle = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);

        // Draw score 
        d.draw_text("Score: ", 12, 12, 20, Color::BLACK);

        // d.draw_fps(12,12);
        draw_box(box_x, box_y, box_line_width, box_width, box_height, &mut d);

        // Update logic according to the snake speed
        _toc = time::Instant::now();
        if (_toc-_tic) > _time{
                // Draw food
            if !food
            {   
                food_x = rng.gen_range(bounding_box.xmin..bounding_box.xmax);
                food_y = rng.gen_range(bounding_box.ymin..bounding_box.ymax);
                food = true;
            }

            if d.is_key_down(KeyboardKey::KEY_S) {
                y = y + pixel_size;
            };
            if d.is_key_down(KeyboardKey::KEY_W) {
                y = y - pixel_size;
            };
            if d.is_key_down(KeyboardKey::KEY_A) {
                x = x - pixel_size;
            };
            if d.is_key_down(KeyboardKey::KEY_D) {
                x = x + pixel_size;
            };

            if x == food_x && y==food_y {food = false};

            (x, y) = check_collision_box(&bounding_box, x, y);
            // Player position
            d.draw_rectangle(x, y, pixel_size, pixel_size, Color::BLUE);
            // Food position
            d.draw_rectangle(food_x, food_y, pixel_size, pixel_size, Color::GREEN);
        }
    }
        
        
}
