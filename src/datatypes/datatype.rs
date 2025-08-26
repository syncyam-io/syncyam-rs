use crate::{datatypes::thread_safe::ThreadSafeDatatype, DataType, DatatypeState};

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
/// use syncyam::{Counter, Datatype};
/// use syncyam::{DatatypeState, DataType};
/// // TODO: this should be updated to create a Counter from a client
/// let counter = Counter::new("example".to_string(), DatatypeState::DueToCreate);
/// assert_eq!(counter.get_key(), "example");
/// assert_eq!(counter.get_type(), DataType::Counter);
/// assert_eq!(counter.get_state(), DatatypeState::DueToCreate);
/// ```
pub trait Datatype {
    fn get_key(&self) -> &str;
    fn get_type(&self) -> DataType;
    fn get_state(&self) -> DatatypeState;
}

pub trait DatatypeBlanket {
    fn get_core(&self) -> &ThreadSafeDatatype;
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
        datatypes::{datatype::Datatype, thread_safe::ThreadSafeDatatype}, DataType,
        DatatypeState,
    };

    #[test]
    fn can_call_datatype_trait_functions() {
        let key = module_path!();
        let data = ThreadSafeDatatype::new(key, DataType::Counter, DatatypeState::DueToCreate);
        assert_eq!(data.get_key(), key);
        assert_eq!(data.get_type(), DataType::Counter);
        assert_eq!(data.get_state(), DatatypeState::DueToCreate);
    }
}
