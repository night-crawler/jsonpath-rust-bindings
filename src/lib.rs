use jsonpath_rust::parser::model::JpQuery;
use jsonpath_rust::parser::parse_json_path;
use jsonpath_rust::query::{js_path_process, QueryRef};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pythonize::{depythonize, pythonize};
use serde_json::Value;

#[cfg(any(
    not(target_os = "linux"),
    all(target_os = "linux", target_env = "musl")
))]
use mimalloc::MiMalloc;
#[cfg(any(
    not(target_os = "linux"),
    all(target_os = "linux", target_env = "musl")
))]
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[cfg(all(target_os = "linux", not(target_env = "musl")))]
use std::alloc::System;
#[cfg(all(target_os = "linux", not(target_env = "musl")))]
#[global_allocator]
static GLOBAL: System = System;

// JSONPath query result containing found data and path
#[pyclass(frozen)]
struct JsonPathResult {
    // Found data value
    #[pyo3(get)]
    data: Option<Py<PyAny>>,

    // Path of the data in JSON
    #[pyo3(get)]
    path: Option<String>,
}

#[pymethods]
impl JsonPathResult {
    // Returns string representation of JsonPathResult
    fn __repr__(slf: PyRef<'_, Self>) -> PyResult<String> {
        repr_json_path_result(slf)
    }
}

// JSONPath finder for executing queries on JSON data
#[pyclass(frozen)]
struct Finder {
    value: Value,
}

#[pymethods]
impl Finder {
    #[new]
    fn py_new(obj: Py<PyAny>) -> PyResult<Self> {
        Ok(Self {
            value: parse_py_object(obj)?,
        })
    }

    // Execute JSONPath query, return list of results containing data and paths
    fn find(self_: PyRef<'_, Self>, query: String) -> PyResult<Vec<JsonPathResult>> {
        find_internal_path_value(&self_.value, &query, |_| true)
    }

    // Execute JSONPath query, return only found data values
    fn find_data(self_: PyRef<'_, Self>, query: String) -> PyResult<Vec<Py<PyAny>>> {
        find_internal_data(&self_.value, &query, |_| true)
    }

    // Execute JSONPath query, return only found absolute paths
    fn find_absolute_path(self_: PyRef<'_, Self>, query: String) -> PyResult<Vec<String>> {
        find_internal_path(&self_.value, &query, |_| true)
    }
}

// Execute JSONPath query and return processed results
fn execute_query<'a>(
    value: &'a Value,
    query: &str,
    predicate: impl Fn(&QueryRef<Value>) -> bool,
) -> PyResult<Vec<QueryRef<'a, Value>>> {
    let parsed_query = parse_query(query)?;
    let processed = js_path_process(&parsed_query, value)
        .map_err(|err| PyValueError::new_err(err.to_string()))?;

    Ok(processed.into_iter().filter(predicate).collect())
}

// Execute query and return JsonPathResult list
fn find_internal_path_value(
    value: &Value,
    query: &str,
    predicate: impl Fn(&QueryRef<Value>) -> bool,
) -> PyResult<Vec<JsonPathResult>> {
    let filtered = execute_query(value, query, predicate)?;

    Python::attach(|py| {
        filtered
            .into_iter()
            .map(|v| map_json_path_value(py, v))
            .collect()
    })
}

// Execute query and return data value list
fn find_internal_data(
    value: &Value,
    query: &str,
    predicate: impl Fn(&QueryRef<Value>) -> bool,
) -> PyResult<Vec<Py<PyAny>>> {
    let filtered = execute_query(value, query, predicate)?;

    Python::attach(|py| {
        filtered
            .into_iter()
            .map(|v| map_json_value(py, v))
            .collect()
    })
}

// Execute query and return path string list
fn find_internal_path(
    value: &Value,
    query: &str,
    predicate: impl Fn(&QueryRef<Value>) -> bool,
) -> PyResult<Vec<String>> {
    let filtered = execute_query(value, query, predicate)?;
    filtered.into_iter().map(|v| map_json_path(v)).collect()
}

// Map QueryRef<Value> to JsonPathResult
fn map_json_path_value(py: Python, jpv: QueryRef<Value>) -> PyResult<JsonPathResult> {
    let path = jpv.clone().path();
    let val = jpv.val();

    Ok(JsonPathResult {
        data: Some(pythonize(py, val)?.into_pyobject(py)?.unbind()),
        path: Some(path),
    })
}

// Extract path string from QueryRef<Value>
fn map_json_path(jpv: QueryRef<Value>) -> PyResult<String> {
    Ok(jpv.path())
}

// Convert value in QueryRef<Value> to Python object
fn map_json_value(py: Python, jpv: QueryRef<Value>) -> PyResult<Py<PyAny>> {
    let val = jpv.val();
    Ok(pythonize(py, val)?.into_pyobject(py)?.unbind())
}

// Parse JSONPath query string directly without caching
fn parse_query(query: &str) -> PyResult<JpQuery> {
    parse_json_path(query).map_err(|err| PyValueError::new_err(format!("{err:?}")))
}

// Convert Python object to serde_json::Value
fn parse_py_object(obj: Py<PyAny>) -> PyResult<Value> {
    Python::attach(|py| {
        let any = obj.bind(py);
        depythonize(any).map_err(|e| PyValueError::new_err(e.to_string()))
    })
}

// Generate string representation for JsonPathResult
fn repr_json_path_result(slf: PyRef<'_, JsonPathResult>) -> PyResult<String> {
    let data_repr = slf
        .data
        .as_ref()
        .map(|data| Python::attach(|py| format!("{:?}", data.bind(py))))
        .unwrap_or_default();

    let path_repr = slf.path.as_deref().unwrap_or("None");
    Ok(format!(
        "JsonPathResult(data={data_repr}, path={path_repr:?})",
    ))
}

#[pymodule]
fn jsonpath_rust_bindings(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Finder>()?;
    m.add_class::<JsonPathResult>()?;
    Ok(())
}
