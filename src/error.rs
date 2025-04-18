#[derive(Debug, thiserror::Error)]
pub enum ErrorKind {
    #[error("resource not found")]
    ResourceNotFound,
    #[error("command buffer not found")]
    CommandBufferNotFound,
}

#[derive(Debug, thiserror::Error)]
pub enum RenderBackendError {
    #[error("kind: {0}")]
    Kind(#[from] ErrorKind),
}

pub type Result<T, E = RenderBackendError> = core::result::Result<T, E>;
