use anyhow::Result;
use mobc::Pool;
use mobc_redis::redis::AsyncCommands;
use mobc_redis::{redis, RedisConnectionManager};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::time::Duration;

pub type RedisPool = Pool<RedisConnectionManager>;

pub struct Cache {
    pub redis: RedisPool,
}

const CACHE_POOL_MAX_OPEN: u64 = 16;
const CACHE_POOL_MAX_IDLE: u64 = 8;
const CACHE_POOL_TIMEOUT_SECONDS: u64 = 1;
const CACHE_POOL_EXPIRE_SECONDS: u64 = 60;

impl Cache {
    pub fn new(cache_url: &str) -> Result<Self> {
        let client = redis::Client::open(cache_url)?;
        let manager = RedisConnectionManager::new(client);
        let pool = Pool::builder()
            .get_timeout(Some(Duration::from_secs(CACHE_POOL_TIMEOUT_SECONDS)))
            .max_open(CACHE_POOL_MAX_OPEN)
            .max_idle(CACHE_POOL_MAX_IDLE)
            .max_lifetime(Some(Duration::from_secs(CACHE_POOL_EXPIRE_SECONDS)))
            .build(manager);
        Ok(Cache { redis: pool })
    }
    pub async fn set_json<'a, T>(&self, key: &str, value: T) -> Result<()>
    where
        T: Serialize + Deserialize<'a>,
    {
        let mut conn = self.redis.get().await?;
        let bytes = serde_json::to_string(&value).map(|s| s.as_bytes().to_vec())?;

        conn.set(key.to_owned(), bytes).await?;

        Ok(())
    }

    pub async fn get_json<'a, T>(&self, key: &str) -> Result<Option<T>>
    where
        T: Serialize + DeserializeOwned,
    {
        let mut conn = self.redis.get().await?;
        let json: Option<String> = conn.get(key).await?;

        if let Some(json) = json {
            let s: T = serde_json::from_str(&json).map_err(|e| {
                log::warn!("{}", e.to_string());
                e
            })?;
            Ok(Some(s))
        } else {
            Ok(None)
        }
    }

    pub async fn evict(&self, key: &str) -> Result<()> {
        let mut conn = self.redis.get().await?;
        conn.del(key).await?;
        Ok(())
    }

    pub async fn save_datasource_view() {}
}
