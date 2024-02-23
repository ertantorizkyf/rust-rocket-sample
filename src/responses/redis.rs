use crate::models::redis::RustRocketRedis;
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct RedisHashResponse {
    pub status: String,
    pub data: Vec<RustRocketRedis>,
}
