use crate::objects::Inventory;

// machiens are stored seperately from the blocks they are in
// they are agnostics with "block entities" for now we seperate them
//
struct Machine {
    process_time: usize,
    input_sides: Option<Vec<Sides>>,
    output_side: Option<Sides>,
}

enum Sides {
    Up,
    Down,
    North,
    South,
    East,
    West,
}

impl Inventory for Machine {
    fn get_inputs() {
        todo!()
    }

    fn get_outputs() {
        todo!()
    }
}
