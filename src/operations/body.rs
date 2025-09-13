use std::fmt::{Debug, Formatter};

use derive_more::Display;

use crate::operations::MemoryMeasurable;

#[derive(Clone, Display)]
pub enum OperationBody {
    #[cfg(test)]
    #[display("Delay4Test")]
    Delay4Test(Delay4TestBody),
    #[display("CounterIncrease{_0}")]
    CounterIncrease(CounterIncreaseBody),
}

impl Debug for OperationBody {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}

impl MemoryMeasurable for OperationBody {
    fn size(&self) -> usize {
        match self {
            #[cfg(test)]
            OperationBody::Delay4Test(body) => body.size(),
            OperationBody::CounterIncrease(body) => body.size(),
        }
    }
}

#[cfg(test)]
#[derive(Debug, Clone, Display)]
#[display("")]
pub struct Delay4TestBody {
    duration_ms: u64,
    success: bool,
}

#[cfg(test)]
impl Delay4TestBody {
    pub fn new(duration_ms: u64, success: bool) -> Self {
        Self {
            duration_ms,
            success,
        }
    }

    pub fn run(&self) -> Result<(), ()> {
        use std::{thread::sleep, time::Duration};
        sleep(Duration::from_millis(self.duration_ms));
        if self.success { Ok(()) } else { Err(()) }
    }
}

#[cfg(test)]
impl MemoryMeasurable for Delay4TestBody {
    fn size(&self) -> usize {
        size_of::<u64>() + size_of::<bool>()
    }
}

#[derive(Debug, Clone, Display)]
#[display("(delta={delta})")]
pub struct CounterIncreaseBody {
    pub delta: i64,
}

impl CounterIncreaseBody {
    pub fn new(delta: i64) -> Self {
        Self { delta }
    }
}

impl MemoryMeasurable for CounterIncreaseBody {
    fn size(&self) -> usize {
        size_of::<i64>()
    }
}

#[cfg(test)]
mod tests_operation_body {
    use tracing::info;

    use crate::operations::{
        MemoryMeasurable,
        body::{CounterIncreaseBody, Delay4TestBody, OperationBody},
    };

    #[test]
    fn can_display_and_debug() {
        let body = OperationBody::CounterIncrease(CounterIncreaseBody::new(123));
        info!("{body} vs. {body:?}");
        let s = body.to_string();
        assert!(s.starts_with("CounterIncrease(") && s.ends_with(')'));
    }

    #[test]
    fn can_measure_body_size() {
        let body = OperationBody::CounterIncrease(CounterIncreaseBody::new(123));
        assert_eq!(body.size(), size_of::<i64>());
        let body = OperationBody::Delay4Test(Delay4TestBody::new(123, true));
        assert_eq!(body.size(), size_of::<u64>() + size_of::<bool>());
    }
}
