use thiserror::Error;

#[derive(Error, Debug)]
pub enum BlockExecutionError {
    #[error("Error is unknown {0}")]
    String(String),
}
