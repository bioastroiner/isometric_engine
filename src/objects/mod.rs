use macroquad::{
    color::WHITE,
    math::{vec3, Rect, Vec3},
    prelude::{gl_use_default_material, gl_use_material},
};

use crate::{draw_tile_ex, DrawTilesParams, Game, TILE_SIZE};

pub mod block;
pub mod machine;
pub mod player;

pub trait Positionable {
    fn pos(&self) -> Vec3;
    fn set_pos(&mut self, pos: Vec3);
}
pub trait Physical: Positionable {
    fn vel(&self) -> Vec3;
    fn set_vel(&mut self, vel: Vec3);
    fn collision_box(&self) -> Option<Rect>;
}
pub trait Renderble: Positionable {
    fn render(&self, state: &Game);
}
pub trait Inventory {
    fn get_inputs();
    fn get_outputs(); // optional, None for general Inventory
}
