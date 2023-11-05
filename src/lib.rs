use jsonpath_rust::JsonPathValue;
use jsonpath_rust::parser::model::JsonPath;
use jsonpath_rust::path::json_path_instance;
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

#[pymethods]
impl JsonPathResult {
    fn __repr__(slf: PyRef<'_, Self>) -> PyResult<String> {
        let data_repr = Python::with_gil(|py| {
            match &slf.data {
                Some(data) => format!("{:?}", data.as_ref(py)),
                None => "None".to_string(),
            }
        });
        Ok(format!(
            "JsonPathResult(data={}, path={:?}, is_new_value={})",
            data_repr,
            slf.path,
            slf.is_new_value
        ))
    }
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


    fn find(self_: PyRef<'_, Self>, query: String) -> PyResult<Vec<JsonPathResult>> {
        let value = &self_.value;
        let query = parse_query(&query)?;

        let slice = json_path_instance(&query, value)
            .find(JsonPathValue::from_root(value))
            .into_iter()
            .collect::<Vec<_>>();

        Python::with_gil(|py| {
            slice.into_iter().map(|v| map_json_path_value(py, v)).collect()
        })
    }
}


fn map_json_path_value(py: Python, jpv: JsonPathValue<Value>) -> PyResult<JsonPathResult> {
    Ok(match jpv {
        JsonPathValue::Slice(data, path) => {
            JsonPathResult {
                data: Some(pythonize(py, data)?),
                path: Some(path.to_string()),
                is_new_value: false,
            }
        }
        JsonPathValue::NewValue(data) => {
            JsonPathResult {
                data: Some(pythonize(py, &data)?),
                path: None,
                is_new_value: true,
            }
        }
        JsonPathValue::NoValue => {
            JsonPathResult {
                data: None,
                path: None,
                is_new_value: false,
            }
        }
    })
}


fn parse_query(query: &str) -> PyResult<JsonPath> {
    match JsonPath::try_from(query) {
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
    Ok(())
}

