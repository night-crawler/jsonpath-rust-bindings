use jsonpath_rust::parser::model::JpQuery;
use jsonpath_rust::parser::parse_json_path;
use jsonpath_rust::query::{js_path_process, QueryRef};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pythonize::{depythonize, pythonize};
use serde_json::Value;

#[cfg(any(
    not(target_os = "linux"),
    all(target_os = "linux", target_env = "musl"),
    all(target_os = "linux", target_arch = "x86_64", not(target_env = "musl")),
    all(target_os = "linux", target_arch = "x86", not(target_env = "musl"))
))]
use mimalloc::MiMalloc;
#[cfg(any(
    not(target_os = "linux"),
    all(target_os = "linux", target_env = "musl"),
    all(target_os = "linux", target_arch = "x86_64", not(target_env = "musl")),
    all(target_os = "linux", target_arch = "x86", not(target_env = "musl"))
))]
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

const PYTHON_PACKAGE_VERSION: &str = env!("CARGO_PKG_VERSION");
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
        find_path_value_internal(&self_.value, &query)
    }

    // Execute JSONPath query, return only found data values
    fn find_data(self_: PyRef<'_, Self>, query: String) -> PyResult<Vec<Py<PyAny>>> {
        find_data_internal(&self_.value, &query)
    }

    // Execute JSONPath query, return only found absolute paths
    fn find_absolute_path(self_: PyRef<'_, Self>, query: String) -> PyResult<Vec<String>> {
        find_paths_internal(&self_.value, &query)
    }
}

// Execute JSONPath query and return processed results
fn execute_query<'a>(value: &'a Value, query: &str) -> PyResult<Vec<QueryRef<'a, Value>>> {
    let parsed_query = parse_query(query)?;
    let processed = js_path_process(&parsed_query, value)
        .map_err(|err| PyValueError::new_err(err.to_string()))?;

    Ok(processed.into_iter().collect())
}

// Execute query and return JsonPathResult list
fn find_path_value_internal(value: &Value, query: &str) -> PyResult<Vec<JsonPathResult>> {
    let result = execute_query(value, query)?;

    Python::attach(|py| {
        result
            .into_iter()
            .map(|v| map_json_path_value(py, v))
            .collect()
    })
}

// Execute query and return data value list
fn find_data_internal(value: &Value, query: &str) -> PyResult<Vec<Py<PyAny>>> {
    let result = execute_query(value, query)?;
    Python::attach(|py| result.into_iter().map(|v| map_json_value(py, v)).collect())
}

// Execute query and return path string list
fn find_paths_internal(value: &Value, query: &str) -> PyResult<Vec<String>> {
    Ok(execute_query(value, query)?
        .into_iter()
        .map(|v| v.path())
        .collect::<Vec<_>>())
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
    m.add("__version__", PYTHON_PACKAGE_VERSION)?;
    Ok(())
}
