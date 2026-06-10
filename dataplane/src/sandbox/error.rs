use thiserror::Error;

#[derive(Error, Debug)]
pub enum SandboxError {
    #[error("namespace error: {0}")]
    Namespace(String),

    #[error("cgroup error: {0}")]
    Cgroup(String),

    #[error("process error: {0}")]
    Process(String),

    #[error("cleanup error: {0}")]
    Cleanup(String),
}