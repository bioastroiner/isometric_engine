use macroquad::math::{vec3, Vec3};

const WIDTH: usize = 256;
const HEIGHT: usize = 64;
const AREA: usize = WIDTH * WIDTH;
const VOL: usize = AREA * HEIGHT;
/// world only stores tiles as they can be only one tile per block
pub struct World {
    tile_storage: [[[u8; WIDTH]; WIDTH]; HEIGHT],
    // entity_storage
}
impl World {
    pub fn new() -> Self {
        Self {
            tile_storage: [[[0; WIDTH]; WIDTH]; HEIGHT],
        }
    }
    pub fn set_block(&mut self, x: usize, y: usize, z: usize, b: u8) {
        self.tile_storage[z][y][x] = b;
    }
    pub fn get_block(&self, x: usize, y: usize, z: usize) -> u8 {
        self.tile_storage[z][y][x]
    }
    pub fn get_block_f(&self, pos: Vec3) -> u8 {
	let mut pos = pos;
	if pos.x.is_sign_negative() {
	    pos.x = - pos.x;
	}
	if pos.y.is_sign_negative() {
	    pos.y = - pos.y;
	}
	if pos.z.is_sign_negative() {
	    pos.z = - pos.z;
	}
        self.tile_storage[pos.z.floor() as usize][pos.y.floor() as usize][pos.x.floor() as usize]
    }
    pub fn blocks(&self) -> Vec<(Vec3, u8)> {
        let mut dest: Vec<(Vec3, u8)> = Vec::new();
        for (i, e) in self.tile_storage.iter().enumerate() {
            let z = i;
            for (i, e) in e.iter().enumerate() {
                let y = i;
                for (i, e) in e.iter().enumerate() {
                    let x = i;
                    if *e != 0 {
                        dest.push((vec3(x as f32, y as f32, z as f32), *e));
                    }
                }
            }
        }
        dest
    }
}
