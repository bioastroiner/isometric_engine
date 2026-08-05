/*************************************
   This Module porpouse is
   to ease rendering graphics in an
   isometric grid.

   Note: this module is in the lowest level possible
   meaning it will be completely unaware of z-axis
   and directly works with xy isometric plain
*************************************/
use macroquad::prelude::*;

use crate::math::screen_to_iso;
#[inline]
pub fn draw_tile_color(x: f32, y: f32, tile_size: (f32, f32), texture: &Texture2D, _color: Color) {
    let v = screen_to_iso(Vec2::new(x - 1., y - 1.), tile_size);
    draw_texture_ex(
        texture,
        v.x,
        v.y,
        WHITE,
        DrawTextureParams {
            dest_size: Some(tile_size.into()),
            ..Default::default()
        },
    );
}
#[derive(Debug, Clone)]
pub struct DrawTilesParams {
    pub margin: (f32, f32),
    pub color: Color,
}
impl Default for DrawTilesParams {
    fn default() -> Self {
        Self {
            margin: Default::default(),
            color: WHITE,
        }
    }
}
pub fn draw_tile_ex(
    x: f32,
    y: f32,
    tile_size: (f32, f32),
    texture: &Texture2D,
    options: DrawTilesParams,
) {
    let v = screen_to_iso(Vec2::new(x - 1., y - 1.), tile_size);
    draw_texture_ex(
        texture,
        v.x + options.margin.0,
        v.y + options.margin.1,
        options.color,
        DrawTextureParams {
            dest_size: Some(
                (
                    tile_size.0 - options.margin.0,
                    tile_size.1 - options.margin.1,
                )
                    .into(),
            ),
            ..Default::default()
        },
    );
}
#[inline]
pub fn draw_tile(x: f32, y: f32, tile_size: (f32, f32), texture: &Texture2D) {
    draw_tile_ex(x, y, tile_size, texture, DrawTilesParams::default());
}

#[inline]
pub fn draw_isometric_axis(at_isometric: Vec2, length: f32, tile_size: (f32, f32)) {
    /* X Axis */
    let v1 = screen_to_iso(
        Vec2::new(0. + at_isometric.x, 0. + at_isometric.y),
        tile_size,
    );
    let v2 = screen_to_iso(
        Vec2::new(length + at_isometric.x, 0. + at_isometric.y),
        tile_size,
    );
    draw_line(v1.x, v1.y, v2.x, v2.y, 2.0, GREEN);
    /* Y Axis */
    let v1 = screen_to_iso(
        Vec2::new(0. + at_isometric.x, 0. + at_isometric.y),
        tile_size,
    );
    let v2 = screen_to_iso(
        Vec2::new(0. + at_isometric.x, length + at_isometric.y),
        tile_size,
    );
    draw_line(v1.x, v1.y, v2.x, v2.y, 2.0, RED);
    /* fake Z Axis */
    let v1 = screen_to_iso(
        Vec2::new(0. + at_isometric.x, 0. + at_isometric.y),
        tile_size,
    );
    let v2 = screen_to_iso(
        Vec2::new(-length + at_isometric.x, -length + at_isometric.y),
        tile_size,
    );
    draw_line(v1.x, v1.y, v2.x, v2.y, 2.0, BLUE);
}

#[inline]
// rather expensive
pub fn draw_isometric_grid(at_isometric: Vec2, length: f32, tile_size: (f32, f32)) {
    let v = at_isometric.floor();
    for i in 1..(length as i32) {
        let v1 = screen_to_iso(Vec2::new(v.x, v.y + i as f32), tile_size);
        let v2 = screen_to_iso(Vec2::new(v.x + length.floor(), v.y + i as f32), tile_size);
        draw_line(v1.x, v1.y, v2.x, v2.y, 3., GREEN);
        let v1 = screen_to_iso(Vec2::new(v.x + i as f32, v.y), tile_size);
        let v2 = screen_to_iso(Vec2::new(v.x + i as f32, v.y + length.floor()), tile_size);
        draw_line(v1.x, v1.y, v2.x, v2.y, 3., RED);
    }
}
