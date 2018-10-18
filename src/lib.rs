
#![feature(specialization)]

#[macro_use]
extern crate pyo3;
extern crate rayon;
#[macro_use]
extern crate lazy_static;

use pyo3::prelude::*;
use std::sync::atomic::{AtomicUsize, Ordering};

lazy_static!{
    static ref g_counter: AtomicUsize = AtomicUsize::new(0);
}


#[pyclass]
pub struct SubmitInfo {
    id: usize,
    creation_date: String,
    error: Option<String>,
    token: PyToken,
}


#[pymethods]
impl SubmitInfo {
    #[new]
    fn __new__(obj: &PyRawObject) -> PyResult<()> {
        obj.init(|token| {
            SubmitInfo {
                id: g_counter.fetch_add(1, Ordering::SeqCst),
                creation_date: "Hello".to_string(),
                error: None,
                token: token
            }
        })
    }
}

/// This module is a python module implemented in Rust.
#[pymodinit]
fn pythoncurrent(_py: Python, m: &PyModule) -> PyResult<()> {
    println!("pythoncurrent() init");
    #[pyfn(m, "submit")]
    pub fn submit_py(py: Python/*, task: PyObject*/) -> PyResult<()> {
        println!("submit()");
        // let task_function = |x: f64| {
        //     let args = (x,);
        //     println!("task_submit()");
        //     let obj: PyObject = task.call1(py, args).unwrap();
        //     println!("task_submit() 2");
        //     let res: f64 = obj.extract::<f64>(py).unwrap();
        //     println!("task_submit() 3");
        //     res
        // };

        // println!("submit() 1");
        // task_function(64f64);
        // println!("submit() 1");
        Ok(())
    }

    #[pyfn(m, "say_hello")]
    pub fn say_hello(_py: Python) -> PyResult<()> {
        println!("Hello there!");
        Ok(())
    }

    m.add_class::<SubmitInfo>()?;
    
    Ok(())
}
