use super::blip::Blip;

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
}

impl World {
    pub fn new(num_starting_blips: i32, width: i32, height: i32) -> World {
        World {
            num_starting_blips: num_starting_blips,
            blips: Self::generate_blips(num_starting_blips),
            width: width,
            height: height,
        }
    }

    // TO BE IMPLEMENTED
    pub fn step() {}

    fn generate_blips(count: i32) -> Vec<Blip> {
        let mut blips: Vec<Blip> = Vec::new();

        for i in 1..=count {
            let blip = Blip::new((0, 0));
            blips.push(blip)
        }

        blips
    }
}
