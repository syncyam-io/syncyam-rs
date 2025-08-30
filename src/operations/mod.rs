use std::{
    fmt::{Debug, Display, Formatter},
    time::SystemTime,
};

use chrono::Local;

#[cfg(test)]
use crate::operations::body::Delay4TestBody;
use crate::operations::body::{CounterIncreaseBody, OperationBody};

pub mod body;
pub mod transaction;

#[derive(Clone)]
pub struct Operation {
    lamport: u64,
    pub body: OperationBody,
    at: SystemTime,
}

impl Operation {
    pub fn new(body: OperationBody) -> Self {
        Self {
            lamport: Default::default(),
            body,
            at: SystemTime::now(),
        }
    }

    pub fn new_counter_increase(delta: i64) -> Self {
        Self::new(OperationBody::CounterIncrease(CounterIncreaseBody::new(
            delta,
        )))
    }

    #[cfg(test)]
    pub fn new_delay_for_test(duration_ms: u64) -> Self {
        Self::new(OperationBody::Delay4Test(Delay4TestBody::new(duration_ms)))
    }

    pub fn set_lamport(&mut self, lamport: u64) {
        self.lamport = lamport;
    }
}

impl Debug for Operation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.to_string().as_str())
    }
}

impl Display for Operation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{}.{} {:?})",
            self.lamport,
            self.body,
            chrono::DateTime::<Local>::from(self.at)
        ))
    }
}

#[cfg(test)]
mod tests_operations {
    use tracing::info;

    use crate::operations::Operation;

    #[test]
    fn can_new_and_print_operations() {
        let op = Operation::new_counter_increase(1);
        info!("{op} vs. {op:?}");
        let s = op.to_string();
        assert_eq!(s, format!("{op:?}"));
    }
}
