use std::str::FromStr;

use jsonpath_rust::{JsonPathFinder, JsonPathInst, JsonPathValue, JsonPtr};
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
        Ok(Self { value: parse_py_object(obj)? })
    }

    fn find_with_paths(self_: PyRef<'_, Self>, query: String) -> PyResult<Vec<JsonPathResult>> {
        let value = &self_.value;
        let query = parse_query(&query)?;

        let finder = JsonPathFinder::new(value.clone().into(), query.into());
        let slice = finder.find_slice();
        Python::with_gil(|py| {
            map_json_path_values(py, slice)
        })
    }

    fn find(self_: PyRef<'_, Self>, query: String) -> PyResult<Vec<JsonPathResult>> {
        let value = &self_.value;
        let query = parse_query(&query)?;
        let slice = query.find_slice(value);

        Python::with_gil(|py| {
            map_json_ptrs(py, slice)
        })
    }
}


#[pyfunction]
fn jsonpath(query: String, obj: PyObject) -> PyResult<Vec<JsonPathResult>> {
    let query = parse_query(&query)?;
    let value = parse_py_object(obj)?;
    let finder = JsonPathFinder::new(value.into(), query.into());
    let slice = finder.find_slice();
    Python::with_gil(|py| {
        map_json_path_values(py, slice)
    })
}


fn map_json_ptrs(py: Python, slice: Vec<JsonPtr<Value>>) -> PyResult<Vec<JsonPathResult>> {
    let mut result = vec![];

    for node in slice {
        match node {
            JsonPtr::Slice(data) => {
                result.push(JsonPathResult {
                    data: Some(pythonize(py, data)?),
                    path: None,
                    is_new_value: false,
                });
            }
            JsonPtr::NewValue(data) => {
                result.push(JsonPathResult {
                    data: Some(pythonize(py, &data)?),
                    path: None,
                    is_new_value: true,
                });
            }
        }
    }

    Ok(result)
}


fn map_json_path_values(py: Python, slice: Vec<JsonPathValue<Value>>) -> PyResult<Vec<JsonPathResult>> {
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


fn parse_query(query: &str) -> PyResult<JsonPathInst> {
    match JsonPathInst::from_str(query) {
        Ok(inst) => Ok(inst),
        Err(err) => {
            Err(PyValueError::new_err(err))
        }
    }
}

fn parse_py_object(obj: PyObject) -> PyResult<Value> {
    Python::with_gil(|py| {
        let obj: &PyAny = obj.downcast(py)?;
        Ok(depythonize(obj)?)
    })
}

#[pymodule]
fn jsonpath_rust_bindings(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Finder>()?;
    m.add_function(wrap_pyfunction!(jsonpath, m)?)?;
    Ok(())
}
