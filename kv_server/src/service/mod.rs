use std::sync::Arc;
use tracing::debug;
use crate::{CommandRequest, CommandResponse, KvError};
use crate::command_request::RequestData;
use crate::storage::memory::MemTable;
use crate::storage::Storage;


mod command_service;

pub trait CommandService {
    fn execute(self, store: &impl Storage) -> CommandResponse;
}

pub struct Service<Store = MemTable> {
    inner: Arc<ServiceInner<Store>>,
}

pub struct ServiceInner<Store> {
    store: Store,
}

impl<Store: Storage> Service<Store> {
    pub fn new(store: Store) -> Self {
        Self {
            inner: Arc::new(ServiceInner {
                store
            })
        }
    }

    pub fn execute(&self, cmd: CommandRequest) -> CommandResponse {
        debug!("Got request: {:?}", cmd);
        let res = dispatch(cmd, &self.inner.store);

        res
    }
}

pub fn dispatch<Store: Storage>(cmd: CommandRequest, store: &Store) -> CommandResponse {
    match cmd.request_data {
        Some(RequestData::Hget(param)) => param.execute(store),
        Some(RequestData::Hgetall(param)) => param.execute(store),
        Some(RequestData::Hset(param)) => param.execute(store),
        None => KvError::InvalidCommand("Request has no data".into()).into(),
        _ => KvError::Internal("Not implemented".into()).into()
    }
}