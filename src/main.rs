pub mod math;
pub mod render;

use math::*;
use miniquad::{window::screen_size, BlendState};
use objects::*;
use render::*;
use world::World;

use macroquad::{
    material,
    prelude::*,
    time,
    ui::{
        widgets::{Button, Group, Window},
        *,
    },
};
use std::{
    cell::{Ref, RefCell, RefMut},
    cmp::Ordering,
    collections::HashMap,
    rc::Rc,
};

mod objects;
mod world;
mod constants {
    pub const TILE_SIZE: (f32, f32) = (64.0, 64.0);
}
use constants::*;

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
pub(crate) enum PlayerOrient {
    _0 = 0,
    _45 = 45,
    _90 = 90,
    _135 = 135,
    _180 = 180,
    _225 = 225,
    _270 = 270,
    _315 = 315,
}
pub(crate) struct Game {
    block_trans_map: Vec<u32>,
    block_textures: Vec<Texture2D>,
    player_textures: HashMap<PlayerOrient, Texture2D>,
    player_object: Rc<RefCell<Player>>,
    world: World,
    debug: bool,
    draw_queue: Vec<Rc<RefCell<dyn Renderble>>>,
    block_material: Material,
    blocks_cover_player: bool,
    selected_id: u32,
    shade_top: Texture2D,
    shade_bot: Texture2D,
    selection_top: Texture2D,
    ui_selection_mode: bool,
    ui_selection_pos: Vec2,
    // buffer_queue: Vec<Rc<RefCell<dyn ISOGraphics>>>, // todo: a buffer for holding old data in draw queue to be moved out or into draw queue on player discovery of new visible chunk
}
impl Game {
    fn player(&self) -> Ref<Player> {
        self.player_object.as_ref().borrow()
    }
    fn player_mut(&self) -> RefMut<Player> {
        self.player_object.as_ref().borrow_mut()
    }
}
#[inline]
fn cmp_tiles(lhs: Vec3, rhs: Vec3) -> Ordering {
    (lhs.x + lhs.y + lhs.z)
        .partial_cmp(&(rhs.x + rhs.y + rhs.z))
        .unwrap()
}
#[test]
fn cmp_tiles_test() {
    assert_eq!(
        cmp_tiles((0., 0., 1.).into(), (1., 1., 2.).into()),
        Ordering::Less
    ); //scenario: player is below a tile that appears above the player
    assert_eq!(
        cmp_tiles((0., 0., 1.).into(), (-1., -1., 0.).into()),
        Ordering::Greater
    ); //scenario: player is below a tile that appears below the player
}
/// for when you want to get a point under a tile or object well centered for use with camera 2d
/// or screen space (for that you first need to use Camera::world_to_space function in order to transform that into screen space from 2d world space)
#[inline]
fn in_2d(pos: Vec3) -> Vec2 {
    let pp = flatten_iso(pos);
    let pp = world_to_is(pp, TILE_SIZE);

    pp.with_x(pp.x + TILE_SIZE.0 / 2.)
        .with_y(pp.y + TILE_SIZE.1 / 2.)
}
/// tests if a block exists on screen (not necesserly visible)
fn is_on_screen(pos: Vec3, cam: &Camera2D) -> bool {
    let r = Rect::new(0., 0., screen_width(), screen_width());
    let f = flatten_iso(pos);
    let f = tile_matrix(TILE_SIZE).inverse().mul_vec2(f);
    r.contains(cam.world_to_screen(f))
}
fn load_player_assets() -> HashMap<PlayerOrient, Texture2D> {
    let mut _player_textures = HashMap::new();
    _player_textures.insert(
        PlayerOrient::_225,
        Texture2D::from_file_with_format(
            include_bytes!("../resources/player/225.png"),
            Some(ImageFormat::Png),
        ),
    );
    _player_textures.insert(
        PlayerOrient::_315,
        Texture2D::from_file_with_format(
            include_bytes!("../resources/player/315.png"),
            Some(ImageFormat::Png),
        ),
    );
    _player_textures.insert(
        PlayerOrient::_45,
        Texture2D::from_file_with_format(
            include_bytes!("../resources/player/45.png"),
            Some(ImageFormat::Png),
        ),
    );
    _player_textures.insert(
        PlayerOrient::_135,
        Texture2D::from_file_with_format(
            include_bytes!("../resources/player/135.png"),
            Some(ImageFormat::Png),
        ),
    );
    _player_textures.insert(
        PlayerOrient::_270,
        Texture2D::from_file_with_format(
            include_bytes!("../resources/player/270.png"),
            Some(ImageFormat::Png),
        ),
    );
    _player_textures.insert(
        PlayerOrient::_90,
        Texture2D::from_file_with_format(
            include_bytes!("../resources/player/90.png"),
            Some(ImageFormat::Png),
        ),
    );
    _player_textures.insert(
        PlayerOrient::_180,
        Texture2D::from_file_with_format(
            include_bytes!("../resources/player/180.png"),
            Some(ImageFormat::Png),
        ),
    );
    _player_textures.insert(
        PlayerOrient::_0,
        Texture2D::from_file_with_format(
            include_bytes!("../resources/player/0.png"),
            Some(ImageFormat::Png),
        ),
    );
    _player_textures
        .iter_mut()
        .for_each(|f| f.1.set_filter(FilterMode::Nearest));
    _player_textures
}
async fn load_tiles_assets() -> Vec<Texture2D> {
    let mut tiles: Vec<Texture2D> = Vec::new();
    tiles.push(Texture2D::from_file_with_format(
        include_bytes!("../empty.png"),
        Some(ImageFormat::Png),
    ));
    // tiles.push(Texture2D::from_file_with_format(
    //     include_bytes!("../tile_select.png"),
    //     Some(ImageFormat::Png),
    // ));
    // tiles.push(Texture2D::from_file_with_format(
    //     include_bytes!("../tile_frame.png"),
    //     Some(ImageFormat::Png),
    // ));
    tiles.push(Texture2D::from_file_with_format(
        include_bytes!("../tile_stone.png"),
        Some(ImageFormat::Png),
    ));
    tiles.push(Texture2D::from_file_with_format(
        include_bytes!("../tile_dirt.png"),
        Some(ImageFormat::Png),
    ));
    tiles.push(Texture2D::from_file_with_format(
        include_bytes!("../tile_grass.png"),
        Some(ImageFormat::Png),
    ));
    tiles.push(Texture2D::from_file_with_format(
        include_bytes!("../tile_stone_smooth.png"),
        Some(ImageFormat::Png),
    ));
    tiles.push(Texture2D::from_file_with_format(
        include_bytes!("../tile.png"),
        Some(ImageFormat::Png),
    ));
    tiles.push(Texture2D::from_file_with_format(
        include_bytes!("../tile_gravel.png"),
        Some(ImageFormat::Png),
    ));
    tiles.push(Texture2D::from_file_with_format(
        include_bytes!("../tile_machine.png"),
        Some(ImageFormat::Png),
    ));
    // tles.push(load_texture("tile_machine.png").await.unwrap());
    for tile in &tiles {
        tile.set_filter(FilterMode::Nearest);
    }
    tiles
}
fn generate_world(world: &mut World) {
    for i in 0..50 {
        for j in 0..50 {
            //ground
            world.set_block(i, j, 0, 3);
            // hill
            // if (5..=8).contains(&i) && (3..=6).contains(&j) {
            // world.set_block(i, j, 2, 3);
            // world.set_block(i, j, 4, 3);
            // }
            // if i > 0 && i < 3 && j > 0 && j < 3 {
            // world.set_block(i, j, 4, 3);
            // }
        }
    }
    // world.set_block(10, 10, 1, 6);
    // world.set_block(11, 10, 1, 6);
    // world.set_block(12, 10, 1, 6);
    // world.set_block(10, 10, 3 + 2, 6);
    // world.set_block(11, 10, 3 + 2, 6);
    // world.set_block(12, 10, 3 + 2, 6);
}
const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");
const BUILD_TIME: Option<&str> = option_env!("SOURCE_DATE_EPOCH");
#[macroquad::main("Isometric Engine")]
async fn main() {
    let _quad_gl = unsafe { get_internal_gl().quad_gl };
    let _quad_context = unsafe { get_internal_gl().quad_context };
    let mut game = Game {
        block_trans_map: vec![0, 1, 2],
        selection_top: Texture2D::from_file_with_format(
            include_bytes!("../selection_top.png"),
            Some(ImageFormat::Png),
        ),

        shade_top: Texture2D::from_file_with_format(
            include_bytes!("../shade_top.png"),
            Some(ImageFormat::Png),
        ),
        shade_bot: Texture2D::from_file_with_format(
            include_bytes!("../shade_bot.png"),
            Some(ImageFormat::Png),
        ),
        selected_id: 3,
        blocks_cover_player: false,
        block_textures: load_tiles_assets().await,
        player_object: Rc::new(RefCell::new(objects::Player::new(
            vec3(0., 0., 1.),
            Vec3::ZERO,
        ))),
        world: world::World::new(),
        player_textures: load_player_assets(),
        debug: if cfg!(debug_assertions) { true } else { false },
        draw_queue: Vec::with_capacity(1000),
        block_material: material::load_material(
            ShaderSource::Glsl {
                vertex: include_str!("shader.vs"),
                fragment: include_str!("shader.fs"),
            },
            MaterialParams {
                uniforms: vec![
                    ("player_gl_pos".to_string(), UniformType::Float2),
                    ("mouse".to_string(), UniformType::Float2),
                    ("resolution".to_string(), UniformType::Float2),
                    ("resolution_cam".to_string(), UniformType::Float2),
                    ("camera_zoom".to_string(), UniformType::Float2),
                    ("player_dist".to_string(), UniformType::Float1),
                    ("player_world_pos".to_string(), UniformType::Float3),
                    ("block_world_pos".to_string(), UniformType::Float3),
                    ("player_hidble".to_string(), UniformType::Int1),
                    ("block_behind_player".to_string(), UniformType::Int1),
                    ("block_over_top".to_string(), UniformType::Int1),
                ],
                pipeline_params: PipelineParams {
                    depth_write: true,
                    depth_test: Comparison::LessOrEqual,
                    color_blend: Some(BlendState::new(
                        miniquad::Equation::Add,
                        miniquad::BlendFactor::Value(miniquad::BlendValue::SourceAlpha),
                        miniquad::BlendFactor::OneMinusValue(miniquad::BlendValue::SourceAlpha),
                    )),
                    alpha_blend: Some(BlendState::new(
                        miniquad::Equation::Add,
                        miniquad::BlendFactor::Zero,
                        miniquad::BlendFactor::One,
                    )),
                    ..Default::default()
                },
                ..Default::default()
            },
        )
        .unwrap(),
        ui_selection_mode: false,
        ui_selection_pos: Vec2::ZERO,
    };
    build_textures_atlas();
    generate_world(&mut game.world);
    let mut camera = Camera2D::from_display_rect(Rect {
        x: -500.,
        y: -500.,
        w: 1000.,
        h: 1000.,
    });
    camera.zoom = vec2(camera.zoom.x, -camera.zoom.y);
    let lower_limit = camera.zoom / 3.;
    let upper_limit = camera.zoom * 3.;

    // let mut draw_queue: Vec<Rc<RefCell<dyn ISOGraphics>>> = Vec::with_capacity(1000);
    game.draw_queue.push(game.player_object.clone());
    // unload blocks from storage into render queue
    // todo: Later do something with dynamic loading where we only load a portion of visible map
    for ele in game.world.blocks() {
        game.draw_queue
            .push(Rc::new(RefCell::new(objects::Block::new(ele.0, ele.1))));
    }
    let mut curser_pos_iso = vec2(0., 0.);
    loop {
        game.block_material.set_uniform("mouse", mouse_position());
        game.block_material.set_uniform("resolution", screen_size());
        game.block_material.set_uniform(
            "resolution_cam",
            camera.screen_to_world(screen_size().into()),
        );
        game.draw_queue.sort_by(|a, b| {
            let a = a.as_ref().borrow();
            let b = b.as_ref().borrow();
            (a.pos().y + a.pos().x + a.pos().z)
                .partial_cmp(&(b.pos().y + b.pos().x + b.pos().z))
                .unwrap()
        });

        set_camera(&camera);
        if mouse_wheel().1.abs() > 0. && !is_key_down(miniquad::KeyCode::LeftShift) {
            camera.zoom += mouse_wheel().1 * get_frame_time() * 0.0001;
            camera.zoom = camera.zoom.clamp(lower_limit, upper_limit);
        } else if mouse_wheel().1.abs() > 0. && is_key_down(miniquad::KeyCode::LeftShift) {
            if mouse_wheel().1.is_sign_positive() {
                if game.selected_id < (game.block_textures.len() - 1) as u32 {
                    game.selected_id += 1;
                } else {
                    game.selected_id = 1;
                }
            } else {
                if game.selected_id > 1 as u32 {
                    game.selected_id -= 1;
                } else {
                    game.selected_id = (game.block_textures.len() - 1) as u32;
                }
            }
        }
        if is_key_down(miniquad::KeyCode::Tab) {
            if game.selected_id < (game.block_textures.len() - 1) as u32 {
                game.selected_id += 1;
            } else {
                game.selected_id = 1;
            }
        }
        game.block_material.set_uniform("camera_zoom", camera.zoom);

        clear_background(BLACK);
        let camera_screen_world =
            camera.screen_to_world(vec2(mouse_position().0, mouse_position().1));
        let csw_in_isometric = iso_to_world(camera_screen_world, TILE_SIZE);
        let grid_pos = Vec2 {
            x: csw_in_isometric.x - 4.,
            y: csw_in_isometric.y - 4.,
        };
        if game.debug {
            draw_isometric_axis(vec2(0., 0.), 10., TILE_SIZE);
            draw_rectangle_lines(
                camera_screen_world.x - 50.,
                camera_screen_world.y - 50.,
                100.,
                100.,
                5.,
                WHITE,
            );
            draw_isometric_grid(grid_pos, 10., TILE_SIZE);
        }

        // update players physics
        let player_pos = game.player_object.as_ref().borrow().pos();
        game.block_material
            .set_uniform("player_world_pos", player_pos);
        let direction2d = -(player_pos.xy() - curser_pos_iso).normalize();
        let direction = vec3(
            direction2d.x,
            direction2d.y,
            game.player_object.as_ref().borrow().vel().z,
        );
        if is_key_down(miniquad::KeyCode::W) {
            if !direction.is_nan() && direction.length() > 0.5 {
                game.player_object.borrow_mut().set_vel(direction * 2.);
                // player.pos += direction * player_speed * get_frame_time();
            }
        } else {
            let z = game.player_object.as_ref().borrow().vel().z;
            game.player_object.borrow_mut().set_vel(vec3(0., 0., z));
        }
        // update physics
        let vel = game.player_object.as_ref().borrow().vel();
        let pos = game.player_object.as_ref().borrow().pos();
        if game
            .world
            .get_block_f((pos + vel * get_frame_time()).with_z(pos.z).floor() + vec3(1.0, 1.0, 0.0))
            != 0
        {
            let z = game.player_object.as_ref().borrow().vel().z;
            game.player_object.borrow_mut().set_vel(vec3(0., 0., z));
        }
        let vel = game.player_object.as_ref().borrow().vel();
        game.player_mut().set_pos(pos + vel * get_frame_time());
        for el in game.draw_queue.iter() {
            let renderable = el.as_ref().borrow();
            renderable.render(&game);
        }
        curser_pos_iso = vec2(csw_in_isometric.x.floor(), csw_in_isometric.y.ceil());
        let h_pos = vec2(curser_pos_iso.x, curser_pos_iso.y);
        if game.debug {
            draw_hexagon(
                tile_matrix(TILE_SIZE).mul_vec2(h_pos).x + TILE_SIZE.0 / 2.,
                tile_matrix(TILE_SIZE).mul_vec2(h_pos).y,
                TILE_SIZE.0 / 2.,
                1.,
                true,
                Color::new(
                    ((get_time() as f32).sin() + 1.0) / 2.0,
                    ((get_time() as f32).cos() + 1.0) / 2.,
                    1.0,
                    1.0,
                ),
                Color::new(0., 0., 0., 0.),
            );
        }
        // selection block
        {
            let tile_under_mouse = csw_in_isometric.floor();
            let mut t = (
                tile_under_mouse.x as usize + 1,
                tile_under_mouse.y as usize + 2,
                player_pos.z as usize,
            );
            let mut offset = 0;
            while game.world.get_block(t.0, t.1, t.2) != 0 {
                t.2 = t.2 + 1;
                offset += 1;
            }
            if player_pos.distance(vec3(t.0 as f32, t.1 as f32, t.2 as f32)) > 1.0 {
                draw_tile(
                    curser_pos_iso.x + 1. - offset as f32,
                    curser_pos_iso.y + 1. - offset as f32,
                    TILE_SIZE,
                    &game.selection_top,
                );
            }
        }
        push_camera_state();
        set_default_camera();
        let v = in_2d(player_pos);
        // send player position on screen to gpu
        // move camera with player
        camera.target = v;
        let v = camera.world_to_screen(v);
        game.block_material.set_uniform("player_gl_pos", v);
        let m: Vec2 = mouse_position().into();
        // hack: convert angle between player's pos to mouse pos to a format we care about
        // 0 starts at right goes to left and up and ends at 360 on right
        let a = (m - v).to_angle().to_degrees() - 180.; // -pi to +pi -> 0 to 2pi
        let mut a = a + 180.; // clockwise to c-clockwise
                              // teach computers to do -theta = 360.-theta
        if a > 360. {
            a -= 360.;
        }
        a = -a;
        if a.is_sign_negative() {
            a += 360.;
        }
        let tile_under_mouse = csw_in_isometric.floor();
        // place block on the mouse click
        if is_mouse_button_pressed(MouseButton::Left)
            || (is_mouse_button_down(MouseButton::Left) && is_key_down(miniquad::KeyCode::LeftControl))
            && /*works partialy*/ !root_ui().is_mouse_over(vec2(mouse_position().0,mouse_position().1))
        {
            let mut t = (
                tile_under_mouse.x as usize + 1,
                tile_under_mouse.y as usize + 2,
                player_pos.z as usize,
            );
            while game.world.get_block(t.0, t.1, t.2) != 0 {
                t.2 = t.2 + 1;
            }
            if player_pos.distance(vec3(t.0 as f32, t.1 as f32, t.2 as f32)) > 1.0 {
                game.world.set_block(t.0, t.1, t.2, game.selected_id as u8);
                // reload the draw queue to get all the new blocks
                game.draw_queue.clear();
                game.draw_queue.push(game.player_object.clone());
                for ele in game.world.blocks() {
                    game.draw_queue
                        .push(Rc::new(RefCell::new(objects::Block::new(ele.0, ele.1))));
                }
            }
        }
        game.player_mut().update_orientation(a);
        if game.debug {
            draw_line(v.x, v.y, m.x, m.y, 2., GREEN);
            draw_circle(v.x, v.y, 10., RED); // draw a point under the player
            draw_text(
                format!("{}", tile_under_mouse).as_str(),
                mouse_position().0,
                mouse_position().1,
                18.,
                YELLOW,
            );
        }
        pop_camera_state();
        // draw a tile over the player for debug reasons
        if game.debug {
            draw_tile(
                flatten_iso(player_pos).x,
                flatten_iso(player_pos).y,
                TILE_SIZE,
                &game.block_textures[1],
            );
        }
        root_ui().group(hash!(), vec2(200., 400.), |ui| {
            ui.button(
                None,
                format!("V:{}\n B:{}", VERSION.unwrap(), BUILD_TIME.unwrap_or("NAN")).as_str(),
            );
            game.debug = if ui.button(None, format!("Debug Mode: {}", game.debug).as_str()) {
                !game.debug
            } else {
                game.debug
            };
            ui.button(
                None,
                format!("Player: {}", game.player_object.as_ref().borrow().pos()).as_str(),
            );
            ui.button(None, format!("Cursor: {tile_under_mouse}").as_str());
            ui.button(None, format!("FPS: {}", get_fps()).as_str());
            ui.button(
                None,
                format!(
                    "Player Orientation: {:?}",
                    game.player_object.as_ref().borrow().orient
                )
                .as_str(),
            );
            ui.button(
                None,
                format!("Render Queue: {}", game.draw_queue.len()).as_str(),
            );
            if ui.button(None, "Toggle Player Fog") {
                let x = if game.blocks_cover_player { 1 } else { 0 };
                game.blocks_cover_player = !game.blocks_cover_player;
                game.block_material.set_uniform("player_hidble", x);
            }
            ui.button(
                None,
                format!("Current BlockID: {}", game.selected_id).as_str(),
            );
            ui.texture(
                game.block_textures[game.selected_id as usize].clone(),
                32.0,
                32.0,
            );
        });
        if is_mouse_button_pressed(MouseButton::Right) {
            game.ui_selection_mode = !game.ui_selection_mode;
            if game.ui_selection_mode {
                game.ui_selection_pos = mouse_position().into();
            } else {
            }
        }
        if game.ui_selection_mode {
            root_ui().button(game.ui_selection_pos, "Select Block:");
            for (id, t) in game.block_textures.iter().enumerate() {
                if id == 0 {
                    continue;
                }
                let s = 32.;
                root_ui().canvas().image(
                    Rect::new(
                        game.ui_selection_pos.x,
                        game.ui_selection_pos.y + (s + 2.0) * id as f32,
                        s,
                        s,
                    ),
                    t,
                );
            }
        }
        let z = match get_last_key_pressed() {
            Some(macroquad::input::KeyCode::Key1) => Some(1),
            Some(macroquad::input::KeyCode::Key2) => Some(2),
            Some(macroquad::input::KeyCode::Key3) => Some(3),
            Some(macroquad::input::KeyCode::Key4) => Some(4),
            Some(macroquad::input::KeyCode::Key5) => Some(5),
            Some(macroquad::input::KeyCode::Key6) => Some(6),
            Some(macroquad::input::KeyCode::Key7) => Some(7),
            _ => None,
        };
        if z.is_some() {
            game.selected_id = z.unwrap();
        }
        next_frame().await;
    }
}
