use std::fmt::Debug;

pub use crate::{
    datatypes::{counter::Counter, datatype::Datatype},
    errors::datatypes::DatatypeError,
    types::datatype::{DataType, DatatypeState},
};

mod constants;
pub(crate) mod datatypes;
pub(crate) mod errors;
pub(crate) mod observability;
pub(crate) mod operations;
pub(crate) mod types;
pub(crate) mod utils;

pub trait IntoString: Into<String> + Debug {}

impl<T: Into<String> + Debug> IntoString for T {}

#[cfg(feature = "tracing")]
#[ctor::ctor]
pub fn init_tracing_subscriber() {
    use tracing::level_filters::LevelFilter;
    observability::tracing_for_test::init(LevelFilter::TRACE);
}
