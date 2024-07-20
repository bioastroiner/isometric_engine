pub mod math;
pub mod tile;

use math::*;
use objects::{ISOGraphics, ISOObject, ISOPhysic, Player};
use tile::*;
use world::World;

use std::{borrow::Borrow, cell::RefCell, collections::HashMap, rc::Rc};

use macroquad::{
    prelude::*,
    ui::{hash, root_ui},
};

mod objects;
mod world;
mod constants {
    pub const TILE_SIZE: (f32, f32) = (64.0, 64.0);
}
use constants::*;
#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
enum PlayerOrient {
    _0 = 0,
    _45 = 45,
    _90 = 90,
    _135 = 135,
    _180 = 180,
    _225 = 225,
    _270 = 270,
    _315 = 315,
}
struct Game {
    block_textures: Vec<Texture2D>,
    player_texture: Texture2D,
    player_textures: HashMap<PlayerOrient, Texture2D>,

    player_object: Rc<RefCell<Player>>,
    world: Box<World>,
}
/// for when you want to get a point under a tile or object well centered for use with camera 2d
/// or screen space (for that you first need to use Camera::world_to_space function in order to transform that into screen space from 2d world space)
#[inline]
fn in_2d(pos: Vec3) -> Vec2 {
    let pp = pos;
    let pp = space_to_iso(pos);
    let pp = to_iso(pp, TILE_SIZE);
    let pp = pp
        .with_x(pp.x + TILE_SIZE.0 / 2.)
        .with_y(pp.y + TILE_SIZE.1 / 2.);
    pp
}
#[macroquad::main("Isometric Engine")]
async fn main() {
    // load textures into GPU
    // items here are named as _name bec they are moved to gamestate pls dont refrence them
    let mut _tiles: Vec<Texture2D> = Vec::new();
    _tiles.push(load_texture("empty.png").await.unwrap()); // this should not be rendered
    _tiles.push(load_texture("tile_select.png").await.unwrap());
    _tiles.push(load_texture("tile_frame.png").await.unwrap());
    _tiles.push(load_texture("tile_grass.png").await.unwrap());
    _tiles.push(load_texture("tile.png").await.unwrap());
    _tiles.push(load_texture("tile_d.png").await.unwrap());
    _tiles.push(load_texture("tile_machine.png").await.unwrap());
    let mut _player_textures = HashMap::new();
    _player_textures.insert(
        PlayerOrient::_225,
        load_texture("resources/player/+x.png").await.unwrap(),
    );
    _player_textures.insert(
        PlayerOrient::_315,
        load_texture("resources/player/+y.png").await.unwrap(),
    );
    _player_textures.insert(
        PlayerOrient::_45,
        load_texture("resources/player/-x.png").await.unwrap(),
    );
    _player_textures.insert(
        PlayerOrient::_135,
        load_texture("resources/player/-y.png").await.unwrap(),
    );
    _player_textures.insert(
        PlayerOrient::_270,
        load_texture("resources/player/+x+y.png").await.unwrap(),
    );
    _player_textures.insert(
        PlayerOrient::_90,
        load_texture("resources/player/-x-y.png").await.unwrap(),
    );
    _player_textures.insert(
        PlayerOrient::_180,
        load_texture("resources/player/+x-y.png").await.unwrap(),
    );
    _player_textures.insert(
        PlayerOrient::_0,
        load_texture("resources/player/-x+y.png").await.unwrap(),
    );
    for ele in &_tiles {
        ele.set_filter(FilterMode::Nearest);
    }
    let _player_texture = load_texture("creeper.png").await.unwrap();
    _player_texture.set_filter(FilterMode::Nearest);
    build_textures_atlas();
    // create Player
    let _player: Rc<RefCell<Player>> = Rc::new(RefCell::new(objects::Player::new(
        vec3(0., 0., 1.),
        Vec3::ZERO,
    )));
    // gen world
    let mut _world = Box::new(world::World::new());
    for i in 0..50 {
        for j in 0..50 {
            //ground
            _world.set_block(i, j, 0, 3);
            // hill
            if (5..=8).contains(&i) && (3..=6).contains(&j) {
                _world.set_block(i, j, 2, 3);
                _world.set_block(i, j, 4, 3);
            }
            if i > 0 && i < 3 && j > 0 && j < 3 {
                _world.set_block(i, j, 4, 3);
            }
        }
    }
    _world.set_block(10, 10, 1, 6);
    _world.set_block(11, 10, 1, 6);
    _world.set_block(12, 10, 1, 6);
    _world.set_block(10, 10, 3 + 2, 6);
    _world.set_block(11, 10, 3 + 2, 6);
    _world.set_block(12, 10, 3 + 2, 6);
    let game = Game {
        block_textures: _tiles,
        player_texture: _player_texture,
        player_object: _player,
        world: _world,
        player_textures: _player_textures,
    };

    let mut camera = Camera2D::from_display_rect(Rect {
        x: -500.,
        y: -500.,
        w: 1000.,
        h: 1000.,
    });
    camera.zoom = vec2(camera.zoom.x, -camera.zoom.y);
    let lower_limit = camera.zoom / 3.;
    let upper_limit = camera.zoom * 3.;

    // render layers
    // each z is a layers
    // tile layer
    // decoration layer (like flowers, rocks, ...)
    // player layer (always renders on top)
    // stuff like roofs that have higher Z than player will not render temporary when players under (so it dosent make the player look like its on top of it rather)

    let mut draw_queue: Vec<Rc<RefCell<dyn ISOGraphics>>> = Vec::with_capacity(1000);
    draw_queue.push(game.player_object.clone());
    // unload blocks from storage into render queue
    // todo: Later do something with dynamic loading where we only load a portion of visible map
    for ele in game.world.blocks() {
        draw_queue.push(Rc::new(RefCell::new(objects::Block::new(ele.0, ele.1))));
    }
    let mut curser_pos_iso = vec2(0., 0.);
    loop {
        draw_queue.sort_by(|a, b| {
            let a = a.as_ref().borrow();
            let b = b.as_ref().borrow();
            (a.pos().y + a.pos().x + a.pos().z)
                .partial_cmp(&(b.pos().y + b.pos().x + b.pos().z))
                .unwrap()
        });

        set_camera(&camera);
        if mouse_wheel().1.abs() > 0. {
            camera.zoom += mouse_wheel().1 * get_frame_time() * 240f32.recip();
            camera.zoom = camera.zoom.clamp(lower_limit, upper_limit);
        }
        // let p_on_scr = camera.world_to_screen(from_iso(
        //     space_to_iso(game.player_object.as_ref().borrow().pos()),
        //     TILE_SIZE,
        // ));
        // camera.offset = p_on_scr;

        clear_background(BLACK);
        draw_isometric_axis(vec2(0., 0.), 10., TILE_SIZE);
        let camera_screen_world =
            camera.screen_to_world(vec2(mouse_position().0, mouse_position().1));
        draw_rectangle_lines(
            camera_screen_world.x - 50.,
            camera_screen_world.y - 50.,
            100.,
            100.,
            5.,
            WHITE,
        );
        let csw_in_isometric = from_iso(camera_screen_world, TILE_SIZE);
        let grid_pos = Vec2 {
            x: csw_in_isometric.x - 4.,
            y: csw_in_isometric.y - 4.,
        };
        draw_isometric_grid(grid_pos, 10., TILE_SIZE);

        // update players physics
        let player_pos = game.player_object.as_ref().borrow().pos();
        let direction2d = -(player_pos.xy() - curser_pos_iso).normalize();
        let direction = vec3(
            direction2d.x,
            direction2d.y,
            game.player_object.as_ref().borrow().vel().z,
        );
        if is_key_down(KeyCode::W) {
            if !direction.is_nan() && direction.length() > 0.5 {
                game.player_object.borrow_mut().set_vel(direction * 2.);
                // player.pos += direction * player_speed * get_frame_time();
            }
        } else {
            let z = game.player_object.as_ref().borrow().vel().z;
            game.player_object.borrow_mut().set_vel(vec3(0., 0., z));
        }
        // todo jumping
        // if is_key_down(KeyCode::Space) && !player.is_jumping && (player.vel.x < 0.1 || player.vel.y < 1.0) {
        //     debug!("Jumped");
        //     player.vel += vec3(0., 0., 15.);
        //     player.is_jumping = true;
        // }
        // if player.is_jumping {
        //     debug!("Jumping");
        //     player.vel -= vec3(0., 0., 2.);
        //     if player.pos.z < 1. {
        //         debug!("Hit ground");
        //         player.vel.z = 0.;
        //         player.pos.z = 1.;
        //         player.is_jumping = false;
        //     }
        // }
        // update physics
        let vel = game.player_object.as_ref().borrow().vel();
        let pos = game.player_object.as_ref().borrow().pos();
        game.player_object
            .borrow_mut()
            .set_pos(pos + vel * get_frame_time());
        // player.pos += player.vel * get_frame_time();

        // render
        for el in draw_queue.iter() {
            let renderable = el.as_ref().borrow();
            renderable.render(&game);
        }
        curser_pos_iso = vec2(csw_in_isometric.x.floor(), csw_in_isometric.y.ceil());
        // cursor
        // draw_tile(curser_pos_iso.x, curser_pos_iso.y, tile_size, &tiles[1]);
        let h_pos = vec2(curser_pos_iso.x, curser_pos_iso.y);
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

        push_camera_state();
        set_default_camera();
        let v = in_2d(player_pos);
        camera.target = v;
        let v = camera.world_to_screen(v);
        let m: Vec2 = mouse_position().into();
        // let a = v.angle_between(v - m).to_degrees() + 180.;
        let a = (m - v).to_angle().to_degrees() - 180.;
        let mut a = a + 180.;
        if a > 360. {
            a = a - 360.;
        }
        a = -a;
        if a.is_sign_negative() {
            a = a + 360.;
        }
        game.player_object.as_ref().borrow_mut().update_orient(a);
        draw_text(
            format!("a: {}, v->m: {}->{}", a, v, m).as_str(),
            v.x,
            v.y,
            18.,
            WHITE,
        );
        draw_line(v.x, v.y, m.x, m.y, 2., GREEN);
        // draw_circle(v.x, v.y, 10., RED);
        draw_text(
            format!(
                "{}",
                vec2(csw_in_isometric.x.floor(), csw_in_isometric.y.ceil())
            )
            .as_str(),
            mouse_position().0,
            mouse_position().1,
            18.,
            YELLOW,
        );
        // draw_text(format!("{b}").as_str(), 40., 30., 14., WHITE);
        pop_camera_state();
        root_ui().group(hash!(), vec2(200., 400.), |ui| {
            ui.button(
                None,
                format!("Player: {}", game.player_object.as_ref().borrow().pos()).as_str(),
            );
            ui.button(None, format!("FPS: {}", get_fps()).as_str());
            ui.button(
                None,
                format!(
                    "Player Orientation: {:?}",
                    game.player_object.as_ref().borrow().orient
                )
                .as_str(),
            );
            // if ui.button(None, "Sort Map") {
            //     draw_queue.sort_by(|(a, _), (b, _)| a.z.partial_cmp(&b.z).unwrap());
            // }
            // for (i, (e, id)) in draw_queue.clone().iter().enumerate() {
            //     if ui.button(None, format!("{i}: {e}",).as_str()) {
            //         draw_queue.remove(i);
            //     }
            // }
        });
        if is_mouse_button_down(MouseButton::Right) {
            root_ui().window(hash!(), mouse_position().into(), vec2(100., 200.), |ui| {
                ui.button(None, "Select Block:");
                for (id, t) in game.block_textures.iter().enumerate() {
                    ui.button(None, format!("BlockID: {}", id));
                    ui.canvas().image(Rect::new(0., 0., 32., 32.), t);
                }
            });
        }
        next_frame().await;
    }
}
