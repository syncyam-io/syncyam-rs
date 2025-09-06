use std::{collections::VecDeque, fmt::Debug, sync::Arc};

use crate::{
    DatatypeState, operations::transaction::Transaction, types::operation_id::OperationId,
};

#[derive(Default)]
pub struct RollbackData {
    pub crdt: Box<[u8]>,
    pub op_id: OperationId,
    pub state: DatatypeState,
    pub transactions: VecDeque<Arc<Transaction>>,
}

impl RollbackData {
    pub fn push_transaction(&mut self, tx: Arc<Transaction>) {
        self.transactions.push_back(tx);
    }

    pub fn set(&mut self, op_id: &OperationId, crdt: Box<[u8]>, state: DatatypeState) {
        self.op_id = op_id.clone();
        self.state = state;
        self.crdt = crdt;
        self.transactions.clear();
    }
}

impl Debug for RollbackData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_map()
            .entry(&"tx len", &self.transactions.len())
            .entry(&"crdt size", &self.crdt.len())
            .finish()
    }
}
