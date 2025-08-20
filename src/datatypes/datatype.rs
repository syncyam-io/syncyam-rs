use crate::{DataType, DatatypeState, datatypes::thread_safe::ThreadSafeDatatype};

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
        DataType, DatatypeState,
        datatypes::{datatype::Datatype, thread_safe::ThreadSafeDatatype},
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
