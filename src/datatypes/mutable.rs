use tracing::instrument;

use crate::{
    DataType, DatatypeError, DatatypeState,
    datatypes::{common::ReturnType, crdts::Crdt},
    operations::{Operation, transaction::Transaction},
    types::operation_id::OperationId,
};

#[derive(Debug)]
pub struct MutableDatatype {
    pub crdt: Crdt,
    pub state: DatatypeState,
    pub op_id: OperationId,
    pub transaction: Option<Transaction>,
}

impl MutableDatatype {
    pub fn new(r#type: DataType, state: DatatypeState) -> Self {
        Self {
            crdt: Crdt::new(r#type),
            state,
            op_id: OperationId::new(),
            transaction: Default::default(),
        }
    }

    pub fn end_transaction(&mut self, tag: Option<String>, committed: bool) {
        if committed {
            if let Some(mut tx) = self.transaction.take() {
                tx.set_tag(tag);
                // do something related to rollback
            }
        } else {
            // do something related to rollback
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
