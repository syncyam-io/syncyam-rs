use parking_lot::RwLock;

use crate::{
    DataType, DatatypeState,
    datatypes::{datatype::Datatype, mutable::MutableDatatype},
};

struct Attributes {
    key: String,
    r#type: DataType,
}

pub struct ThreadSafeDatatype {
    attr: Attributes,
    mutable: RwLock<MutableDatatype>,
}

impl Datatype for ThreadSafeDatatype {
    fn get_key(&self) -> &str {
        self.attr.key.as_ref()
    }

    fn get_type(&self) -> DataType {
        self.attr.r#type
    }

    fn get_state(&self) -> DatatypeState {
        self.mutable.read().state
    }
}

impl ThreadSafeDatatype {
    pub fn new(key: &str, r#type: DataType, state: DatatypeState) -> Self {
        let attr = Attributes {
            key: key.to_owned(),
            r#type,
        };
        let mutable = RwLock::new(MutableDatatype { state });
        Self { attr, mutable }
    }
}
