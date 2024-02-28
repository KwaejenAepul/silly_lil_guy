use macroquad::prelude::*;

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
#[derive(Debug)]
struct Position {
    x: f32,
    y: f32,
}
#[derive(Debug)]
struct Player {
    speed: f32,
    position: Position,
    size: f32,
    direction: Direction,
    // textures: Vec<Texture2D>,
    //animation_timer: f32,
}
impl Player {
    fn update(&mut self) {
        if is_key_down(KeyCode::Up) {
            self.direction = Direction::Up;
            self.position.y -= self.speed * get_frame_time();
        } else if is_key_down(KeyCode::Down) {
            self.direction = Direction::Down;
            self.position.y += self.speed * get_frame_time();
        } else if is_key_down(KeyCode::Left) {
            self.direction = Direction::Left;
            self.position.x -= self.speed * get_frame_time();
        } else if is_key_down(KeyCode::Right) {
            self.direction = Direction::Right;
            self.position.x += self.speed * get_frame_time();
        }
    }

    fn draw(&self) {
        draw_rectangle(
            self.position.x,
            self.position.y,
            self.size,
            self.size,
            WHITE,
        );
    }
}

fn spritesheet_to_vec(image: Image, size: f32, sprite_h: f32) -> Vec<Texture2D> {
    let mut spritevec: Vec<Texture2D> = Vec::new();
    let mut i: f32 = 0.0;
    while i < sprite_h {
        let subimage = image.sub_image(macroquad::math::Rect {
            x: i * size,
            y: 0.0,
            w: size,
            h: size,
        });
        i += 1.0;
        spritevec.push(Texture2D::from_image(&subimage));
    }
    spritevec
}

#[macroquad::main("silly lil guy")]
async fn main() {
    //for loop to get textures
    let image = load_image("assets/lil_guy_sprites.png").await.unwrap();
    let spritevec = spritesheet_to_vec(image, 32.0, 12.0);
    let mut player = Player {
        speed: 500.0,
        position: Position { x: 200.0, y: 200.0 },
        size: 16.0,
        direction: Direction::Down,
    };

    loop {
        clear_background(BLACK);
        player.update();
        player.draw();
        draw_texture(&spritevec[0], 20.0, 20.0, WHITE);
        let fps = format!("fps:{}", macroquad::time::get_fps());
        draw_text(&fps, 20.0, 20.0, 20.0, DARKGRAY);

        next_frame().await
    }
}
