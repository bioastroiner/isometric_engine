use std::cmp::Ordering::{Equal, Greater, Less};

use macroquad::{
    camera::{pop_camera_state, push_camera_state, set_camera, set_default_camera},
    color::{hsl_to_rgb, rgb_to_hsl, WHITE},
    logging::debug,
    math::{vec3, FloatExt, Rect, Vec2, Vec3, Vec3Swizzles},
    prelude::{gl_use_default_material, gl_use_material},
    shapes::{draw_rectangle, draw_rectangle_lines},
    time::get_time,
};

use crate::{
    cmp_tiles, constants, draw_tile, draw_tile_ex, flatten_iso, render, DrawTilesParams, Game,
    PlayerOrient, TILE_SIZE,
};

#[derive(Debug)]
pub struct Player {
    pos: Vec3,
    vel: Vec3,
    pub is_jumping: bool,
    pub orient: PlayerOrient,
}

pub struct Block {
    pub block_id: u8,
    pos: Vec3,
}
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

impl Player {
    pub fn new(pos: Vec3, vel: Vec3) -> Self {
        Player {
            pos,
            vel,
            is_jumping: true,
            orient: PlayerOrient::_45,
        }
    }
    pub fn update_orientation(&mut self, degrees: f32) {
        let q: f32 = 45. / 2.;
        let ors = &[
            // PlayerOrient::_0,
            PlayerOrient::_45,
            PlayerOrient::_90,
            PlayerOrient::_135,
            PlayerOrient::_180,
            PlayerOrient::_225,
            PlayerOrient::_270,
            PlayerOrient::_315,
        ];
        if degrees > 360. - q || degrees < q {
            self.orient = PlayerOrient::_0;
            return;
        } else {
            for e in ors {
                if degrees > *e as i32 as f32 - q && degrees < *e as i32 as f32 + q {
                    self.orient = *e;
                    return;
                }
            }
        }
    }
}

impl Positionable for Player {
    fn pos(&self) -> Vec3 {
        self.pos
    }

    fn set_pos(&mut self, pos: Vec3) {
        self.pos = pos;
    }
}

impl Block {
    pub fn new(pos: Vec3, block_id: u8) -> Block {
        Block { block_id, pos }
    }
}

impl Positionable for Block {
    fn pos(&self) -> Vec3 {
        self.pos
    }

    fn set_pos(&mut self, pos: Vec3) {
        self.pos = pos
    }
}
/*
   Graphics
*/
const RENDER_DISTANCE: f32 = 8.;
impl Renderble for Block {
    fn render(&self, game_state: &Game) {
        let mut c = WHITE;
        let player_pos = game_state.player().pos();
        let player_pos_i = flatten_iso(player_pos);
        let p = flatten_iso(self.pos);
        let dist_to_player = (player_pos_i - p).length().abs();
        game_state
            .block_material
            .set_uniform("player_dist", dist_to_player);
        game_state.block_material.set_uniform("block_world_pos", self.pos);
        gl_use_material(&game_state.block_material);
        draw_tile_ex(
            p.x,
            p.y,
            TILE_SIZE,
            &game_state.block_textures[self.block_id as usize],
            DrawTilesParams {
                color: c,
                ..Default::default()
            },
        );
        gl_use_default_material();
    }
}

impl Renderble for Player {
    fn render(&self, game_state: &Game) {
        let p = flatten_iso(self.pos);
        let t = game_state.player_textures.get(&self.orient).unwrap();
        draw_tile(p.x, p.y, constants::TILE_SIZE, t)
    }
}
/*
   Physics
*/
impl Physical for Player {
    fn vel(&self) -> Vec3 {
        self.vel
    }

    fn set_vel(&mut self, vel: Vec3) {
        self.vel = vel;
    }

    fn collision_box(&self) -> Option<Rect> {
        todo!()
    }
}
