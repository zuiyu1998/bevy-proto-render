#[derive(Debug, thiserror::Error)]
pub enum ErrorKind {
    #[error("resource not found")]
    ResourceNotFound,
    #[error("render pipieline not found")]
    RenderPipelineNotFound,
    #[error("pipieline not match")]
    PipelineNotMatch,
}

#[derive(Debug, thiserror::Error)]
pub enum RenderBackendError {
    #[error("kind: {0}")]
    Kind(#[from] ErrorKind),
}

pub type Result<T, E = RenderBackendError> = core::result::Result<T, E>;
