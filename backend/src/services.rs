use anyhow::Result;
use crypto::digest::Digest;
use crypto::md5::Md5;
use redis::aio::ConnectionManager;
use redis::{AsyncCommands, Client};

#[derive(Clone)]
pub struct UrlShortenerService {
    redis_client: Client,
    redis_connection_manager: ConnectionManager,
}

impl UrlShortenerService {
    pub fn new(redis_client: Client, redis_connection_manager: ConnectionManager) -> Self {
        UrlShortenerService {
            redis_client,
            redis_connection_manager,
        }
    }
    pub async fn get_url(&self, hashed_url: String) -> Result<String> {
        let raw_url = self
            .redis_connection_manager
            .clone()
            .get(hashed_url.clone())
            .await?;
        Ok(raw_url)
    }
    pub async fn post_url(&self, url: String) -> Result<String> {
        let mut md5 = Md5::new();
        md5.input(url.as_bytes());
        let hashed_url = md5.result_str();
        self.redis_connection_manager
            .clone()
            .set(hashed_url.clone(), url)
            .await?;
        Ok(hashed_url)
    }
    pub async fn delete_url(&self, hashed_url: String) -> Result<()> {
        let result = self
            .redis_connection_manager
            .clone()
            .del(hashed_url)
            .await?;
        Ok(())
    }
}
