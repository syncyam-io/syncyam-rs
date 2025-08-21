/// DataType represents the kinds of Datatypes in SyncYam
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum DataType {
    Counter = 0,
    Variable = 1,
    List = 2,
}

/// DatatypeState represents the state of a Datatype in SyncYam.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(i32)]
pub enum DatatypeState {
    /// The Datatype is scheduled to be created on the SyncYam server.
    #[default]
    DueToCreate = 0,
    /// The Datatype is scheduled to be subscribed on the SyncYam server.
    DueToSubscribe = 1,
    /// The Datatype is scheduled to be subscribed or created if it does not exist on the SyncYam server.
    DueToSubscribeOrCreate = 2,
    /// The Datatype has been subscribed on the SyncYam server.
    Subscribed = 3,
    /// The Datatype is scheduled to be unsubscribed from the SyncYam server.
    DueToUnsubscribe = 4,
    /// The Datatype is no longer synchronized with the SyncYam server.
    Closed = 5,
    /// The Datatype is scheduled to be deleted from the SyncYam server.
    DueToDelete = 6,
    /// The Datatype has been deleted and synchronized with the SyncYam server.
    Deleted = 7,
}
