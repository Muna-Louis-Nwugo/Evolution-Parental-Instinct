use super::blip::Blip;
use super::mover;

pub struct Partition {
    id: i32,
    owned_blips: Vec<Blip>,
}

impl Partition {
    pub fn new(id: i32) -> Partition {
        Partition {
            id: id,
            owned_blips: Vec::new(),
        }
    }

    pub fn add_blips(&mut self, blips: Vec<Blip>) {
        self.owned_blips = blips;
    }
}

/*
 * This module contains the world of the simulator. It acts as the system's central driver.
 *
 * Fields:
 * num_starting_blips - number of starting blips
 * width - width of canvas
 * height - height of canvas
 *
 * Methods:
 * step: proceed to the next step of the simulation
 */

pub struct World {
    num_starting_blips: i32,
    blips: Vec<Blip>,
    width: i32,
    height: i32,
    partitions: [(i32, i32); 9],
}

impl World {
    pub fn new(num_starting_blips: i32, width: i32, height: i32) -> World {
        World {
            num_starting_blips: num_starting_blips,
            blips: Self::generate_blips(num_starting_blips),
            width: width,
            height: height,
            partitions: Self::create_partition_grid(),
        }
    }

    fn create_partition_grid() -> [(i32, i32); 9] {
        /*Creates partitions. 1 is in upper right corner; 2, below that; 4 on the left of one,
         * etc...*/
        let mut coordinates: [(i32, i32); 9] = [(0, 0); 9];
        let mut array_position: usize = 0;
        let num_partitions: i32 = 3;
        for i in 0..3 {
            let x: i32 = num_partitions.pow(i);

            for i in 0..3 {
                let y: i32 = num_partitions.pow(i);
                coordinates[array_position] = (x, y);
                array_position += 1;
            }
        }

        coordinates
    }
    // TO BE IMPLEMENTED
    pub fn step() {
        mover::move_blips(self.blips, 0, 1);
    }

    fn generate_blips(count: i32) -> Vec<Blip> {
        let mut blips: Vec<Blip> = Vec::new();

        for _i in 1..=count {
            let blip = Blip::new((0, 0));
            blips.push(blip)
        }

        blips
    }
}
