use std::fmt::{Debug, Formatter};

use derive_more::Display;

#[derive(Clone, Display)]
pub enum OperationBody {
    #[cfg(test)]
    #[display("Delay4Test")]
    Delay4Test(Delay4TestBody),
    #[display("CounterIncrease({_0}")]
    CounterIncrease(CounterIncreaseBody),
}

impl Debug for OperationBody {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}

#[cfg(test)]
#[derive(Debug, Clone, Display)]
#[display("")]
pub struct Delay4TestBody {
    duration_ms: u64,
}

#[cfg(test)]
impl Delay4TestBody {
    pub fn new(duration_ms: u64) -> Self {
        Self { duration_ms }
    }

    pub fn run(&self) {
        use std::{thread::sleep, time::Duration};
        sleep(Duration::from_millis(self.duration_ms));
    }
}

#[derive(Debug, Clone, Display)]
#[display("delta={delta})")]
pub struct CounterIncreaseBody {
    pub delta: i64,
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
