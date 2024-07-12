extern crate reqwest;

use reqwest::Response;
use crate::gelbooru::types::Post;
use serde_json::Value;

use super::types::{APIMethods, Attributes};

/// Gelbooru API URL
pub const API_URL: &str = "https://gelbooru.com";

/// Gelbooru API client struct
pub struct Client {
    /// Optional proxy field, for use if access to Gelbooru is restricted in user country
    proxy: Option<reqwest::Proxy>
}

impl Client {
    /// Creates a new instance of `Client`
    /// 
    /// # Arguments
    /// 
    /// * `proxy` - Optional argument for proxy, any protocol is supported, String, a URL like
    ///     `socks5://127.0.0.1:8381`.
    ///
    pub fn new(proxy: Option<&str>) -> Self {
        if let Some(proxy) = proxy {
            Client {
                proxy: Some(reqwest::Proxy::all(proxy).unwrap())
            }
        } else {
            Client {
                proxy: None
            }
        }
    }

    /// Sends a request to Gelbooru API, returns reqwest Response wrapped in Result
    ///
    /// # Arguments
    ///
    /// * `url` - Request URL
    /// * `proxy` - Optional proxy
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let body = request(format!("{}/{}", API_URL, APIMethods::PostsList.as_str())).await?;
    /// let text = body.text().await?;
    /// println!("{:?}", text);
    /// ```
    async fn request(&self, url: String) -> Result<Response, reqwest::Error> {
        let client = reqwest::Client::builder();
        if let Some(proxy) = &self.proxy {
            client
                .proxy(proxy.clone())
                .build()?
                .get(url)
                .header(reqwest::header::CONTENT_TYPE, "application/json")
                .send()
                .await
        } else {
            client
                .build()?
                .get(url)
                .send()
                .await
        }
    }

    /// Fetches all posts from page, retuns a vector of Post wrapped in Result
    ///
    /// # Arguments
    ///
    /// * `tags` - Argument to get only posts from specified tags
    /// * `page` - Argument to get posts from specified page
    ///
    /// # Examples
    /// ```
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = imgdl_rs::gelbooru::request::Client::new(None);
    ///     let posts = client.fetch_posts(
    ///         "rating:general", 3
    ///     ).await?; // Fetch all posts with tag `rating:general` from page 2
    ///
    ///     println!("{:?}", posts); 
    ///     Ok(())
    /// }
    /// ```
    pub async fn fetch_posts(&self, tags: &str, page: i64) 
        -> Result<Vec<Post>, serde_json::Error> {
        let json: Value = Self::request(
            self,
            format!(
                "{}/{}&tags={tags}&pid={page}",
                API_URL,
                APIMethods::PostsList.as_str()
                )
            ).await.unwrap().json().await.unwrap();
        let posts: Result<Vec<Post>, serde_json::Error> = serde_json::from_value(json["post"].clone());
        posts
    }

    /// Fetches attributes of page, retuns an Attributes struct wrapped in Result
    ///
    /// # Arguments
    ///
    /// * `tags` - Argument to get only posts from specified tags
    /// * `page` - Argument to get attributes for specified page 
    ///
    /// # Examples
    /// ```
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = imgdl_rs::gelbooru::request::Client::new(None);
    ///     let posts = client.fetch_attributes(
    ///         "rating:general", 1
    ///     ).await?; // Fetch attributes for tag `rating:general`
    ///
    ///     println!("{:?}", posts); 
    ///     Ok(())
    /// }
    /// ```
    pub async fn fetch_attributes(&self, tags: &str, page: i64) 
        -> Result<Attributes, serde_json::Error> {
        let json: Value = Self::request(
            self,
            format!(
                "{}/{}&tags={tags}&pid={page}",
                API_URL,
                APIMethods::PostsList.as_str()
                )
            ).await.unwrap().json().await.unwrap();
        let posts: Result<Attributes, serde_json::Error> = serde_json::from_value(json["@attributes"].clone());
        posts
    }
}
