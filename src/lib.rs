use std::str::FromStr;

use jsonpath_rust::{JsonPathFinder, JsonPathInst, JsonPathValue};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pythonize::{depythonize, pythonize};
use serde_json::Value;

#[pyclass(frozen)]
struct JsonPathResult {
    #[pyo3(get)]
    data: Option<PyObject>,

    #[pyo3(get)]
    path: Option<String>,

    #[pyo3(get)]
    is_new_value: bool,
}


#[pyclass(frozen)]
struct Finder {
    value: Value,
}

#[pymethods]
impl Finder {
    #[new]
    fn py_new(obj: PyObject) -> PyResult<Self> {
        let value: PyResult<Value> = Python::with_gil(|py| {
            let obj: &PyAny = obj.downcast(py)?;
            Ok(depythonize(obj)?)
        });

        Ok(Self { value: value? })
    }

    fn find(self_: PyRef<'_, Self>, query: String) -> PyResult<Vec<JsonPathResult>> {
        let value = &self_.value;
        let query = match JsonPathInst::from_str(&query) {
            Ok(inst) => inst,
            Err(err) => {
                return Err(PyValueError::new_err(err));
            }
        };

        // TODO: how to get rid of cloning?
        //  Also, how can we return python references back to python?
        let finder = JsonPathFinder::new(value.clone().into(), query.into());
        Python::with_gil(|py| {
            map_result(py, finder.find_slice())
        })
    }
}


#[pyfunction]
fn jsonpath(path: String, obj: PyObject) -> PyResult<Vec<JsonPathResult>> {
    let query = match JsonPathInst::from_str(&path) {
        Ok(inst) => inst,
        Err(err) => {
            return Err(PyValueError::new_err(err));
        }
    };
    Python::with_gil(|py| {
        let obj: &PyAny = obj.downcast(py)?;
        let value: Value = depythonize(obj)?;
        let finder = JsonPathFinder::new(value.into(), query.into());
        map_result(py, finder.find_slice())
    })
}

fn map_result(py: Python, slice: Vec<JsonPathValue<Value>>) -> PyResult<Vec<JsonPathResult>> {
    let mut result = vec![];

    for node in slice {
        match node {
            JsonPathValue::Slice(data, path) => {
                result.push(JsonPathResult {
                    data: Some(pythonize(py, data)?),
                    path: Some(path.to_string()),
                    is_new_value: false,
                });
            }
            JsonPathValue::NewValue(data) => {
                result.push(JsonPathResult {
                    data: Some(pythonize(py, &data)?),
                    path: None,
                    is_new_value: true,
                });
            }
            JsonPathValue::NoValue => {
                result.push(JsonPathResult {
                    data: None,
                    path: None,
                    is_new_value: false,
                })
            }
        }
    }

    Ok(result)
}


#[pymodule]
fn jsonpath_rust_bindings(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Finder>()?;
    m.add_function(wrap_pyfunction!(jsonpath, m)?)?;
    Ok(())
}
