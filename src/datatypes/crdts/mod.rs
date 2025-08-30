#[cfg(test)]
use crate::operations::body::OperationBody;
use crate::{
    DataType, DatatypeError,
    datatypes::{common::ReturnType, crdts::counter_crdt::CounterCrdt},
    operations::Operation,
};

pub mod counter_crdt;

#[derive(Debug)]
pub enum Crdt {
    Counter(CounterCrdt),
}

impl Crdt {
    pub fn new(r#type: DataType) -> Self {
        match r#type {
            DataType::Counter => Crdt::Counter(CounterCrdt::default()),
            _ => unreachable!("invalid type"),
        }
    }

    pub fn execute_local_operation(&mut self, op: &Operation) -> Result<ReturnType, DatatypeError> {
        #[cfg(test)]
        {
            if let OperationBody::Delay4Test(body) = &op.body {
                body.run();
                return Ok(ReturnType::None);
            }
        }
        match self {
            Crdt::Counter(c) => c.execute_local_operation(op),
        }
    }
}
