use std::sync::Arc;

use parking_lot::RwLock;

use crate::{
    Counter, DataType, DatatypeState, IntoString, clients::datatype_manager::DatatypeManager,
    datatypes::DatatypeSet, errors::clients::ClientError, types::uid::Cuid,
};

pub struct ClientBuilder {
    collection: String,
    alias: String,
    cuid: Cuid,
}

impl ClientBuilder {
    pub fn build(self) -> Result<Client, ClientError> {
        let client_info = Arc::new(ClientInfo {
            collection: self.collection.into_boxed_str(),
            cuid: self.cuid,
            alias: self.alias.into_boxed_str(),
        });

        Ok(Client {
            info: client_info.clone(),
            datatypes: RwLock::new(DatatypeManager::new(client_info.clone())),
        })
    }
}

#[derive(Default)]
pub struct ClientInfo {
    pub collection: Box<str>,
    pub cuid: Cuid,
    pub alias: Box<str>,
}

pub struct Client {
    info: Arc<ClientInfo>,
    datatypes: RwLock<DatatypeManager>,
}

impl Client {
    pub fn builder(collection: impl IntoString, alias: impl IntoString) -> ClientBuilder {
        ClientBuilder {
            collection: collection.into(),
            alias: alias.into(),
            cuid: Cuid::new(),
        }
    }

    fn subscribe_or_create_datatype(
        &self,
        key: String,
        r#type: DataType,
        state: DatatypeState,
    ) -> Result<DatatypeSet, ClientError> {
        self.datatypes
            .write()
            .subscribe_or_create_datatype(&key, r#type, state)
    }

    pub fn subscribe_counter(&self, key: impl IntoString) -> Result<Counter, ClientError> {
        let ds = self.subscribe_or_create_datatype(
            key.into(),
            DataType::Counter,
            DatatypeState::DueToSubscribe,
        )?;
        let DatatypeSet::Counter(counter) = ds;
        Ok(counter)
    }

    pub fn create_counter(&self, key: impl IntoString) -> Result<Counter, ClientError> {
        let ds = self.subscribe_or_create_datatype(
            key.into(),
            DataType::Counter,
            DatatypeState::DueToCreate,
        )?;
        let DatatypeSet::Counter(counter) = ds;
        Ok(counter)
    }

    pub fn subscribe_or_create_counter(
        &self,
        key: impl IntoString,
    ) -> Result<Counter, ClientError> {
        let ds = self.subscribe_or_create_datatype(
            key.into(),
            DataType::Counter,
            DatatypeState::DueToSubscribeOrCreate,
        )?;
        let DatatypeSet::Counter(counter) = ds;
        Ok(counter)
    }

    pub fn get_datatype(&self, key: &str) -> Option<DatatypeSet> {
        self.datatypes.read().get_datatype(key)
    }

    pub fn get_collection(&self) -> &str {
        &self.info.collection
    }

    pub fn get_alias(&self) -> &str {
        &self.info.alias
    }
}

#[cfg(test)]
mod tests_client {
    use crate::{Datatype, DatatypeState, clients::client::Client};

    #[test]
    fn can_assert_send_and_sync_traits() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<Client>();
    }

    #[test]
    fn can_build_client() {
        let client = Client::builder("collection1", "alias1").build().unwrap();
        assert_eq!(client.get_collection(), "collection1");
        assert_eq!(client.get_alias(), "alias1");
    }

    #[test]
    fn can_use_counter_from_client() {
        let client1 = Client::builder(module_path!(), module_path!())
            .build()
            .unwrap();

        assert!(client1.get_datatype("k1").is_none());

        let counter1 = client1.subscribe_counter("k1").unwrap();
        assert_eq!(counter1.get_state(), DatatypeState::DueToSubscribe);
        assert!(client1.get_datatype("k1").is_some());

        let client2 = Client::builder(module_path!(), module_path!())
            .build()
            .unwrap();
        let counter2 = client2.create_counter("k1").unwrap();
        assert_eq!(counter2.get_state(), DatatypeState::DueToCreate);

        let client3 = Client::builder(module_path!(), module_path!())
            .build()
            .unwrap();
        let counter3 = client3.subscribe_or_create_counter("k1").unwrap();
        assert_eq!(counter3.get_state(), DatatypeState::DueToSubscribeOrCreate);
    }
}
