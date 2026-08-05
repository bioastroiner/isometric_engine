use std::collections::HashMap;

use macroquad::{
    math::{vec3, Rect, Vec3},
    prelude::ImageFormat,
    texture::{FilterMode, Texture2D},
    time::get_frame_time,
};

use crate::{
    constants,
    math::flatten_iso,
    objects::{Physical, Positionable, Renderble},
    render::draw_tile,
    world::World,
    Game,
};
#[derive(Eq, PartialEq, Hash, Debug, Clone)]
pub enum PlayerOrient {
    _0 = 0,
    _45 = 45,
    _90 = 90,
    _135 = 135,
    _180 = 180,
    _225 = 225,
    _270 = 270,
    _315 = 315,
}
#[derive(Debug, PartialEq)]
pub struct Player {
    pub pos: Vec3,
    pub vel: Vec3,
    pub is_jumping: bool,
    pub orient: PlayerOrient,
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
                if degrees > e.clone() as i32 as f32 - q && degrees < e.clone() as i32 as f32 + q {
                    self.orient = e.clone();
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

pub fn load_player_assets() -> HashMap<PlayerOrient, Texture2D> {
    let mut _player_textures = HashMap::new();
    _player_textures.insert(
        PlayerOrient::_225,
        Texture2D::from_file_with_format(
            include_bytes!("../../resources/player/225.png"),
            Some(ImageFormat::Png),
        ),
    );
    _player_textures.insert(
        PlayerOrient::_315,
        Texture2D::from_file_with_format(
            include_bytes!("../../resources/player/315.png"),
            Some(ImageFormat::Png),
        ),
    );
    _player_textures.insert(
        PlayerOrient::_45,
        Texture2D::from_file_with_format(
            include_bytes!("../../resources/player/45.png"),
            Some(ImageFormat::Png),
        ),
    );
    _player_textures.insert(
        PlayerOrient::_135,
        Texture2D::from_file_with_format(
            include_bytes!("../../resources/player/135.png"),
            Some(ImageFormat::Png),
        ),
    );
    _player_textures.insert(
        PlayerOrient::_270,
        Texture2D::from_file_with_format(
            include_bytes!("../../resources/player/270.png"),
            Some(ImageFormat::Png),
        ),
    );
    _player_textures.insert(
        PlayerOrient::_90,
        Texture2D::from_file_with_format(
            include_bytes!("../../resources/player/90.png"),
            Some(ImageFormat::Png),
        ),
    );
    _player_textures.insert(
        PlayerOrient::_180,
        Texture2D::from_file_with_format(
            include_bytes!("../../resources/player/180.png"),
            Some(ImageFormat::Png),
        ),
    );
    _player_textures.insert(
        PlayerOrient::_0,
        Texture2D::from_file_with_format(
            include_bytes!("../../resources/player/0.png"),
            Some(ImageFormat::Png),
        ),
    );
    _player_textures
        .iter_mut()
        .for_each(|f| f.1.set_filter(FilterMode::Nearest));
    _player_textures
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

pub fn is_player_colliding_with_solid_block(world: &World, player: &Player) -> bool {
    let vel = player.vel();
    let pos = player.pos();
    world.get_block_f((pos + vel * get_frame_time()).with_z(pos.z).floor() + vec3(1.0, 1.0, 0.0))
        != 0
}
pub fn is_player_colliding_with_solid_ground(world: &World, player: &Player) -> bool {
    let vel = player.vel();
    let pos = player.pos();
    world.get_block_f(
        (pos + vel * get_frame_time())
            .with_x(pos.x)
            .floor()
            .with_y(pos.y)
            .floor()
            + vec3(0.0, 0.0, -1.0),
    ) != 0
}
