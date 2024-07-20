use macroquad::{
    color::{hsl_to_rgb, rgb_to_hsl, WHITE},
    logging::debug,
    math::{vec3, Vec2, Vec3, Vec3Swizzles},
};

use crate::{
    constants, draw_tile, draw_tile_margin_color, space_to_iso, tile, Game, PlayerOrient, TILE_SIZE,
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
pub trait ISOPhysic {
    fn vel(&self) -> Vec3;
    fn set_vel(&mut self, vel: Vec3);
}

pub trait ISOObject {
    fn pos(&self) -> Vec3;
    fn set_pos(&mut self, pos: Vec3);
}

pub trait ISOGraphics: ISOObject {
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
    pub fn update_orient(&mut self, degrees: f32) {
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

impl ISOObject for Player {
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

impl ISOObject for Block {
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
impl ISOGraphics for Block {
    fn render(&self, game_state: &Game) {
        let mut c = WHITE;
        let mut light = 1.;
        let player_pos = game_state.player_object.clone().as_ref().borrow().pos();

        for i in 1..8 {
            if game_state
                .world
                .get_block_f(self.pos + vec3(0., 0., i as f32))
                != 0
            {
                light = (i as f32) / 8.;
                break;
            }
        }

        let (h, s, mut l) = rgb_to_hsl(c);
        l = light;
        c = hsl_to_rgb(h, s, l);

        if (self.pos.z - player_pos.z).abs() > 8. {
            // dont render if a block is 8 block down or up from player
            return;
        } else {
            let player_pos_i = space_to_iso(player_pos);
            let p = space_to_iso(self.pos);
            let dist_to_player = (player_pos_i - p).length().abs();
            let a = if dist_to_player < 4. {
                1.0
            } else if dist_to_player < 5. {
                0.8
            } else if dist_to_player < 6. {
                0.3
            } else if dist_to_player < 8. {
                0.1
            } else {
                dist_to_player.recip()
            };
            // this kinda cool, uncomment this if u want it to look like light
            // let a = dist_to_player.recip() * 1.2;
            c.a = a;
            draw_tile_margin_color(
                p.x,
                p.y,
                TILE_SIZE,
                &game_state.block_textures[self.block_id as usize],
                1.,
                c,
            );
        }
    }
}

impl ISOGraphics for Player {
    fn render(&self, game_state: &Game) {
        let p = space_to_iso(self.pos);
        let t = game_state.player_textures.get(&self.orient).unwrap();
        draw_tile(p.x, p.y, constants::TILE_SIZE, t)
    }
}
/*
   Physics
*/
impl ISOPhysic for Player {
    fn vel(&self) -> Vec3 {
        self.vel
    }

    fn set_vel(&mut self, vel: Vec3) {
        self.vel = vel;
    }
}
