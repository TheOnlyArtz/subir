use redis::{Connection, RedisError, FromRedisValue, from_redis_value, ErrorKind, Commands};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RedisSavedFileInfo {
    pub name: String,
    pub media_link: String,
    pub timestamp: i64,
}

impl FromRedisValue for RedisSavedFileInfo {
    fn from_redis_value(v: &redis::Value) -> redis::RedisResult<Self> {
        let v: String = from_redis_value(v)?;
        let splitted =  v.split('|');
        let mut iter = splitted.into_iter();

        let name = iter.next().unwrap_or("no name");
        let media_link = iter.next().unwrap_or("no link");
        let timestamp: i64 = iter.next().unwrap().parse().unwrap_or(0);

        Ok(RedisSavedFileInfo {
            name: name.to_string(),
            media_link: media_link.to_string(),
            timestamp
        })
    }
}

pub struct RedisService {
    pub connection: Connection,
}

impl RedisService {
    pub fn from(client: redis::Client) -> Result<Self, RedisError> {
        Ok(Self {
            connection: client.get_connection()?,
        })
    }
}

pub trait Queries {
    fn get_files(&mut self, offset: isize, limit: isize) -> Result<Vec<RedisSavedFileInfo>, RedisError>;
    fn push_file_info(&mut self, info: RedisSavedFileInfo) -> Result<isize, RedisError>;
}

impl Queries for RedisService {
    fn get_files(&mut self, offset: isize, limit: isize) -> Result<Vec<RedisSavedFileInfo>, RedisError> {
        self.connection.lrange("uploaded_files", offset, limit)
    }

    fn push_file_info(&mut self, info: RedisSavedFileInfo) -> Result<isize, RedisError> {
        let parsed = format!("{}|{}|{}", info.name, info.media_link, info.timestamp);
        self.connection.rpush("uploaded_files", parsed)
    }
}
