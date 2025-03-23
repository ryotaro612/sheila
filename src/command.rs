pub(crate) enum Command {
    Stop,
}

/**
 *
 */
#[derive(Debug)]
pub(crate) enum ErrorReason {
    InvalidParams { reason: String },
    ServerError { reason: String },
}
