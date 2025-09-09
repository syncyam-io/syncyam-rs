pub mod clients;
pub mod datatypes;

macro_rules! err {
    ($enum_variant:path) => {{
        let err = $enum_variant("".to_string());
        tracing::error!("{}\n{}", err, std::backtrace::Backtrace::capture());
        err
    }};

    ($enum_variant:path, $msg:expr) => {{
        let err = $enum_variant(std::format!("{}", $msg));
        tracing::error!("{}\n{}", err, std::backtrace::Backtrace::capture());
        err
    }};
}

pub(crate) use err;

#[cfg(test)]
mod tests_datatype_errors {
    use crate::{ClientError, DatatypeError, errors::err};

    #[test]
    fn can_compare_errors() {
        let e1 = DatatypeError::FailedTransaction("e1".to_string());
        let e2 = DatatypeError::FailedTransaction("e2".to_string());
        assert_eq!(e1, e2);

        let e3 = DatatypeError::FailedToDeserialize("e2".to_string());
        assert_ne!(e2, e3);
    }

    #[test]
    fn can_use_err_macro() {
        let d1 = err!(DatatypeError::FailedToDeserialize, "datatype error");
        let d2 = err!(DatatypeError::FailedTransaction);
        assert_ne!(d1, d2);
        let c1 = err!(
            ClientError::CannotSubscribeOrCreateDatatype,
            "clients error"
        );
        let c2 = err!(ClientError::CannotSubscribeOrCreateDatatype);
        assert_eq!(c1, c2);
    }
}
