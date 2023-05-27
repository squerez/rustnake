use rand::Rng;
use raylib::prelude::*;

pub struct BBOX{
    pub xmin: i32,
    pub ymin: i32,
    pub xmax: i32,
    pub ymax: i32,
}

#[derive(Copy, Clone)]
pub enum DIRECTION{
    Up, Down, Left, Right
}

pub struct SNAKE{
    pub body: [VECTOR2D; 600],
    pub direction: DIRECTION,
}

impl SNAKE{
    pub fn new() -> SNAKE{
        SNAKE{body: [VECTOR2D{x: 0, y:0}; 600], direction: DIRECTION::Right}
    }
}

#[derive(Copy, Clone)]
pub struct VECTOR2D{
    pub x: i32,
    pub y: i32,
}

pub fn update_snake(snake: &mut SNAKE, count: i32, bounding_box: &BBOX) -> bool{
    
    // Update tail
    for i in (1..(count+1) as usize).rev(){
        snake.body[i] = snake.body[i-1];
    }

    // Update head
    match snake.direction{
        DIRECTION::Up => {snake.body[0].y -= 1;},
        DIRECTION::Down => {snake.body[0].y += 1;},
        DIRECTION::Left => {snake.body[0].x -= 1;},
        DIRECTION::Right => {snake.body[0].x += 1;},
    };
    check_collision_box(&bounding_box, &mut snake.body[0]);

    // Check for self collision
    for i in 1..(count+1) as usize{
        if snake.body[0].x == snake.body[i].x &&
           snake.body[0].y == snake.body[i].y{
            return false;
        }
    }
    return true;

}

pub fn check_collision_box(bounding_box: &BBOX, object: &mut VECTOR2D){
    if object.x<bounding_box.xmin {object.x = bounding_box.xmax};
    if object.x>bounding_box.xmax {object.x = bounding_box.xmin};
    if object.y<bounding_box.ymin {object.y = bounding_box.ymax};
    if object.y>bounding_box.ymax {object.y = bounding_box.ymin};
}

pub fn update_real_pos(object: &VECTOR2D, padding: &VECTOR2D, size: &VECTOR2D) -> VECTOR2D{
    let x: i32 = object.x * size.x + padding.x;
    let y: i32 = object.y * size.y + padding.y;
    let real_coord: VECTOR2D = VECTOR2D {x: x, y: y};
    return real_coord;
}

pub fn get_input_dir(mut direction:DIRECTION, prev_direction:DIRECTION, handle: &RaylibDrawHandle) -> DIRECTION{
    if handle.is_key_down(KeyboardKey::KEY_W) {
        if !matches!(prev_direction, DIRECTION::Down) {direction = DIRECTION::Up};
    };
    if handle.is_key_down(KeyboardKey::KEY_A) {
        if !matches!(prev_direction, DIRECTION::Right) {direction = DIRECTION::Left};
    };
    if handle.is_key_down(KeyboardKey::KEY_S) {
        if !matches!(prev_direction, DIRECTION::Up) {direction = DIRECTION::Down};
    };
    if handle.is_key_down(KeyboardKey::KEY_D) {
        if !matches!(prev_direction, DIRECTION::Left) {direction = DIRECTION::Right};
    };
    return direction;
}

pub fn gen_food(object: &mut VECTOR2D, bounding_box: &BBOX){
    // Random food number
    let mut rng = rand::thread_rng();
    object.x = rng.gen_range(bounding_box.xmin..bounding_box.xmax);
    object.y = rng.gen_range(bounding_box.ymin..bounding_box.ymax);
}