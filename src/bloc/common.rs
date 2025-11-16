use std::fmt::Debug;

#[derive(Debug, Clone)]
pub enum LookupError {
    FailedResponseBuilder,
    FailedRequest,
}
