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
    }};
}

pub(crate) use err;
