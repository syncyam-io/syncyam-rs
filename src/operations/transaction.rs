use std::fmt::{Debug, Display, Formatter};

use crate::{
    operations::Operation,
    types::{operation_id::OperationId, uid::Cuid},
};

pub struct Transaction {
    cuid: Cuid,
    cseq: u64,
    sseq: u64,
    tag: Option<String>,
    event: bool,
    operations: Vec<Operation>,
}

impl Transaction {
    pub fn new(op_id: &mut OperationId) -> Self {
        Self {
            cuid: op_id.cuid.clone(),
            cseq: op_id.next_cseq(),
            sseq: 0,
            tag: None,
            event: false,
            operations: vec![],
        }
    }

    pub fn get_op_id(&self) -> OperationId {
        let mut op_id = OperationId::new_with_cuid(&self.cuid);
        op_id.cseq = self.cseq;
        op_id
    }

    pub fn set_tag(&mut self, tag: Option<String>) {
        self.tag = tag;
    }

    pub fn set_event(&mut self, event: bool) {
        self.event = event;
    }

    pub fn push_operation(&mut self, op: Operation) {
        self.operations.push(op);
    }
}

impl Debug for Transaction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}
impl Display for Transaction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let tag_arg = match &self.tag {
            Some(tag) => format!("🔖:{tag}"),
            None => String::new(),
        };
        let event_arg = if self.event { " ✅ " } else { " " };
        let mut lamport_arg = String::from("[]");
        if !self.operations.is_empty() {
            let first = &self.operations[0];
            if self.operations.len() > 1 {
                let last = self.operations.last().unwrap();
                lamport_arg = format!("[{}-{}]", first, last)
            } else {
                lamport_arg = format!("[{}]", first)
            }
        }
        f.write_fmt(format_args!(
            "TX({}{}{}:{}:{}:{})",
            tag_arg, event_arg, self.cuid, self.cseq, self.sseq, lamport_arg,
        ))
    }
}

#[cfg(test)]
mod tests_transaction {
    use tracing::info;

    use super::{OperationId, Transaction};
    use crate::operations::Operation;

    #[test]
    fn can_debug_and_display_transaction() {
        let mut op_id = OperationId::new();
        let mut tx = Transaction::new(&mut op_id);
        info!("{tx}");
        tx.set_tag(Some("tag1".to_string()));
        tx.set_event(true);
        info!("{tx}");
        let mut op1 = Operation::new_counter_increase(1);
        op1.lamport = 1;
        let mut op2 = Operation::new_counter_increase(2);
        op2.lamport = 2;
        tx.operations.push(op1);
        info!("{tx:?}");
        tx.set_event(false);
        tx.operations.push(op2);
        info!("{tx}");

        let op_id_tx = tx.get_op_id();
        info!("{op_id_tx}");
        assert_eq!(op_id, op_id_tx);
    }
}
