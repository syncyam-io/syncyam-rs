use std::sync::Arc;

use crate::{
    DataType, DatatypeState,
    datatypes::{datatype::DatatypeBlanket, thread_safe::ThreadSafeDatatype},
    operations::Operation,
};

/// Counter is a conflict-free datatype that can be increased.
pub struct Counter {
    datatype: Arc<ThreadSafeDatatype>,
}

impl Counter {
    pub(crate) fn new(key: String, state: DatatypeState) -> Self {
        Counter {
            datatype: Arc::new(ThreadSafeDatatype::new(&key, DataType::Counter, state)),
        }
    }

    pub fn increase_by(&self, delta: i64) -> i64 {
        let _op = Operation::new_counter_increase(delta);
        0
    }

    pub fn increase(&self) -> i64 {
        self.increase_by(1)
    }
}

impl DatatypeBlanket for Counter {
    fn get_core(&self) -> &ThreadSafeDatatype {
        self.datatype.as_ref()
    }
}

#[cfg(test)]
mod tests_counter {
    use crate::{
        DataType,
        datatypes::{counter::Counter, datatype::Datatype},
    };

    #[test]
    fn can_assert_send_and_sync_traits() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<Counter>();
    }

    #[test]
    fn can_call_public_blanket_trait_methods() {
        let counter = Counter::new(module_path!().to_string(), Default::default());
        assert_eq!(counter.get_type(), DataType::Counter);
        assert_eq!(counter.get_key(), module_path!().to_string());
        assert_eq!(counter.get_state(), Default::default());
    }
}
