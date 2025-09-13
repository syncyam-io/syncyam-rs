use std::sync::Arc;

use tracing::instrument;

use crate::{
    DataType, DatatypeError, DatatypeState,
    datatypes::{common::ReturnType, crdts::Crdt, rollback::RollbackData},
    operations::{Operation, transaction::Transaction},
    types::operation_id::OperationId,
};

#[derive(Debug)]
pub struct MutableDatatype {
    pub crdt: Crdt,
    pub state: DatatypeState,
    pub op_id: OperationId,
    pub transaction: Option<Transaction>,
    pub rollback: RollbackData,
}

impl MutableDatatype {
    pub fn new(r#type: DataType, state: DatatypeState) -> Self {
        Self {
            crdt: Crdt::new(r#type),
            state,
            op_id: OperationId::new(),
            transaction: Default::default(),
            rollback: Default::default(),
        }
    }

    pub fn set_rollback(&mut self) {
        self.rollback
            .set(&self.op_id, self.crdt.serialize(), self.state);
    }

    #[instrument(skip_all)]
    pub fn do_rollback(&mut self) {
        self.op_id = self.rollback.op_id.clone();
        self.state = self.rollback.state;
        self.crdt.deserialize(&self.rollback.crdt);
        let transactions = self.rollback.transactions.clone();

        for tx in transactions.iter() {
            self.replay_transaction(tx);
        }
    }

    pub fn end_transaction(&mut self, tag: Option<String>, committed: bool) {
        if committed {
            if let Some(mut tx) = self.transaction.take() {
                tx.set_tag(tag);
                let tx = Arc::new(tx);
                self.rollback.push_transaction(tx);
            }
        } else {
            self.do_rollback();
        }
    }

    fn replay_local_operation(
        &mut self,
        op: &Operation,
        op_id: &OperationId,
    ) -> Result<ReturnType, DatatypeError> {
        self.op_id.sync(op_id);
        let result = self.crdt.execute_local_operation(op);
        if result.is_err() {
            // this cannot happen
            unreachable!()
        }
        result
    }

    fn replay_transaction(&mut self, tx: &Arc<Transaction>) {
        if *tx.cuid() == self.op_id.cuid {
            let mut op_id = tx.get_op_id();
            tx.iter().for_each(|op| {
                op_id.lamport = op.lamport;
                self.replay_local_operation(op, &op_id).unwrap();
            });
        } else {
            // replay remote operation
        }
    }

    #[instrument(skip_all)]
    pub fn execute_local_operation(
        &mut self,
        mut op: Operation,
    ) -> Result<ReturnType, DatatypeError> {
        let is_new_tx = self.transaction.is_none();
        if is_new_tx {
            self.transaction = Some(Transaction::new(&mut self.op_id));
        }
        op.set_lamport(self.op_id.next_lamport());
        let result = self.crdt.execute_local_operation(&op);
        if result.is_ok() {
            if let Some(tx) = self.transaction.as_mut() {
                tx.push_operation(op);
            }
        } else {
            if is_new_tx {
                self.op_id.prev_cseq();
                self.transaction = None;
            }
            self.op_id.prev_lamport();
        }
        result
    }
}
