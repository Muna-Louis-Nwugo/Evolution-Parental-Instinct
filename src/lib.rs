#[pyo3::pymodule]
mod blip_mover {
    use pyo3::{
        prelude::*,
        types::{PyList, PyTuple},
    };

    #[pyfunction]
    fn move_blips<'py>(blips: &Bound<'py, PyList>) -> PyResult<Bound<'py, PyList>> {
        for blip in blips {
            let change: (i32, i32) = (0, 1);

            blip.call_method1("update_pos", change)?;
        }

        Ok(blips.clone())
    }
}
