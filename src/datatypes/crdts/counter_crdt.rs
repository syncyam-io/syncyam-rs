use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug)]
pub struct CounterCrdt {
    value: i64,
}

impl CounterCrdt {
    pub fn new() -> Self {
        Self { value: 0 }
    }

    pub fn increase_by(&mut self, value: i64) -> i64 {
        self.value += value;
        self.value
    }

    pub fn value(&self) -> i64 {
        self.value
    }
}

impl Serialize for CounterCrdt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i64(self.value)
    }
}

impl<'de> Deserialize<'de> for CounterCrdt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = i64::deserialize(deserializer)?;
        Ok(Self { value })
    }
}

#[cfg(test)]
mod tests_counter_crdt {
    use tracing::info;

    use crate::datatypes::crdts::counter_crdt::CounterCrdt;

    #[test]
    fn can_new_and_increase_counter() {
        let mut counter = CounterCrdt::new();
        counter.increase_by(1);
        counter.increase_by(-2);
        assert_eq!(counter.value(), -1);
    }

    #[test]
    fn can_serialize_and_deserialize_counter_crdt() {
        let mut counter = CounterCrdt::new();
        counter.increase_by(123);

        let serialized = serde_json::to_string(&counter).unwrap();
        info!("serialized counter: {serialized}");
        assert_eq!(serialized, "123");

        let deserialized: CounterCrdt = serde_json::from_str::<CounterCrdt>(&serialized).unwrap();
        assert_eq!(deserialized.value(), counter.value());
    }
}
