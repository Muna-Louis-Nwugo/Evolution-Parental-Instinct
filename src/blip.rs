/*Blip Data Structure*/

/* This module contains all data related to the "blip" entity, as well as relevant modifiers.
 *
 * States:
 * Position - Blip's current position on canvas
 * Velocity - Blip's current velocity
 * Acceleration - Blips current acceleration
 *
 * Getters:
 * get_position()
 * get_velocity()
 * get_acceleration()
 *
 */
pub struct Blip {
    position: (i32, i32),
    velocity: (i32, i32),
    acceleration: (i32, i32),
    max_acceleration: i32,
    max_velocity: i32,
}

impl Blip {
    // constructor
    pub fn new(position: (i32, i32)) -> Self {
        Blip {
            position: position,
            velocity: (0, 0),
            acceleration: (0, 0),
            max_acceleration: 1,
            max_velocity: 1,
        }
    }
    // getters
    pub fn get_position(&self) -> &(i32, i32) {
        /*gets blip's current position*/
        &self.position
    }

    pub fn get_velocity(&self) -> &(i32, i32) {
        /*gets blip's current veolcity*/
        &self.velocity
    }

    pub fn get_acceleration(&self) -> &(i32, i32) {
        /*gets blip's current acceleration*/
        &self.acceleration
    }
}
