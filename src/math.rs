

/****************************************
   Extention of Math Utilities
   related to isometric grid
   Linear Algebra
****************************************/
use macroquad::math::*;

#[inline]
pub fn tile_matrix(tile_size: (f32, f32)) -> Mat2 {
    let (w, h) = tile_size;
    Mat2::from_cols_array(&[0.5 * w, -0.5 * w, 0.25 * h, 0.25 * h]).transpose()
}
#[inline]
pub fn transform_tile(x: f32, y: f32, tile_size: (f32, f32)) -> (f32, f32) {
    let mat = tile_matrix(tile_size);
    (mat.mul_vec2(vec2(x, y)).x, mat.mul_vec2(vec2(x, y)).y)
}
/// consumes a 2d coordinates and converts it to a 3d isometric coordinate
#[inline]
pub fn to_iso(v_2d: Vec2, tile_size: (f32, f32)) -> Vec2 {
    tile_matrix(tile_size).mul_vec2(v_2d)
}
/// consumes a 3d isometric coordinates and converts it to a 2d coordinate
#[inline]
pub fn from_iso(v_iso: Vec2, tile_size: (f32, f32)) -> Vec2 {
    tile_matrix(tile_size).inverse().mul_vec2(v_iso)
}

/// flattens virtual 3d space coordinate to 2d isometric coordinate
///
/// Note: for each z value we move x and y coordinate down until the z=0
/// that's where our tile will land on
#[inline]
pub fn space_to_iso(space: Vec3) -> Vec2 {
    vec2(space.x - space.z, space.y - space.z)
}
