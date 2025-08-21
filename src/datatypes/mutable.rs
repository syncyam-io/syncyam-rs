use crate::{DatatypeState, datatypes::crdts::Crdt};

pub struct MutableDatatype {
    pub state: DatatypeState,
    #[allow(dead_code)]
    pub crdt: Crdt,
}
