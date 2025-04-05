#[derive(Debug, PartialEq, Clone)]
/**
 *
 */
pub(crate) enum Command {
    Stop,
    Status,
    Display {
        file: String,
        monitor: Option<String>,
    },
}

/**
 * Represents errors that the drawer can raise.
 */
#[derive(Debug)]
pub(crate) enum ErrorReason {
    InvalidParams { reason: String },
    ServerError { reason: String },
}
