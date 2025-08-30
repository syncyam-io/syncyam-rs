use thiserror::Error;

#[derive(Debug, Error)]
pub enum DatatypeError {
    #[error("failed to do transaction: {0}")]
    FailedTransaction(String),
}

impl PartialEq for DatatypeError {
    fn eq(&self, other: &Self) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }
}

#[cfg(test)]
mod tests_datatype_errors {
    use super::*;

    #[test]
    fn can_compare_datatypes_errors() {
        let e1 = DatatypeError::FailedTransaction("e1".to_string());
        let e2 = DatatypeError::FailedTransaction("e2".to_string());
        assert_eq!(e1, e2)
    }
}
