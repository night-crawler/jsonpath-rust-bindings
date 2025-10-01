use jsonpath_rust::parser::model::JpQuery;
use jsonpath_rust::parser::parse_json_path;
use jsonpath_rust::query::{js_path_process, QueryRef};
use mimalloc::MiMalloc;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pythonize::{depythonize, pythonize};
use serde_json::Value;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[pyclass(frozen)]
struct JsonPathResult {
    #[pyo3(get)]
    data: Option<Py<PyAny>>,

    #[pyo3(get)]
    path: Option<String>,
}

#[pymethods]
impl JsonPathResult {
    fn __repr__(slf: PyRef<'_, Self>) -> PyResult<String> {
        repr_json_path_result(slf)
    }
}

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

    fn find(self_: PyRef<'_, Self>, query: String) -> PyResult<Vec<JsonPathResult>> {
        find_internal(&self_.value, &query, |_| true)
    }
}

fn find_internal(
    value: &Value,
    query: &str,
    predicate: impl Fn(&QueryRef<Value>) -> bool,
) -> PyResult<Vec<JsonPathResult>> {
    let query = parse_query(query)?;
    let processed = match js_path_process(&query, value) {
        Ok(p) => p,
        Err(err) => {
            return Err(PyValueError::new_err(err.to_string()));
        }
    };
    let filtered = processed.into_iter().filter(predicate);

    Python::attach(|py| {
        filtered
            .into_iter()
            .map(|v| map_json_path_value(py, v))
            .collect()
    })
}

fn map_json_path_value(py: Python, jpv: QueryRef<Value>) -> PyResult<JsonPathResult> {
    let path = jpv.clone().path();
    let val = jpv.val();

    let res = JsonPathResult {
        data: Some(pythonize(py, val)?.into_pyobject(py)?.unbind()),
        path: Some(path),
    };

    Ok(res)
}

fn parse_query(query: &str) -> PyResult<JpQuery> {
    match parse_json_path(query) {
        Ok(inst) => Ok(inst),
        Err(err) => Err(PyValueError::new_err(format!("{err:?}"))),
    }
}

fn parse_py_object(obj: Py<PyAny>) -> PyResult<Value> {
    Python::attach(|py| {
        let any = obj.bind(py);
        let value = depythonize(any)?;
        Ok(value)
    })
}

fn repr_json_path_result(slf: PyRef<'_, JsonPathResult>) -> PyResult<String> {
    let data_repr = slf
        .data
        .as_ref()
        .map(|data| Python::attach(|py| format!("{:?}", data.bind(py))))
        .unwrap_or_default();

    let path_repr = match &slf.path {
        Some(path) => path,
        None => "None",
    };
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
