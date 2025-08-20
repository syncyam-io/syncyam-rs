pub use crate::types::datatype::{DataType, DatatypeState};

#[allow(dead_code)]
mod constants;
mod datatypes;
pub(crate) mod observability;
pub(crate) mod types;
pub(crate) mod utils;

#[cfg(feature = "tracing")]
#[ctor::ctor]
pub fn init_tracing_subscriber() {
    use tracing::level_filters::LevelFilter;
    observability::tracing_for_test::init(LevelFilter::TRACE);
}
