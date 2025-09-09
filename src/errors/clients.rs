use thiserror::Error;

#[derive(Debug, Error)]
pub enum ClientError {
    #[error("Cannot subscribe or create datatype: {0}")]
    CannotSubscribeOrCreateDatatype(String),
}

impl PartialEq for ClientError {
    fn eq(&self, other: &Self) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }
}
