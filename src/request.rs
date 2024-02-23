extern crate reqwest;

use crate::types::Post;
use reqwest::Response;
use serde_json::Value;
use error_chain::error_chain;

use crate::auth::Auth;

/// Boosty API URL
pub const API_URL: &str = "https://api.boosty.to";

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

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
async fn request(method: String, auth: Option<Auth>) -> Result<Response> {
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
/// * `auth` - Optional argument for authorization in API
///
/// # Examples
/// ```
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let posts = boosty_rs::request::fetch_posts("boosty".to_string(), None).await?;
///     println!("{:?}", posts); 
///     Ok(())
/// }
/// ```
pub async fn fetch_posts(blog_name: String, auth: Option<Auth>) -> Result<Vec<Post>> {
    let url = format!("blog/{}/post/", blog_name);
    let json: Value = request(url.clone(), auth).await?.json().await?;
    let posts: Vec<Post> = serde_json::from_value(json["data"].clone()).unwrap();
    Ok(posts)
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
///     let post = boosty_rs::request::fetch_post("boosty".to_string(), "c9fb8a19-c45e-4602-9942-087c3af28c1b".to_string(), None).await?;
///     println!("{:?}", post); 
///     Ok(())
/// }
/// ```
pub async fn fetch_post(blog_name: String, post_id: String, auth: Option<Auth>) -> Result<Post> {
    let url = format!("blog/{}/post/{}", blog_name, post_id);
    let post: Post = request(url, auth).await?.json().await?;
    Ok(post)
}
