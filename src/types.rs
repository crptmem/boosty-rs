use serde::{Serialize, Deserialize};

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
/// Paid post teaser
pub struct Teaser {
    #[serde(rename = "type")]
    /// Type of teaser
    pub content_type: String,
    /// Width of content
    pub width: Option<isize>,
    /// Height of content
    pub height: Option<isize>,
    /// Rendition of content
    pub rendition: Option<String>,
    /// URL of content
    pub url: Option<String>,
    /// Teaser ID
    pub id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
/// Attached content to post
pub struct Data {
    #[serde(rename = "type")]
    /// Type of teaser
    pub content_type: String,
    /// Width of content
    pub width: Option<isize>,
    /// Height of content
    pub height: Option<isize>,
    /// Rendition of content
    pub rendition: Option<String>,
    /// URL of content
    pub url: Option<String>,
    /// Teaser ID
    pub id: Option<String>,
    // Content itself (for example text)
    pub content: Option<String>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
/// Post reactions count
pub struct Reactions {
    /// Angry reactions count
    pub angry: isize,
    /// Heart (likes) reaction count
    pub heart: isize,
    /// Fire reaction count
    pub fire: isize,
    /// Like reaction count (not the same as heart)
    pub like: isize,
    /// Dislike reaction count
    pub dislike: isize,
    /// Wonder reaction count
    pub wonder: isize,
    /// Laught reaction count
    pub laught: isize,
    /// Sad reaction count,
    pub sad: isize
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
/// Post reactions and comments count
pub struct Count {
    /// Likes count
    pub likes: isize,
    /// Reactions count
    pub reactions: Reactions,
    /// Comments count
    pub comments: isize
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
    /// Reactions and comments count
    pub count: Count,
    /// Paid post data
    pub data: Option<Vec<Data>>,
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
    /// Donations count
    pub donations: isize,
    /// Post title
    pub title: String
}
