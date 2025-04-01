#[derive(Debug, PartialEq, Clone)]
pub(crate) enum Command {
    Stop,
    Status,
}

/**
 *
 */
#[derive(Debug)]
pub(crate) enum ErrorReason {
    InvalidParams { reason: String },
    ServerError { reason: String },
}
