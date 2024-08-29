use bevy::prelude::*;

pub fn draw(
    UVec2 { x, y }: UVec2,
    pixels: &mut Vec<u8>,
    UVec2 {
        x: width,
        y: height,
    }: UVec2,
) {
    let index = x % width;
    pixels[index as usize] = 255;
}
