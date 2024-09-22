use macroquad::{
    color::WHITE,
    math::{Rect, Vec3, vec3},
    prelude::{gl_use_default_material, gl_use_material},
};

use crate::{
    constants, draw_tile, draw_tile_ex, flatten_iso, DrawTilesParams, Game, PlayerOrient, TILE_SIZE,
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
    pub fn bbox(&self,pos: Vec3) -> bool {
	/*
  bool PointIsInCube( Point3D p, float x_max, float x_min, float y_max, float y_min, float z_max, float z_min)
  {
        return (p.x <= x_max && p.x >= x_min) && (p.y <= y_max && p.y >= y_min) && (p.z <= z_max && p.z >= z_min);
  }
	*/
	(pos.x <= self.pos.x + 1.0 && pos.x >= self.pos.x) &&
	(pos.y <= self.pos.y + 1.0 && pos.y >= self.pos.y) &&
	(pos.z <= self.pos.z + 1.0 && pos.z >= self.pos.z)
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
        let c = WHITE;
        let player_pos = game_state.player().pos();
        let player_pos_i = flatten_iso(player_pos);
        let p = flatten_iso(self.pos);
        let dist_to_player = (player_pos_i - p).length().abs();
	let h = if player_pos_i.x < self.pos.x - 1.0 && player_pos_i.y < self.pos.y - 1.0 {
	    0
	} else {
	    1
	};
	let top = game_state.world.get_block_f(self.pos + vec3(0.0,0.0,1.0));
	game_state.block_material.set_uniform("block_behind_player",h);
	game_state.block_material.set_uniform("block_over_top",top as u32);
        game_state
            .block_material
            .set_uniform("player_dist", dist_to_player);
        game_state
            .block_material
            .set_uniform("block_world_pos", self.pos);
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
	// draw the shade if it's right under a block and is visible (like when player is seeing trough)
	if top != 0 && (dist_to_player < 5.0 && !game_state.block_trans_map.contains(&(top as u32))) {
	 draw_tile_ex(
            p.x,
            p.y,
            TILE_SIZE,
            &game_state.shade_top,
            DrawTilesParams {
                color: c,
                ..Default::default()
            },
        );   
	}
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
