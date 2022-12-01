use crypto::digest::Digest;
use crypto::md5::Md5;
use futures_util::future;
use redis::aio::ConnectionManager;
use redis::{AsyncCommands, Client, RedisError};

use crate::types::KeyValue;

#[derive(Clone)]
pub struct UrlShortenerService {
    redis_connection_manager: ConnectionManager,
}

impl UrlShortenerService {
    pub fn new(redis_client: Client, redis_connection_manager: ConnectionManager) -> Self {
        UrlShortenerService {
            redis_connection_manager,
        }
    }
    pub async fn get_all_urls(&self) -> Result<Vec<KeyValue>, RedisError> {
        let mut mgr = self.redis_connection_manager.clone();
        let all_urls: Vec<String> = mgr.keys("*").await?;
        let futures = all_urls.into_iter().map(|key| async move {
            let url = self.get_url(key.clone()).await;
            KeyValue {
                hashed: key,
                url: url.unwrap(),
            }
        });
        let result = future::join_all(futures)
            .await
            .into_iter()
            .collect::<Vec<_>>();
        Ok(result)
    }
    pub async fn get_url(&self, hashed: String) -> Result<String, RedisError> {
        let raw_url = self
            .redis_connection_manager
            .clone()
            .get(hashed.clone())
            .await?;
        Ok(raw_url)
    }
    pub async fn post_url(&self, url: String) -> Result<String, RedisError> {
        let mut md5 = Md5::new();
        md5.input(url.as_bytes());
        let hashed = md5.result_str();
        self.redis_connection_manager
            .clone()
            .set(hashed.clone(), url)
            .await?;
        Ok(hashed)
    }
    pub async fn delete_url(&self, hashed: String) -> Result<(), RedisError> {
        self.redis_connection_manager
            .clone()
            .del(hashed)
            .await?;
        Ok(())
    }
}
