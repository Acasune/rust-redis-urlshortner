use crypto::digest::Digest;
use crypto::md5::Md5;
use futures_util::future;
use redis::aio::ConnectionManager;
use redis::{AsyncCommands, Client, RedisError};

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
    pub async fn get_all_urls(&self) -> Result<Vec<String>, RedisError> {
        let mut mgr = self.redis_connection_manager.clone();
        let all_urls: Vec<String> = mgr.keys("*").await?;
        let futures = all_urls.into_iter().map(|key| {
            let k = key as String;
            async move { self.get_url(k.clone()).await }
        });
        let result = future::join_all(futures)
            .await
            .into_iter()
            .map(|v| v.unwrap())
            .collect::<Vec<String>>();
        Ok(result)
    }
    pub async fn get_url(&self, hashed_url: String) -> Result<String, RedisError> {
        let raw_url = self
            .redis_connection_manager
            .clone()
            .get(hashed_url.clone())
            .await?;
        Ok(raw_url)
    }
    pub async fn post_url(&self, url: String) -> Result<String, RedisError> {
        let mut md5 = Md5::new();
        md5.input(url.as_bytes());
        let hashed_url = md5.result_str();
        self.redis_connection_manager
            .clone()
            .set(hashed_url.clone(), url)
            .await?;
        Ok(hashed_url)
    }
    pub async fn delete_url(&self, hashed_url: String) -> Result<(), RedisError> {
        self.redis_connection_manager
            .clone()
            .del(hashed_url)
            .await?;
        Ok(())
    }
}
