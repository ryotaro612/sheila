///
#[derive(Debug, PartialEq, Clone)]
pub(crate) enum Command {
    Stop {
        monitor: Option<String>,
    },
    Status,
    Shutdown,
    Play {
        files: Vec<String>,
        monitor: Option<String>,
        random: bool,
    },
}

/// Represents an error that can occur at a server.
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

pub(crate) fn make_invalid_params(msg: &str) -> ErrorReason {
    ErrorReason::InvalidParams {
        reason: msg.to_string(),
    }
}
