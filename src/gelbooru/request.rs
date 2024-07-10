extern crate reqwest;

use reqwest::Response;
use serde_json::Value;

use super::types;

/// Sends a request to Gelbooru API, returns reqwest Response wrapped in Result
///
/// # Arguments
///
/// * `root_url` - Root URL of Gelbooru-powered site
/// * `proxy` - Optional proxy
///
/// # Examples
///
/// ```ignore
/// let body = request("blog/boosty/post").await?;
/// let text = body.text().await?;
/// println!("{:?}", text);
/// ```
async fn request(root_url: String, proxy: Option<reqwest::Proxy>) -> Result<Response, reqwest::Error> {
    let client = reqwest::Client::builder();
    if let Some(proxy_unwrapped) = proxy {
        client
            .proxy(proxy_unwrapped)
            .build()?
            .get(root_url)
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .send()
            .await
    } else {
        client
            .build()?
            .get(root_url)
            .send()
            .await
    }
}

/// Fetches all posts from page, retuns a vector of Post wrapped in Result
///
/// # Arguments
///
/// * `url` - Root site url w/ trailing slash (like https://safebooru.org)
/// * `tag` - Optional argument to get only posts from specified tags
/// * `pid` - Optional argument to get posts from specified page
/// * `proxy` - Optional proxy
///
/// # Examples
/// ```
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let posts = imgdl_rs::gelbooru::request::fetch_posts(
///         String::from("https://safebooru.org/"), Some(String::from("omori")), Some(2), None
///     ).await?; // Fetch all posts with tag `omori` from page 2
///
///     println!("{:?}", posts); 
///     Ok(())
/// }
/// ```
pub async fn fetch_posts(url: String, tags: Option<String>, pid: Option<i64>, proxy: Option<reqwest::Proxy>) -> Result<Vec<types::Post>, serde_json::Error> {
    let mut result_url: String = format!("{url}/{}&tags=", super::types::APIMethods::PostsList.as_str());
    if let Some(tags_unwrapped) = tags {
        result_url.push_str(&tags_unwrapped);
    }
    if let Some(pid_unwrapped) = pid {
        result_url.push_str(format!("&pid={}", pid_unwrapped).as_str());
    }
    let json: Value = request(result_url, proxy).await.unwrap().json().await.unwrap();
    let posts: Result<Vec<types::Post>, serde_json::Error> = serde_json::from_value(json);
    posts
}
