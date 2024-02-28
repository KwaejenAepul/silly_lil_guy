use macroquad::prelude::*;
mod imageconverter;
mod player;


#[macroquad::main("silly lil guy")]
async fn main() {
    let image = load_image("assets/lil_guy_sprites.png").await.unwrap();
    let spritevec = imageconverter::spritesheet_to_vec(image, 32.0, 12.0);
    let mut player = player::Player {
        speed: 500.0,
        position: player::Position { x: 200.0, y: 200.0 },
        direction: player::Direction::Down,
        textures: spritevec,
        animation_timer: 0.0,
    };

    loop {
        clear_background(PURPLE);
        player.update();
        player.draw();
        let fps = format!("fps:{}", macroquad::time::get_fps());
        draw_text(&fps, 20.0, 20.0, 20.0, DARKGRAY);

        next_frame().await
    }
}
