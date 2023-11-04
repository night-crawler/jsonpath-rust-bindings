use std::str::FromStr;

use jsonpath_rust::{JsonPathFinder, JsonPathInst, JsonPathValue};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pythonize::{depythonize, pythonize};
use serde_json::Value;

#[pyfunction]
fn jsonpath(path: String, obj: PyObject) -> PyResult<Vec<PyObject>> {
    let query = match JsonPathInst::from_str(&path) {
        Ok(inst) => inst,
        Err(err) => {
            return Err(PyValueError::new_err(err))
        }
    };
    Python::with_gil(|py| {
        let obj: &PyAny = obj.downcast(py)?;
        let value: Value = depythonize(obj)?;
        let finder = JsonPathFinder::new(value.into(), query.into());

        let slice = finder.find_slice();
        let mut result = vec![];
        for node in slice {
            match node {
                JsonPathValue::Slice(data, path) => {
                    let tuple = (data, path.to_string());
                    let serialized = pythonize(py, &tuple)?;
                    result.push(serialized);
                }
                JsonPathValue::NewValue(data) => {
                    let serialized = pythonize(py, &data)?;
                    result.push(serialized);
                }
                JsonPathValue::NoValue => {}
            }
        }
        Ok(result)
    })
}


#[pymodule]
fn jsonpath_rust_bindings(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(jsonpath, m)?)?;
    Ok(())
}
