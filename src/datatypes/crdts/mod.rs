use crate::{DataType, datatypes::crdts::counter_crdt::CounterCrdt};

#[allow(dead_code)]
pub mod counter_crdt;

#[derive(Debug)]
#[allow(dead_code)]
pub enum Crdt {
    Counter(CounterCrdt),
}

impl Crdt {
    pub fn new(r#type: DataType) -> Self {
        match r#type {
            DataType::Counter => Crdt::Counter(CounterCrdt::new()),
            _ => unreachable!("invalid type"),
        }
    }
}
