use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Rating {
    Explicit,
    General,
    Sensitive,
    Questionable
}

#[derive(Serialize, Deserialize, Debug, Clone)]
/// Gelbooru post
pub struct Post {
    /// Post ID
    pub id: i64,
    /// When post was created
    pub created_at: String,
    /// Post score
    pub score: i64,
    /// Image width
    pub width: i64,
    /// Image height
    pub height: i64,
    /// MD5 image hash
    pub md5: String,
    /// Post directory
    pub directory: String,
    /// Image file name
    pub image: String,
    /// Image rating
    pub rating: Rating,
    /// Source URL
    pub source: String,
    /// When post was changed
    pub change: Option<i64>,
    /// Post owner
    pub owner: String,
    /// Creator ID
    pub creator_id: i64,
    /// Post parent ID (0 if no parent)
    pub parent_id: i64,
    /// Does this post have a sample
    pub sample: i64,
    /// Preview image height
    pub preview_height: i64,
    /// Preview image width
    pub preview_width: i64,
    /// Post tags
    pub tags: String,
    /// Post title
    pub title: String,
    /// File URL (full res)
    pub file_url: String,
    /// Preview URL
    pub preview_url: String,
    /// Sample URL
    pub sample_url: String,
    /// Sample height
    pub sample_height: i64,
    /// Sample width
    pub sample_width: i64,
    /// Status
    pub status: String,
    /// Is post locked
    pub post_locked: i64,
    /// Does this post have child posts
    pub has_children: String
}

pub enum APIMethods {
    PostsList
}

impl APIMethods {
    pub fn as_str(&self) -> &'static str {
        match self {
            APIMethods::PostsList => "/index.php?page=dapi&s=post&q=index&json=1",
        }
    }
}
