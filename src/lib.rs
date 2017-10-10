#[macro_use]
extern crate cpython;
use cpython::{Python,PyResult};

fn double(_py:Python, x:i32)->PyResult<i32>{
	Ok(x*2)
}

py_module_initializer!(exemple,initexample, PyInit_example, |py,m|{
	m.add(py,"double",py_fn!(py,double(x:i32)))?;
	Ok(())
});