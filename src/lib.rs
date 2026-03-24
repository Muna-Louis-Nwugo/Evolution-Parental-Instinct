// begin transition to Rust-centric simulator
mod blip;
mod mover;
mod world;

#[pyo3::pymodule]
mod blip_mover {
    use pyo3::{PyResult, prelude::*, types::PyList};

    #[pyfunction]
    fn move_blips<'py>(
        blips: &Bound<'py, PyList>,
        width: i32,
        height: i32,
    ) -> PyResult<Bound<'py, PyList>> {
        /*Moves a group of blips*/
        // TODO: Change below for loop to a map for efficiency
        for blip in blips {
            let change: (i32, i32) = (0, 1);
            blip.call_method1("update_pos", change)?;
        }
        let blips_list: Bound<'py, PyList> = blips.clone();

        check_collisions(blips_list, width, height);
        Ok(blips.clone())
    }

    fn check_collisions<'py>(
        blips: Bound<'py, PyList>,
        width: i32,
        height: i32,
    ) -> PyResult<(bool, Vec<(Bound<'py, PyAny>, Bound<'py, PyAny>)>)> {
        /*Checks if any blips are currently colliding.
         * If blips are colliding, this returns true, and then a list of tuples of all colliding
         * blips*/
        let mut collision_detected: bool = false;
        let mut collision_pairs: Vec<(Bound<'py, PyAny>, Bound<'py, PyAny>)> = Vec::new();
        let blips_len = blips.len();

        for i in 0..blips_len {
            let current_blip = blips.get_item(i)?;
            let current_blip_pos = current_blip.call_method0("get_pos")?;

            for j in (i + 1)..blips_len {
                if j == i {
                    continue;
                }

                let check_blip = blips.get_item(j)?;
                let check_blip_pos = check_blip.call_method0("get_pos")?;

                if current_blip_pos.eq(check_blip_pos)? {
                    collision_detected = true;
                    collision_pairs.push((current_blip.clone(), check_blip));
                }
            }
        }

        Ok((collision_detected, collision_pairs))
    }
}
