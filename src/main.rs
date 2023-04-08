use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};

#[derive(Debug)]
struct MyStruct {
    field1: i32,
    field2: f64,
}

fn vec_to_dataframe(py: Python, my_vec_of_structs: Vec<MyStruct>) -> PyResult<PyObject> {
    let pandas = PyModule::import(py, "pandas")?;
    let dataframe_class = pandas.getattr("DataFrame")?;

    let mut rows = Vec::new();
    for item in my_vec_of_structs {
        let pydict = PyDict::new(py);
        pydict.set_item("field1", item.field1)?;
        pydict.set_item("field2", item.field2)?;
        rows.push(pydict);
    }

    let pylist = PyList::new(py, &rows);

    let py_dataframe = dataframe_class.call1((pylist,))?;

    Ok(py_dataframe.into_py(py))
}


fn main() -> PyResult<()> {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let my_vec = vec![
            MyStruct { field1: 1, field2: 2.0 },
            MyStruct { field1: 3, field2: 4.0 },
            MyStruct { field1: 5, field2: 6.0 },
        ];
        let df = vec_to_dataframe(py, my_vec)?;
        println!("{}", df);
        Ok(())
    })
}
