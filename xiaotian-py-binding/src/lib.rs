use std::sync::Arc;

use pyo3::{exceptions::PyTypeError, prelude::*};
use xiaotian::{log::init_logger, models::SourceType, process::Processor, storage::MemoryStorage};

#[pyclass(name = "Processor")]
pub struct PyProcessor {
    processor: Arc<Processor<MemoryStorage>>,
    runtime: Arc<tokio::runtime::Runtime>,
}

#[pymethods]
impl PyProcessor {
    #[new]
    fn try_new() -> PyResult<Self> {
        init_logger();
        let config = xiaotian::AppConfig::load("config.toml").unwrap();

        let runtime = Arc::new(tokio::runtime::Runtime::new().unwrap());
        runtime.clone().block_on(async move {
            let processor = xiaotian::default_processor(&config).await.unwrap();
            Ok(Self {
                processor: Arc::new(processor),
                runtime,
            })
        })
    }

    fn get_source_list(&self, source_type: i8) -> PyResult<Vec<(i64, String)>> {
        let source_type = SourceType::try_from(source_type).map_err(PyTypeError::new_err)?;
        let res: Vec<(i64, String)> = self.runtime.block_on(async move {
            match source_type {
                SourceType::GitHub => self
                    .processor
                    .list_repositories()
                    .await
                    .unwrap_or_default()
                    .iter()
                    .map(|repo| (repo.id as i64, repo.full_name()))
                    .collect(),
                SourceType::HackerNews => self
                    .processor
                    .list_hacker_news()
                    .await
                    .unwrap_or_default()
                    .iter()
                    .map(|hn| (hn.id as i64, hn.feed_type.to_string()))
                    .collect(),
            }
        });
        Ok(res)
    }

    fn get_model_list(&self) -> PyResult<Vec<String>> {
        Ok(self.processor.schedule_handler.available_models())
    }

    fn fetch_updates(
        &self,
        source_type: i8,
        source_id: i32,
        model: String,
        to_emails: Vec<String>,
    ) -> PyResult<(String, String)> {
        let source_type = SourceType::try_from(source_type).map_err(PyTypeError::new_err)?;
        let res: Result<(String, String), String> = self.runtime.block_on(async move {
            let res = self
                .processor
                .schedule_handler
                .run_single(source_type, source_id, model, to_emails)
                .await
                .map_err(|e| e.to_string())?;
            Ok(res)
        });
        Ok(res.unwrap())
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn _lowlevel(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyProcessor>()?;
    Ok(())
}
