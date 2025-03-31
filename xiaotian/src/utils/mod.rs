use thiserror::Error;

pub mod github_client;

#[derive(Debug, Error)]
pub enum UtilsError {
    #[error("Github error: {0}")]
    GithubError(String),
}
