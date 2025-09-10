use thiserror::Error;

/// Errors related to client-side operations and datatype management.
///
/// # Equality
/// Two `ClientError` values are considered equal if they are the **same variant**,
/// regardless of their message payload. See the custom `PartialEq` implementation.
///
#[derive(Debug, Error)]
pub enum ClientError {
    /// Subscribe or Create Datatype failed.
    ///
    /// Returned when a request to subscribe or create a datatype is
    /// incompatible with an existing instance for the same key (for
    /// example, mismatched type or datatype state).
    #[error("Cannot subscribe or create datatype: {0}")]
    FailedToSubscribeOrCreateDatatype(String),
}

impl PartialEq for ClientError {
    fn eq(&self, other: &Self) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }
}
