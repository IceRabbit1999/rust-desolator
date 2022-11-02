pub mod abi;

use abi::{command_request::RequestData, *};
use crate::KvError;

impl CommandRequest {
    pub fn new_hset(table: impl Into<String>, key: impl Into<String>, value: Value) -> Self {
        Self {
            request_data: Some(RequestData::Hset(
                Hset {
                    table: table.into(),
                    pair: Some(Kvpair::new(key, value)),
                }
            )),
        }
    }

    pub fn new_hgetall(table: impl Into<String>) -> Self {
        Self {
            request_data: Some(RequestData::Hgetall(Hgetall {
                table: table.into(),
            })),
        }
    }

    pub fn new_hget(table: impl Into<String>, key: impl Into<String>) -> Self {
        Self {
            request_data: Some(RequestData::Hget(Hget {
                table: table.into(),
                key: key.into(),
            }))
        }
    }
}

impl Kvpair {
    pub fn new(key: impl Into<String>, value: Value) -> Self {
        Self {
            key: key.into(),
            value: Some(value),
        }
    }
}

impl From<String> for Value {
    fn from(s: String) -> Self {
        Self {
            value: Some(value::Value::String(s)),
        }
    }
}

impl From<&str> for Value {
    fn from(s: &str) -> Self {
        Self {
            value: Some(value::Value::String(s.to_string())),
        }
    }
}

impl From<i64> for Value {
    fn from(i: i64) -> Self {
        Self {
            value: Some(value::Value::Integer(i))
        }
    }
}

impl From<Value> for CommandResponse {
    fn from(v: Value) -> Self {
        Self {
            status: 200,
            values: vec![v],
            ..Default::default()
        }
    }
}

impl From<Vec<Kvpair>> for CommandResponse {
    fn from(v: Vec<Kvpair>) -> Self {
       Self {
           status: 200,
           pairs: v,
           ..Default::default()
       }
    }
}

impl From<KvError> for CommandResponse {
    fn from(e: KvError) -> Self {
        let mut result = Self {
            status: 500,
            message: e.to_string(),
            values: vec![],
            pairs: vec![]
        };

        match e {
            KvError::NotFound(_,_) => result.status = 404,
            KvError::InvalidCommand(_) => result.status = 400,
            _ => {}
        }

        result
    }


}