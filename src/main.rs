pub mod math;
pub mod tile;

use math::*;
use tile::*;

use std::{collections::HashMap, ops::Range};

use macroquad::{
    prelude::*,
    shapes,
    ui::{hash, root_ui, widgets::Label},
};

mod components {
    struct Player {}
    impl Player {
        pub fn tick_controler() {}
    }
}

mod world {
    use macroquad::{
        logging::debug,
        math::{vec3, Vec3},
    };

    use crate::BlockType;

    const XY_SIZE: usize = 2000;
    const Z_SIZE: usize = 200;
    const AREA: usize = XY_SIZE * XY_SIZE;
    const VOL: usize = AREA * Z_SIZE;
    /// world only stores tiles as they can be only one tile per block
    pub struct World {
        tile_storage: [[[u8; XY_SIZE]; XY_SIZE]; Z_SIZE],
        // entity_storage
    }
    impl World {
        pub fn new() -> Self {
            Self {
                tile_storage: [[[0; XY_SIZE]; XY_SIZE]; Z_SIZE],
            }
        }
        pub fn set_block(&mut self, x: usize, y: usize, z: usize, b: u8) {
            self.tile_storage[z][y][x] = b;
        }
        pub fn get_block(&self, x: usize, y: usize, z: usize) -> u8 {
            self.tile_storage[z][y][x]
        }
        pub fn blocks_to_render_queue(&self, dest: &mut Vec<(Vec3, BlockType)>) {
            for (i, e) in self.tile_storage.iter().enumerate() {
                let z = i;
                for (i, e) in e.iter().enumerate() {
                    let y = i;
                    for (i, e) in e.iter().enumerate() {
                        let x = i;
                        if *e != 0 {
                            dest.push((vec3(x as f32, y as f32, z as f32), BlockType::Tile(*e)));
                        }
                    }
                }
            }
        }
    }
}

type Vel = Vec3;

#[derive(Clone, Copy, Debug)]
enum BlockType {
    Tile(u8),
    Entity(Vel),
}

#[macroquad::main("Isometric Engine")]
async fn main() {
    let mut camera = Camera2D::from_display_rect(Rect {
        x: -500.,
        y: -500.,
        w: 1000.,
        h: 1000.,
    });
    camera.zoom = vec2(camera.zoom.x, -camera.zoom.y);

    // render layers
    // each z is a layers
    // tile layer
    // decoration layer (like flowers, rocks, ...)
    // player layer (always renders on top)
    // stuff like roofs that have higher Z than player will not render temporary when players under (so it dosent make the player look like its on top of it rather)

    let mut tiles: Vec<Texture2D> = Vec::new();
    tiles.push(load_texture("empty.png").await.unwrap()); // this should not be rendered
    tiles.push(load_texture("tile_select.png").await.unwrap());
    tiles.push(load_texture("tile_frame.png").await.unwrap());
    tiles.push(load_texture("tile_grass.png").await.unwrap());
    tiles.push(load_texture("tile.png").await.unwrap());
    tiles.push(load_texture("tile_d.png").await.unwrap());
    tiles.push(load_texture("tile_machine.png").await.unwrap());
    build_textures_atlas();
    for ele in &tiles {
        ele.set_filter(FilterMode::Nearest);
    }
    let player_texture = load_texture("creeper.png").await.unwrap();
    player_texture.set_filter(FilterMode::Nearest);

    // world gen
    let mut world = Box::new(world::World::new());
    for i in 0..50 {
        for j in 0..50 {
            for k in 0..(j) % 50 {
                world.set_block(i, j, k, 3);
            }
            // world.set_block(i, j, 0, 3);
        }
    }

    let mut draw_queue: Vec<(Vec3, BlockType)> = Vec::with_capacity(1000);
    draw_queue.push(((vec3(0., 0., 1.)), BlockType::Entity(Vec3::ZERO)));
    world.blocks_to_render_queue(&mut draw_queue);
    // sorting a list of object in a 3d space
    //// 1. Determine if boxes overlap on screen
    ////      sort by z
    // 2. Determine which boxes in front
    // 3. Draw Boxes in correct order

    let player_speed = 2.0;
    let mut curser_pos_iso = vec2(0., 0.);
    let tile_size = (64.0, 64.0);

    loop {
        draw_queue
            .sort_by(|(a, _), (b, _)| (a.y + a.x + a.z).partial_cmp(&(b.y + b.x + b.z)).unwrap());
        // draw_queue
        //     .sort_by(|(a, _), (b, _)| (a.z).partial_cmp(&(a.z)).unwrap());

        set_camera(&camera);
        clear_background(BLACK);
        draw_isometric_axis(vec2(0., 0.), 10., tile_size);
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
        let csw_in_isometric = from_iso(camera_screen_world, tile_size);
        draw_isometric_grid(csw_in_isometric, 10., tile_size);
        for (b, i) in draw_queue.iter_mut() {
            match i {
                BlockType::Entity(vel) => {
                    let player_pos = b;
                    let direction = -(player_pos.xy() - curser_pos_iso).normalize();
                    if is_key_down(KeyCode::W) {
                        if !direction.is_nan() && direction.length() > 0.5 {
                            *player_pos += vec3(direction.x, direction.y, 0.)
                                * player_speed
                                * get_frame_time();
                        }
                    }
                }
                _ => (),
            }
        }
        for (b, i) in draw_queue.iter() {
            match i {
                BlockType::Tile(tile_id) => {
                    let p = space_to_iso(*b);
                    draw_tile_margin(p.x, p.y, tile_size, &tiles[(*tile_id) as usize], 0.);
                }
                BlockType::Entity(vel) => {
                    let p = space_to_iso(*b);
                    draw_tile(p.x, p.y, tile_size, &player_texture)
                }
                _ => todo!(),
            }
        }
        curser_pos_iso = vec2(csw_in_isometric.x.floor(), csw_in_isometric.y.ceil());
        // cursor
        // draw_tile(curser_pos_iso.x, curser_pos_iso.y, tile_size, &tiles[1]);
        let h_pos = vec2(curser_pos_iso.x, curser_pos_iso.y);
        draw_hexagon(
            tile_matrix(tile_size).mul_vec2(h_pos).x + tile_size.0 / 2.,
            tile_matrix(tile_size).mul_vec2(h_pos).y,
            tile_size.0 / 2.,
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
            if ui.button(None, "Sort Map") {
                draw_queue.sort_by(|(a, _), (b, _)| a.z.partial_cmp(&b.z).unwrap());
            }
            for (i, (e, id)) in draw_queue.clone().iter().enumerate() {
                if ui.button(None, format!("{i}: {e}",).as_str()) {
                    draw_queue.remove(i);
                }
            }
        });
        next_frame().await;
    }
}
