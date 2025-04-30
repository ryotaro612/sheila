/**
 *
 */
#[derive(Debug, PartialEq, Clone)]
pub(crate) enum Command {
    Stop {
        monitor: Option<String>,
    },
    Status,
    Shutdown,
    Play {
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

pub(crate) fn make_server_error(msg: &str) -> ErrorReason {
    ErrorReason::ServerError {
        reason: msg.to_string(),
    }
}
