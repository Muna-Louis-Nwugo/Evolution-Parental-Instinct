use super::blip::Blip;

pub fn move_blips(blips: Vec<&mut Blip>, x: i32, y: i32) {
    for blip in blips {
        blip.update_position(x, y);
    }
}

fn check_collision(blips: &Vec<Blip>) -> Option<Vec<&Blip, &Blip>> {
    let mut collision_detected: bool = false;
    let mut collision_pairs: Vec<(&Blip, &Blip)> = Vec::new();
    let blips_len: usize = blips.len();

    for i in 0..blips_len {
        let current_blip: &Blip = &blips[i];
        let current_blip_pos: (i32, i32) = *current_blip.get_position();

        for i in (i + 1)..blips_len {
            let check_blip: &Blip = &blips[i];
            let check_blip_pos: (i32, i32) = *check_blip.get_position();

            if check_blip_pos == current_blip_pos {
                collision_detected = true;
                collision_pairs.push((current_blip, check_blip));
            }
        }
    }

    if !collision_detected {
        None
    } else {
        Some(Box::new(collision_pairs))
    }
}
