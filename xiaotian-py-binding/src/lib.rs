use std::sync::Arc;

use pyo3::{exceptions::PyTypeError, prelude::*};
use xiaotian::{models::SourceType, process::Processor, storage::MemoryStorage};

#[pyfunction]
fn get_source_type_list() -> PyResult<Vec<i8>> {
    Ok(vec![SourceType::GitHub.into()])
}

#[pyclass(name = "Processor")]
pub struct PyProcessor {
    processor: Arc<Processor<MemoryStorage>>,
}

#[pymethods]
impl PyProcessor {
    #[new]
    fn try_new() -> PyResult<Self> {
        let config = xiaotian::AppConfig::load("config.toml").unwrap();

        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(async move {
                let processor = xiaotian::default_processor(&config).await.unwrap();
                Ok(Self {
                    processor: Arc::new(processor),
                })
            })
    }

    fn get_source_list(&self) -> PyResult<Vec<(i64, String)>> {
        let res: Vec<(i64, String)> =
            tokio::runtime::Runtime::new()
                .unwrap()
                .block_on(async move {
                    self.processor
                        .list_handler
                        .get_all_repositories()
                        .await
                        .unwrap_or_default()
                        .iter()
                        .map(|repo| (repo.id as i64, repo.name.clone()))
                        .collect()
                });
        Ok(res)
    }

    fn get_model_list(&self) -> PyResult<Vec<String>> {
        Ok(vec!["llama3.2".to_string()])
    }

    fn fetch_updates(
        &self,
        source_type: i8,
        source_id: i32,
        model: String,
        to_emails: Vec<String>,
    ) -> PyResult<String> {
        let source_type = SourceType::try_from(source_type).map_err(|e| PyTypeError::new_err(e))?;
        let res: Result<String, String> =
            tokio::runtime::Runtime::new()
                .unwrap()
                .block_on(async move {
                    self.processor
                        .schedule_handler
                        .run_single(source_type, source_id, model, to_emails)
                        .await
                        .map_err(|e| e.to_string())?;
                    Ok("success".to_string())
                });
        Ok(res.unwrap())
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn _lowlevel(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyProcessor>()?;
    m.add_function(wrap_pyfunction!(get_source_type_list, m)?)?;
    Ok(())
}
