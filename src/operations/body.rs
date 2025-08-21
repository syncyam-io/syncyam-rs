use std::fmt::{Debug, Formatter};

use derive_more::Display;

#[derive(Clone, Display)]
pub enum OperationBody {
    #[display("CounterIncrease({_0}")]
    CounterIncrease(CounterIncreaseBody),
}

impl Debug for OperationBody {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}

#[derive(Debug, Clone, Display)]
#[display("delta={delta})")]
pub struct CounterIncreaseBody {
    delta: i64,
}

impl CounterIncreaseBody {
    pub fn new(delta: i64) -> Self {
        Self { delta }
    }
}

#[cfg(test)]
mod tests_operation_body {
    use tracing::info;

    use crate::operations::body::{CounterIncreaseBody, OperationBody};

    #[test]
    fn can_display_and_debug() {
        let body = OperationBody::CounterIncrease(CounterIncreaseBody::new(123));
        info!("{body} vs. {body:?}");
        let s = body.to_string();
        assert!(s.starts_with("CounterIncrease(") && s.ends_with(')'));
    }
}
