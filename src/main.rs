pub mod renderer;

use std::{collections::HashMap, ops::Range};

use macroquad::{
    prelude::*,
    ui::{hash, root_ui, widgets::Label},
};

#[inline]
fn transform_tile(x: f32, y: f32, tile_size: (f32, f32)) -> (f32, f32) {
    let mat = tile_matrix(tile_size);
    (mat.mul_vec2(vec2(x, y)).x, mat.mul_vec2(vec2(x, y)).y)
}

#[inline]
fn tile_matrix(tile_size: (f32, f32)) -> Mat2 {
    let (w, h) = tile_size;
    Mat2::from_cols_array(&[0.5 * w, -0.5 * w, 0.25 * h, 0.25 * h]).transpose()
}

#[inline]
fn draw_tile(x: f32, y: f32, tile_size: (f32, f32), texture: &Texture2D) {
    let (x, y) = transform_tile(x - 1., y - 1., tile_size);
    draw_texture_ex(
        texture,
        x,
        y,
        WHITE,
        DrawTextureParams {
            // flip_y: true,
            ..Default::default()
        },
    );
}

#[inline]
fn draw_isometric_axis(at_isometric: Vec2, length: f32, tile_size: (f32, f32)) {
    /* X Axis */
    let (x1, y1) = transform_tile(0. + at_isometric.x, 0. + at_isometric.y, tile_size);
    let (x2, y2) = transform_tile(length + at_isometric.x, 0. + at_isometric.y, tile_size);
    draw_line(x1, y1, x2, y2, 2.0, GREEN);
    /* Y Axis */
    let (x1, y1) = transform_tile(0. + at_isometric.x, 0. + at_isometric.y, tile_size);
    let (x2, y2) = transform_tile(0. + at_isometric.x, length + at_isometric.y, tile_size);
    draw_line(x1, y1, x2, y2, 2.0, RED);
    /* fake Z Axis */
    let (x1, y1) = transform_tile(0. + at_isometric.x, 0. + at_isometric.y, tile_size);
    let (x2, y2) = transform_tile(
        -length + at_isometric.x,
        -length + at_isometric.y,
        tile_size,
    );
    draw_line(x1, y1, x2, y2, 2.0, BLUE);
}

#[inline]
// rather expensive
fn draw_isometric_grid(at_isometric: Vec2, length: f32, tile_size: (f32, f32)) {
    let v = at_isometric.floor();
    for i in 1..(length as i32) {
        let (x1, y1) = transform_tile(v.x, v.y + i as f32, tile_size);
        let (x2, y2) = transform_tile(v.x + length.floor(), v.y + i as f32, tile_size);
        draw_line(x1, y1, x2, y2, 3., GREEN);
        let (x1, y1) = transform_tile(v.x + i as f32, v.y, tile_size);
        let (x2, y2) = transform_tile(v.x + i as f32, v.y + length.floor(), tile_size);
        draw_line(x1, y1, x2, y2, 3., RED);
    }
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
    tiles.push(load_texture("tile_select.png").await.unwrap());
    tiles.push(load_texture("tile_frame.png").await.unwrap());
    tiles.push(load_texture("tile_grass.png").await.unwrap());
    tiles.push(load_texture("tile.png").await.unwrap());
    tiles.push(load_texture("tile_d.png").await.unwrap());
    for ele in &tiles {
        ele.set_filter(FilterMode::Nearest);
    }
    let player_texture = load_texture("creeper.png").await.unwrap();
    player_texture.set_filter(FilterMode::Nearest);

    let (mut px, mut py) = (0.0f32, 0.0f32);

    let x_size = 100;
    let y_size = 100;
    let z_size = 20;
    let mut tile_map: Vec<(Vec3, u32)> = Vec::with_capacity(1000);
    tile_map.push((vec3(0., 0., 1.0), 0));
    tile_map.push((vec3(5., 5., 3.), 4));
    for i in 0..10 {
        for j in 0..10 {
            tile_map.push((vec3(i as f32, j as f32, 0.), 2));
        }
    }

    let player_speed = 2.0;
    loop {
        if is_key_down(KeyCode::S) {
            py -= get_frame_time() * player_speed;
        }
        if is_key_down(KeyCode::W) {
            py += get_frame_time() * player_speed;
        }
        if is_key_down(KeyCode::A) {
            px -= get_frame_time() * player_speed;
        }
        if is_key_down(KeyCode::D) {
            px += get_frame_time() * player_speed;
        }

        set_camera(&camera);
        clear_background(BLACK);
        draw_isometric_axis(vec2(0., 0.), 10., (32., 32.));
        let c = camera.screen_to_world(vec2(mouse_position().0, mouse_position().1));
        draw_rectangle_lines(c.x - 50., c.y - 50., 100., 100., 5., WHITE);
        let c = camera.screen_to_world(vec2(mouse_position().0, mouse_position().1));
        let a = tile_matrix((32., 32.)).inverse().mul_vec2(c);
        draw_isometric_grid(a, 10., (32., 32.));
        let b = tile_matrix((32., 32.)).mul_vec2(c);
        let (x, y) = transform_tile(a.x as f32, a.y as f32, (32., 32.));
        draw_text(format!("{}", vec2(x, y)).as_str(), x, y, 18., YELLOW);
        for (b, i) in tile_map.iter() {
            draw_tile(b.x - b.z, b.y - b.z, (32., 32.), &tiles[(*i) as usize]);
        }
        // player
        draw_tile(px, py, (32., 32.), &player_texture);
        // cursor
        draw_tile(a.x.floor(), a.y.ceil(), (32., 32.), &tiles[0]);

        push_camera_state();
        set_default_camera();
        // draw_text(format!("{b}").as_str(), 40., 30., 14., WHITE);
        pop_camera_state();
        root_ui().group(hash!(), vec2(200., 400.), |ui| {
            if ui.button(None, "Sort Map") {
                tile_map.sort_by(|(a, _), (b, _)| a.z.partial_cmp(&b.z).unwrap());
            }
            for (i, (e, id)) in tile_map.clone().iter().enumerate() {
                if ui.button(None, format!("{i}: {e}:{id}",).as_str()) {
                    tile_map.remove(i);
                }
            }
        });
        next_frame().await;
    }
}
