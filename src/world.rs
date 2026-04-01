use std::sync::mpsc;
use std::thread::{self, JoinHandle};

use super::blip::Blip;
use super::mover;

pub struct Partition {
    id: i32,
    owned_blips: Vec<Blip>,
    x_boundary: i32,
    y_boundary: i32,
}

impl Partition {
    pub fn new(id: i32, x_boundary: i32, y_boundary: i32) -> Partition {
        Partition {
            id: id,
            owned_blips: Vec::new(),
            x_boundary: x_boundary,
            y_boundary: y_boundary,
        }
    }

    pub fn add_blips(&mut self, blips: Vec<Blip>) {
        self.owned_blips = blips;
    }

    pub fn execute(self) -> Self {
        self
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
    threads: Vec<thread::JoinHandle<()>>,
}

impl World {
    pub fn new(num_starting_blips: i32, width: i32, height: i32) -> World {
        let partitions = Self::create_partition_grid();

        World {
            num_starting_blips: num_starting_blips,
            blips: Self::generate_blips(num_starting_blips),
            width: width,
            height: height,
            threads: Self::spawn_threads(partitions),
        }
    }

    fn spawn_threads(partitions: Vec<Partition>) -> Vec<thread::JoinHandle<()>> {
        let mut threads = Vec::new();

        for partition in partitions {
            let thread = thread::spawn(move || {
                let me = partition;
                me.execute();
            });

            threads.push(thread);
        }

        threads
    }

    fn create_partition_grid() -> Vec<Partition> {
        /*Creates partitions. 1 is in upper right corner; 2, below that; 4 on the left of one,
         * etc...*/
        let mut coordinates: [(i32, i32); 9] = [(0, 0); 9];
        let mut partitions = Vec::new();
        let mut array_position: usize = 0;
        let num_partitions: i32 = 3;
        let partition_id = 0;

        for i in 0..3 {
            let x: i32 = num_partitions.pow(i);

            for i in 0..3 {
                let y: i32 = num_partitions.pow(i);
                coordinates[array_position] = (x, y);

                let partition = Partition::new(partition_id, x, y);
                array_position += 1;
            }
        }

        partitions
    }
    // TO BE IMPLEMENTED
    pub fn step(&mut self) {
        mover::move_blips(&mut self.blips, 0, 1);
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
