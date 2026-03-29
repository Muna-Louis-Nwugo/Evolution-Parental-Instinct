use super::blip::Blip;

pub fn move_blips(blips: &mut Vec<Blip>, x: i32, y: i32) {
    for blip in blips.iter_mut() {
        blip.update_position(x, y);
    }

    let collisions = check_collision(blips);

    match collisions {
        None => return,

        Some(collisions) => {
            for blip_pair in collisions {
                handle_collision(blip_pair, blips);
            }
        }
    }
}

// STUPID STUPID SOLUTION, JUST USING IT TO MOVE QUICKLY
fn handle_collision(position: (usize, usize), blips: &mut Vec<Blip>) {
    blips[position.0].update_position(0, 2);
    blips[position.1].update_position(0, 2);
}

fn check_collision(blips: &mut Vec<Blip>) -> Option<Vec<(usize, usize)>> {
    let mut collision_detected = false;
    let mut collision_pairs = Vec::new();
    let blips_len = blips.len();

    for i in 0..blips_len {
        let current_blip = &blips[i];
        let current_blip_pos = current_blip.get_position();

        for j in (i + 1)..blips_len {
            let check_blip = &blips[j];
            let check_blip_pos = check_blip.get_position();

            if current_blip_pos == check_blip_pos {
                collision_detected = true;
                collision_pairs.push((i, j));
            }
        }
    }
    if !collision_detected {
        None
    } else {
        Some(collision_pairs)
    }
}
// fn check_collision<'a>(blips: &'a mut Vec<Blip>) -> Option<Vec<(&'a mut Blip, &'a mut Blip)>> {
