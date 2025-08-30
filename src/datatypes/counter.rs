use std::{error::Error, sync::Arc};

use tracing::Span;

use crate::{
    DataType, DatatypeError, DatatypeState, IntoString,
    datatypes::{
        common::ReturnType,
        crdts::Crdt,
        datatype::DatatypeBlanket,
        datatype_instrument,
        transactional::{TransactionContext, TransactionalDatatype},
    },
    operations::Operation,
};

/// A counter is a conflict-free datatype that supports increment operations.
#[derive(Clone)]
pub struct Counter {
    datatype: Arc<TransactionalDatatype>,
    tx_ctx: Arc<TransactionContext>,
}

impl Counter {
    // TODO: this should be pub (crate)
    pub fn new(key: String, state: DatatypeState) -> Self {
        Counter {
            datatype: Arc::new(TransactionalDatatype::new(&key, DataType::Counter, state)),
            tx_ctx: Default::default(),
        }
    }

    datatype_instrument! {
    pub fn increase_by(&self, delta: i64) -> i64 {
            let span = Span::current();
        let op = Operation::new_counter_increase(delta);
        match self
            .datatype
            .execute_local_operation_as_tx(self.tx_ctx.clone(), op, span)
        {
            Ok(ReturnType::Counter(c)) => c,
            _ => self.get_value(),
        }
    }}

    pub fn increase(&self) -> i64 {
        self.increase_by(1)
    }

    pub fn get_value(&self) -> i64 {
        let mutable = self.datatype.mutable.read();
        let Crdt::Counter(c) = &mutable.crdt;
        c.value()
    }

    pub fn transaction(
        &self,
        tag: impl IntoString,
        tx_func: fn(Counter) -> Result<(), Box<dyn Error>>,
    ) -> Result<(), DatatypeError> {
        let this_tx_ctx = Arc::new(TransactionContext::new(tag));
        let this_tx_ctx_clone = this_tx_ctx.clone();
        let do_tx_func = move || {
            let mut counter_clone = self.clone();
            counter_clone.tx_ctx = this_tx_ctx_clone.clone();
            match tx_func(counter_clone) {
                Ok(_) => Ok(()),
                Err(e) => Err(DatatypeError::FailedTransaction(e.to_string())),
            }
        };
        self.datatype
            .do_transaction(this_tx_ctx, do_tx_func, Span::current())
    }
}

impl DatatypeBlanket for Counter {
    fn get_core(&self) -> &TransactionalDatatype {
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
        let counter = Counter::new(module_path!().to_owned(), Default::default());
        assert_eq!(counter.get_type(), DataType::Counter);
        assert_eq!(counter.get_key(), module_path!().to_string());
        assert_eq!(counter.get_state(), Default::default());
    }

    #[test]
    fn can_execute_counter_operations() {
        let counter = Counter::new(module_path!().to_owned(), Default::default());
        assert_eq!(1, counter.increase());
        assert_eq!(11, counter.increase_by(10));
        assert_eq!(11, counter.get_value());
    }
}
