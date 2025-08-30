use std::sync::Arc;

use parking_lot::RwLock;
use tracing::{Span, info_span, instrument};

use crate::{
    DataType, DatatypeState, IntoString,
    datatypes::{common::ReturnType, datatype::Datatype, mutable::MutableDatatype},
    errors::datatypes::DatatypeError,
    operations::Operation,
    types::uid::Duid,
    utils::{defer_guard::DeferGuard, no_guard_mutex::NoGuardMutex},
};

#[derive(Debug, Default)]
pub struct TransactionContext {
    tag: Option<String>,
}

impl TransactionContext {
    pub fn new(tag: impl IntoString) -> Self {
        Self {
            tag: Some(tag.into()),
        }
    }

    pub fn has_tag(&self) -> bool {
        self.tag.is_some()
    }
}

impl PartialEq for TransactionContext {
    fn eq(&self, other: &Self) -> bool {
        let s_ptr: *const Self = self;
        let o_ptr: *const Self = other;
        s_ptr == o_ptr
    }
}

#[cfg(test)]
mod tests_transaction_context {
    use std::sync::Arc;

    use super::*;
    #[test]
    fn can_compare_transaction_contexts() {
        let ctx1 = TransactionContext::default();
        let ctx2 = TransactionContext::default();
        assert_eq!(ctx1, ctx1);
        assert_ne!(ctx1, ctx2);

        let ctx3 = Arc::new(TransactionContext::default());
        let ctx4 = ctx3.clone();
        assert_eq!(ctx3, ctx4);
        assert_ne!(ctx1, *ctx3);
    }
}

#[derive(derive_more::Display)]
pub enum BeginTransactionResult<'a> {
    #[display("BeginTx")]
    BeginTx(DeferGuard<'a>),
    #[display("SameCtx")]
    SameCtx,
    #[display("OtherCtx")]
    OtherCtx,
}

pub struct Attributes {
    pub key: String,
    pub r#type: DataType,
    pub duid: Duid,
}

pub struct TransactionalDatatype {
    pub attr: Attributes,
    pub mutable: RwLock<MutableDatatype>,
    tx_ctx: RwLock<Option<Arc<TransactionContext>>>,
    tx_mutex: NoGuardMutex,
}

impl Datatype for TransactionalDatatype {
    fn get_key(&self) -> &str {
        self.attr.key.as_ref()
    }

    fn get_type(&self) -> DataType {
        self.attr.r#type
    }

    fn get_state(&self) -> DatatypeState {
        self.mutable.read().state
    }
}

impl TransactionalDatatype {
    pub fn new(key: &str, r#type: DataType, state: DatatypeState) -> Self {
        let attr = Attributes {
            key: key.to_owned(),
            r#type,
            duid: Duid::new(),
        };
        Self {
            attr,
            mutable: RwLock::new(MutableDatatype::new(r#type, state)),
            tx_ctx: Default::default(),
            tx_mutex: Default::default(),
        }
    }

    #[instrument(skip_all, parent = _parent_span)]
    pub fn execute_local_operation_as_tx(
        &self,
        tx_ctx: Arc<TransactionContext>,
        op: Operation,
        _parent_span: Span,
    ) -> Result<ReturnType, DatatypeError> {
        let span = Span::current();
        let mut _defer_guard = None;
        loop {
            match self.begin_transaction(tx_ctx.clone(), &span) {
                BeginTransactionResult::BeginTx(mut dg) => {
                    dg.add_defer_func(move |_committed| {
                        self.tx_mutex.unlock();
                    });
                    dg.commit();
                    _defer_guard = Some(dg);
                    break;
                }
                BeginTransactionResult::SameCtx => {
                    // After the first call inside do_transaction, subsequent execute_local_operation_as_tx calls in tx_func run as SameCtx.
                    // If execute_local_operation_as_tx is invoked from another thread, the tx_mutex can ensure exclusive execution.
                    let mut dg = DeferGuard::new();
                    dg.add_defer_func(move |_committed| {
                        self.tx_mutex.unlock();
                    });
                    _defer_guard = Some(dg);
                    break;
                }
                BeginTransactionResult::OtherCtx => {
                    // For OtherCtx, begin_transaction is repeatedly attempted until it transitions to BeginTx.
                    // If an operation is already running, the tx_mutex is locked, so we wait until we can acquire the tx_mutex lock.
                    self.wait_for_mutex();
                    std::thread::yield_now();
                }
            }
        }
        self.tx_mutex.lock();
        let mut mutable = self.mutable.write();
        let ret = mutable.execute_local_operation(op)?;
        _defer_guard.as_mut().unwrap().commit();
        Ok(ret)
    }

    #[instrument(skip_all)]
    fn end_transaction(&self, tag: Option<String>, committed: bool) {
        let mut mutable = self.mutable.write();
        mutable.end_transaction(tag, committed);
        self.tx_ctx.write().take();
    }

    #[instrument(skip_all, parent=_parent_span, fields(tag=tx_ctx.tag))]
    fn begin_transaction(
        &self,
        tx_ctx: Arc<TransactionContext>,
        _parent_span: &Span,
    ) -> BeginTransactionResult {
        let mut self_tx_ctx = self.tx_ctx.write();
        // self.tx_ctx defaults to None when no transaction is active.
        // Once a transaction begins, self.tx_ctx is set to the current transaction context.
        // After do_transaction gets BeginTx by begin_transaction, this None condition branch won't execute again.
        if self_tx_ctx.is_none() {
            let curr_tx_ctx = {
                if tx_ctx.has_tag() {
                    tx_ctx.clone()
                } else {
                    // If an operation is executed without a transaction, tx_ctx does not have a tag.
                    // In this case, a new TransactionContext is created,
                    // so BeginTransactionResult::SameCtx cannot be returned for the execution of the concurrent operations.
                    Arc::new(TransactionContext::default())
                }
            };
            *self_tx_ctx = Some(curr_tx_ctx.clone());

            let mut defer_guard = DeferGuard::new();
            defer_guard.add_defer_func(move |committed| {
                self.end_transaction(curr_tx_ctx.tag.to_owned(), committed);
            });
            BeginTransactionResult::BeginTx(defer_guard)
        } else if self_tx_ctx.as_ref().unwrap() == &tx_ctx {
            // When execute_local_op_as_tx is called within tx_func of do_transaction, SameCtx should be returned.
            // This means self.tx_ctx is not replaced by end_transaction.
            BeginTransactionResult::SameCtx
        } else {
            // When either execute_local_op_as_tx or do_transaction is called from another thread while do_transaction is running, OtherTx is returned.
            // OtherTx causes begin_transaction to be repeated in a loop until it transitions to BeginTx.
            BeginTransactionResult::OtherCtx
        }
    }

    #[instrument(skip_all, parent=_parent_span)]
    pub fn do_transaction<F>(
        &self,
        tx_ctx: Arc<TransactionContext>,
        tx_func: F,
        _parent_span: Span,
    ) -> Result<(), DatatypeError>
    where
        F: FnOnce() -> Result<(), DatatypeError>,
    {
        let parent_span = Span::current();
        loop {
            match self.begin_transaction(tx_ctx.clone(), &parent_span) {
                BeginTransactionResult::BeginTx(mut dg) => {
                    let span = info_span!("tx_func");
                    return span.in_scope(|| tx_func().inspect(|_x| dg.commit()));
                }
                BeginTransactionResult::SameCtx => {
                    // do_transaction should not / cannot be called with the same tx_ctx from different threads
                    unreachable!(
                        "do_transaction should not be called concurrently with same context"
                    );
                }
                BeginTransactionResult::OtherCtx => {
                    // This can occur when the current transaction cannot begin due to any other concurrent operation or transaction.
                    self.wait_for_mutex();
                }
            }
        }
    }

    #[inline]
    fn wait_for_mutex(&self) {
        self.tx_mutex.lock();
        self.tx_mutex.unlock();
    }
}

#[cfg(test)]
mod tests_transactional {
    use std::sync::Arc;

    use parking_lot::Mutex;
    use tracing::{Span, info, info_span};

    use crate::{
        DataType, DatatypeState,
        datatypes::transactional::{TransactionContext, TransactionalDatatype},
        operations::Operation,
    };

    #[tokio::test(flavor = "multi_thread", worker_threads = 10)]
    async fn can_do_transaction() {
        let tx_dt = Arc::new(TransactionalDatatype::new(
            "can_do_transaction",
            DataType::Counter,
            DatatypeState::default(),
        ));
        let span = info_span!("test_do_transaction");
        let _guard = span.enter();

        let mut join_handles = vec![];

        for i in 0..5 {
            let tx_dt = tx_dt.clone();
            let parent_span = span.clone();
            join_handles.push(tokio::spawn(async move {
                let tx_ctx = Arc::new(TransactionContext::new(format!("test_tx_{i}")));
                tx_dt.clone().do_transaction(
                    tx_ctx.clone(),
                    move || {
                        let span = Span::current();
                        tx_dt.execute_local_operation_as_tx(
                            tx_ctx.clone(),
                            Operation::new_delay_for_test(1),
                            span,
                        )?;
                        Ok(())
                    },
                    parent_span.clone(),
                )
            }));
        }
        for jh in join_handles {
            jh.await.unwrap().expect("failed to join thread");
        }
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 10)]
    async fn can_execute_the_same_tx_ctx_continuously_with_none_tx_ctx() {
        let tx_dt = Arc::new(TransactionalDatatype::new(
            module_path!(),
            DataType::Counter,
            Default::default(),
        ));
        let span = info_span!("test_continuously_tx_ctx");
        let _span_guard = span.enter();
        let tx_ctx_with_tag = Arc::new(TransactionContext::new("test_tx"));
        let tx_ctx_with_no_tag: Arc<TransactionContext> = Default::default();
        let mut join_handles = vec![];
        let executions = Arc::new(Mutex::new(vec![]));
        for i in 0..5 {
            {
                let tx_dt = tx_dt.clone();
                let tx_ctx = tx_ctx_with_tag.clone();
                let op = Operation::new_delay_for_test(50);
                let parent_span = span.clone();
                let executions = executions.clone();
                join_handles.push(tokio::spawn(async move {
                    tx_dt
                        .execute_local_operation_as_tx(tx_ctx, op, parent_span)
                        .unwrap();
                    executions.lock().push(-(i + 1));
                    info!("with tag:{i}");
                }));
            }
            {
                let tx_dt = tx_dt.clone();
                let tx_ctx = tx_ctx_with_no_tag.clone();
                let op = Operation::new_delay_for_test(50);
                let executions = executions.clone();
                join_handles.push(tokio::spawn(async move {
                    let span = Span::current();
                    tx_dt
                        .execute_local_operation_as_tx(tx_ctx, op, span)
                        .unwrap();
                    executions.lock().push(i + 1);
                    info!("with no tag:{i}");
                }));
            }
        }

        for jh in join_handles {
            jh.await.unwrap();
        }
        let executions = executions.lock();
        info!("{:?}", *executions);
        assert!(check_negatives_contiguous(&executions))
    }

    fn check_negatives_contiguous(nums: &Vec<i32>) -> bool {
        let mut in_negative_block = false;
        let mut finished_negative_block = false;

        for &n in nums {
            if n < 0 {
                if finished_negative_block {
                    return false;
                }
                in_negative_block = true;
            } else if in_negative_block {
                finished_negative_block = true;
                in_negative_block = false;
            }
        }

        true
    }
}
