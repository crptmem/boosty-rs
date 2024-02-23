extern crate reqwest;

use reqwest::Response;
use serde_json::Value;
use serde::{Deserialize, Serialize};
use error_chain::error_chain;

/// Boosty API URL
pub const API_URL: &str = "https://api.boosty.to";

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "UPPERCASE")]
/// Prices of a paid post in two currencies
pub struct CurrencyPrices {
    /// Price in russian rubles
    pub rub: f64,
    /// Price in american dollars
    pub usd: f64
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Teaser {
    #[serde(rename = "type")]
    /// Type of teaser
    pub ctype: String,
    /// Width of content
    pub width: Option<isize>,
    /// Height of content
    pub height: Option<isize>,
    /// Rendition of content
    pub rendition: Option<String>,
    /// URL of content
    pub url: Option<String>,
    /// Teaser ID
    pub id: Option<String>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
/// Boosty post on a blog
pub struct Post {
    /// Creation time in Unix format
    pub created_at: u64,
    /// Update time in Unix format
    pub updated_at: Option<u64>,
    /// Publish time in Unix format
    pub publish_time: u64,

    /// Paid post price in two currencies
    pub currency_prices: CurrencyPrices,
    /// Teaser of paid post
    pub teaser: Vec<Teaser>,
    /// Is post views counter visible
    pub show_views_counter: bool,
    /// Price of a paid post (0 if free)
    pub price: isize,
    /// Post ID
    pub id: String,
    /// Post title
    pub title: String
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
async fn request(method: String) -> Result<Response> {
    let url = format!("{}/v1/{}", API_URL, method);  // Will result in something like
                                                     // https://api.boosty.to/v1/blog/boosty/post/
                                                     // Trailing slash is required only for
                                                     // fetching all posts, otherwise 404
                                                     // will be returned.
    Ok(reqwest::get(url).await?)
}

/// Fetches all posts from blog, retuns a vector of Post wrapped in Result
///
/// # Arguments
///
/// * `blog_name` - Name of a blog to get posts
///
/// # Examples
/// ```
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let posts = boosty_rs::request::fetch_posts("boosty".to_string()).await?;
///     println!("{:?}", posts); 
///     Ok(())
/// }
/// ```
pub async fn fetch_posts(blog_name: String) -> Result<Vec<Post>> {
    let url = format!("blog/{}/post/", blog_name);
    let json: Value = request(url.clone()).await?.json().await?;
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
///     let post = boosty_rs::request::fetch_post("boosty".to_string(), "c9fb8a19-c45e-4602-9942-087c3af28c1b".to_string()).await?;
///     println!("{:?}", post); 
///     Ok(())
/// }
/// ```
pub async fn fetch_post(blog_name: String, post_id: String) -> Result<Post> {
    let url = format!("blog/{}/post/{}", blog_name, post_id);
    let post: Post = request(url).await?.json().await?;
    Ok(post)
}
