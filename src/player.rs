use macroquad::prelude::*;

#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
#[derive(Debug)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}
#[derive(Debug)]
pub struct Player {
    speed: f32,
    position: Position,
    direction: Direction,
    textures: Vec<Texture2D>,
    animation_timer: f32,
}
impl Player {
    pub fn create_player(
        speed: f32,
        position: Position,
        direction: Direction,
        texture: Vec<Texture2D>,
        animation_timer: f32,
    ) -> Player {
        Player {
            speed: speed,
            position: position,
            direction: direction,
            textures: texture,
            animation_timer: animation_timer,
        }
    }
    pub fn update(&mut self) {
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

    pub fn draw(&mut self) {
        let mut image: &Texture2D;
        match self.direction {
            Direction::Up => image = &self.textures[9],
            Direction::Down => image = &self.textures[0],
            Direction::Left => image = &self.textures[3],
            Direction::Right => image = &self.textures[6],
        }
        if is_key_down(KeyCode::Up) {
            if self.animation_timer < 0.25 {
                image = &self.textures[10];
                self.animation_timer += get_frame_time();
            } else {
                image = &self.textures[11];
                self.animation_timer += get_frame_time();
            }
        } else if is_key_down(KeyCode::Down) {
            if self.animation_timer < 0.25 {
                image = &self.textures[1];
                self.animation_timer += get_frame_time();
            } else {
                image = &self.textures[2];
                self.animation_timer += get_frame_time();
            }
        } else if is_key_down(KeyCode::Left) {
            if self.animation_timer < 0.25 {
                image = &self.textures[4];
                self.animation_timer += get_frame_time();
            } else {
                image = &self.textures[5];
                self.animation_timer += get_frame_time();
            }
        } else if is_key_down(KeyCode::Right) {
            if self.animation_timer < 0.25 {
                image = &self.textures[7];
                self.animation_timer += get_frame_time();
            } else {
                image = &self.textures[8];
                self.animation_timer += get_frame_time();
            }
        }
        if self.animation_timer > 0.5 {
            self.animation_timer = 0.0;
        }

        draw_texture(image, self.position.x, self.position.y, WHITE);
    }
}
