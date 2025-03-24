use pyo3::prelude::*;

/// Prints a message.
#[pyfunction]
fn hello1(repo: String, owner: String) -> PyResult<String> {
    Ok(format!(
        "Hello1 from xiaotian-py-binding! repo: {}, owner: {}",
        repo, owner
    ))
}

#[pyfunction]
fn fetch_updates() -> PyResult<String> {
    let res: Result<String, String> =
        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(async move {
                let processor = xiaotian::default_processor().await.unwrap();
                processor.schedule_handler.run().await;
                Ok("success".to_string())
            });
    Ok(res.unwrap())
}

/// A Python module implemented in Rust.
#[pymodule]
fn _lowlevel(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(hello1, m)?)?;
    m.add_function(wrap_pyfunction!(fetch_updates, m)?)?;
    Ok(())
}
