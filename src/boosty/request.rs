extern crate reqwest;

use crate::boosty::types::Post;
use reqwest::Response;
use serde_json::Value;

use crate::boosty::auth::Auth;

/// Boosty API URL
pub const API_URL: &str = "https://api.boosty.to";

/// Sends a request to Boosty API, returns reqwest Response wrapped in Result
///
/// # Arguments
///
/// * `method` - A string that holds API method, for example blog/blog_name/post
///
/// # Examples
///
/// ```ignore
/// let body = request("blog/boosty/post").await?;
/// let text = body.text().await?;
/// println!("{:?}", text);
/// ```
async fn request(method: String, auth: Option<Auth>) -> Result<Response, reqwest::Error> {
    let url = format!("{}/v1/{}", API_URL, method);  // Will result in something like
                                                     // https://api.boosty.to/v1/blog/boosty/post/
                                                     // Trailing slash is required only for
                                                     // fetching all posts, otherwise 404
                                                     // will be returned.
    let client = reqwest::Client::new();
    if auth.is_some() {
        Ok(
            client
            .get(url)
            .headers(auth.unwrap().headers)
            .send()
            .await?)
    } else {
        Ok(
            client
            .get(url)
            .send()
            .await?)
    }
}

/// Fetches all posts from blog, retuns a vector of Post wrapped in Result
///
/// # Arguments
///
/// * `blog_name` - Name of a blog to get posts
/// * `limit` - Count of posts to return (Boosty default when no provided in request is 100)
/// * `auth` - Optional argument for authorization in API
///
/// # Examples
/// ```
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let posts = imgdl_rs::boosty::request::fetch_posts("crptmem".to_string(), 300, None).await?;
///     println!("{:?}", posts); 
///     Ok(())
/// }
/// ```
pub async fn fetch_posts(blog_name: String, limit: i64, auth: Option<Auth>) -> Result<Vec<Post>, serde_json::Error> {
    let url = format!("blog/{}/post/?limit={}", blog_name, limit);
    let json: Value = request(url.clone(), auth).await.unwrap().json().await.unwrap();
    let posts: Result<Vec<Post>, serde_json::Error> = serde_json::from_value(json["data"].clone());
    posts
}

/// Fetches all posts from blog but do not parse, returns a serde Value
///
/// # Arguments
///
/// * `blog_name` - Name of a blog to get posts
/// * `limit` - Count of posts to return (Boosty default when no provided in request is 100)
/// * `auth` - Optional argument for authorization in API
///
/// # Examples
/// ```
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let posts = imgdl_rs::boosty::request::fetch_posts_raw("crptmem".to_string(), 300, None).await?;
///     println!("{:?}", posts); 
///     Ok(())
/// }
/// ```
pub async fn fetch_posts_raw(blog_name: String, limit: i64, auth: Option<Auth>) -> Result<Value, serde_json::Error> {
    let url = format!("blog/{}/post/?limit={}", blog_name, limit);
    let json: Value = request(url.clone(), auth).await.unwrap().json().await.unwrap();
    Ok(json)
}

/// Fetch a certain post from blog, retuns Post wrapped in Result
///
/// # Arguments
///
/// * `blog_name` - Name of a blog to get posts
/// * `post_id` - ID of a post in blog
///
/// # Examples
/// ```
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let post = imgdl_rs::boosty::request::fetch_post("boosty".to_string(), "c9fb8a19-c45e-4602-9942-087c3af28c1b".to_string(), None).await?;
///     println!("{:?}", post); 
///     Ok(())
/// }
/// ```
pub async fn fetch_post(blog_name: String, post_id: String, auth: Option<Auth>) -> Result<Post, serde_json::Error> {
    let url = format!("blog/{}/post/{}", blog_name, post_id);
    let post: Post = request(url, auth).await.unwrap().json().await.unwrap();
    Ok(post)
}
