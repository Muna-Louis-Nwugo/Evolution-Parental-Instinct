#[pyo3::pymodule]
mod blip_mover {
    use pyo3::{prelude::*, types::PyList};

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
        let blips_list: Bound<'py, Pylist> = blips.clone();

        check_collisions(blips_list, width, height);
        Ok(blips.clone())
    }

    fn check_collisions<'py>(
        blips: Bound<'py, PyList>,
        width: i32,
        height: i32,
    ) -> (bool, Vec<(PyAny, PyAny)>) {
        /*Checks if any blips are currently colliding.
         * If blips are colliding, this returns true, and then a list of tuples of all colliding
         * blips*/
        let collision_detected: bool = false;
        let collision_pairs: Vec<PyAny, PyAny>;

        Python::attach(|py| {
            let len: PyAny = PyModule::from_code(py, 
                c"def length(ls: list) :
                    return len(ls)", 
                    c"length.py", 
                    c"",
            )?
            .gettattr("length")?
            .into();

        blips_len: i32 = len.call1(py, (blips));
        for i in 0..blips_len {
            current_blip: PyAny = blips.get_item(i);
            for j: i32 in 0..blips_len {
                if j == i {
                    continue;
                }


            }
        }

        })
        return (true, blips);
    }
}
