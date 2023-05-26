use raylib::prelude::*;
use rand::Rng;
use std::time;

// fn movement(raylib::drawing::RaylibDrawHandle a) {

// }
enum DIRECTION{
    Up, Down, Left, Right
}

#[derive(Debug)]
struct BBOX{
    xmin: i32,
    ymin: i32,
    xmax: i32,
    ymax: i32,
}

#[derive(Debug)]
struct OBJECT{
    x: i32,
    y: i32,
    real_x: i32,
    real_y: i32,
}

struct VECTOR2D{
    x: i32,
    y: i32,
}

fn check_collision_box(bounding_box: &BBOX, object: &mut OBJECT){
    if object.x<bounding_box.xmin {object.x = bounding_box.xmax};
    if object.x>bounding_box.xmax {object.x = bounding_box.xmin};
    if object.y<bounding_box.ymin {object.y = bounding_box.ymax};
    if object.y>bounding_box.ymax {object.y = bounding_box.ymin};
}

fn update_real_pos(object: &mut OBJECT, padding: &VECTOR2D, size: &VECTOR2D){
    object.real_x = object.x * size.x + padding.x;
    object.real_y = object.y * size.y + padding.y;
}

fn get_input_dir(mut direction:DIRECTION, handle: &RaylibDrawHandle) -> DIRECTION{
    if handle.is_key_down(KeyboardKey::KEY_W) {
        if !matches!(direction, DIRECTION::Down) {direction = DIRECTION::Up};
    };
    if handle.is_key_down(KeyboardKey::KEY_A) {
        if !matches!(direction, DIRECTION::Right) {direction = DIRECTION::Left};
    };
    if handle.is_key_down(KeyboardKey::KEY_S) {
        if !matches!(direction, DIRECTION::Up) {direction = DIRECTION::Down};
    };
    if handle.is_key_down(KeyboardKey::KEY_D) {
        if !matches!(direction, DIRECTION::Left) {direction = DIRECTION::Right};
    };
    return direction;
}

fn main() {
    // Hardcoded window number because of the pixel size
    // TODO make it dynamic
    let win_height: i32 = 515;
    let win_width: i32 = 480;
    let (mut rl, thread) = raylib::init()
        .size(win_width, win_height)
        .title("rustsnake")
        .build();

    rl.set_target_fps(60);

    // Converting the whole area as if it was pixels
    let pixel_pad: VECTOR2D = VECTOR2D {x: 0, y: 35};
    let pixel_size: VECTOR2D = VECTOR2D {x: 20, y: 20};
    let game_area: BBOX = BBOX {xmin: 0,
                                ymin: 0,
                                xmax: 23,
                                ymax: 23};
    
    // Random food number
    let mut rng = rand::thread_rng();
    
    // Snake position
    let mut snake: OBJECT = OBJECT{x:5,y:5,real_x:0,real_y:0};
    let mut food: OBJECT = OBJECT{x:0,y:0,real_x:0,real_y:0};
    let mut food_exists: bool = false;
    let mut snake_dir:DIRECTION = DIRECTION::Right;
    
    // Snake speed
    let snake_speed: time::Duration = time::Duration::new(0, 100_000_000);
    let mut tic: time::Instant = time::Instant::now();
    let mut toc: time::Instant;

    // Track score
    let mut score: i64 = 0;
    let mut score_test: String;

    while !rl.window_should_close() {
        // Start Drawing
        let mut d: RaylibDrawHandle = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);

        // Draw score
        score_test = format!("Score: {score}");
        d.draw_text(score_test.as_str(), 12, 12, 20, Color::BLACK);
        d.draw_rectangle(0, 30, win_width, 5, Color::RED);
        // d.draw_fps(12,12);

        // Check for inputs
        snake_dir = get_input_dir(snake_dir, &d);

        // Update logic according to the snake speed
        toc = time::Instant::now();
        if (toc-tic) > snake_speed{
            tic = time::Instant::now();
            // Draw food
            if !food_exists
            {   
                food.x = rng.gen_range(game_area.xmin..game_area.xmax);
                food.y = rng.gen_range(game_area.ymin..game_area.ymax);
                update_real_pos(&mut food, &pixel_pad, &pixel_size);
                food_exists = true;
            }
            
            match snake_dir{
                DIRECTION::Up => {snake.y -= 1;},
                DIRECTION::Down => {snake.y += 1;},
                DIRECTION::Left => {snake.x -= 1;},
                DIRECTION::Right => {snake.x += 1;},
            };

            if snake.x == food.x && snake.y==food.y {
                food_exists = false;
                score += 1;
            };
            check_collision_box(&game_area, &mut snake);
            update_real_pos(&mut snake, &pixel_pad, &pixel_size);
        }
        // Player position
        d.draw_rectangle(snake.real_x, snake.real_y, pixel_size.x, pixel_size.y, Color::BLUE);
        // Food position
        d.draw_rectangle(food.real_x, food.real_y, pixel_size.x, pixel_size.y, Color::GREEN);
    }
        
}
