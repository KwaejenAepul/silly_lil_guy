use macroquad::prelude::*;

pub fn spritesheet_to_vec(image: Image, size: f32, sprite_h: f32) -> Vec<Texture2D> {
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
