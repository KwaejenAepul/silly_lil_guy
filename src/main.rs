use macroquad::prelude::*;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}
struct Position {
    x: f32,
    y: f32,
}
struct Player {
    speed: f32,
    position: Position,
    size: f32,
    direction: Direction,
}

#[macroquad::main("silly lil guy")]
async fn main() {
    let mut player = Player {
        speed: 20.0,
        position: Position { x: 200.0, y: 200.0 },
        size: 32.0,
        direction: Direction::Down,
    };

    loop {
        clear_background(BLACK);

        if is_key_down(KeyCode::Up) {
            player.direction = Direction::Up;
            player.position.y -= player.speed;
        } else if is_key_down(KeyCode::Down) {
            player.direction = Direction::Down;
            player.position.y += player.speed;
        } else if is_key_down(KeyCode::Left) {
            player.direction = Direction::Left;
            player.position.x -= player.speed;
        } else if is_key_down(KeyCode::Right) {
            player.direction = Direction::Right;
            player.position.x += player.speed;
        }
        draw_rectangle(
            player.position.x,
            player.position.y,
            player.size,
            player.size,
            WHITE,
        );
        draw_text("HELLO", 20.0, 20.0, 20.0, DARKGRAY);

        next_frame().await
    }
}
