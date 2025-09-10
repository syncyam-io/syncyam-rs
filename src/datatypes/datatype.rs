use crate::{DataType, DatatypeState, datatypes::transactional::TransactionalDatatype};

/// The `Datatype` trait defines the common interface for all
/// conflict-free datatypes (e.g., Counter, Register).
///
/// Each datatype exposes:
/// - a **key**: a unique identifier used to distinguish instances,
/// - a **type**: an enum variant of [`DataType`] describing the kind of datatype,
/// - a **state**: a [`DatatypeState`] indicating the current lifecycle/status.
///
///
/// # Example
/// ```
/// use syncyam::Client;
/// use syncyam::{Counter, Datatype};
/// use syncyam::{DatatypeState, DataType};
/// let client = Client::builder("test-collection", "test-client").build().unwrap();
/// let counter = client.create_counter("test-counter".to_string()).unwrap();
/// assert_eq!(counter.get_key(), "test-counter");
/// assert_eq!(counter.get_type(), DataType::Counter);
/// assert_eq!(counter.get_state(), DatatypeState::DueToCreate);
/// ```
pub trait Datatype {
    fn get_key(&self) -> &str;
    fn get_type(&self) -> DataType;
    fn get_state(&self) -> DatatypeState;
}

pub trait DatatypeBlanket {
    fn get_core(&self) -> &TransactionalDatatype;
}

impl<T> Datatype for T
where
    T: DatatypeBlanket,
{
    fn get_key(&self) -> &str {
        self.get_core().get_key()
    }

    fn get_type(&self) -> DataType {
        self.get_core().get_type()
    }

    fn get_state(&self) -> DatatypeState {
        self.get_core().get_state()
    }
}

#[cfg(test)]
mod tests_datatype_trait {
    use crate::{
        DataType, DatatypeState,
        datatypes::{datatype::Datatype, transactional::TransactionalDatatype},
    };

    #[test]
    fn can_call_datatype_trait_functions() {
        let key = module_path!();
        let data = TransactionalDatatype::new(
            key,
            DataType::Counter,
            Default::default(),
            Default::default(),
        );
        assert_eq!(data.get_key(), key);
        assert_eq!(data.get_type(), DataType::Counter);
        assert_eq!(data.get_state(), DatatypeState::DueToCreate);
    }
}
