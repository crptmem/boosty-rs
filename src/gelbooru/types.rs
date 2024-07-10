use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
/// Gelbooru post
pub struct Post {
    /// Directory where source image is stored
    pub directory: String,
    /// Image hash
    pub hash: String,
    /// Image height
    pub height: i64,
    /// Post ID
    pub id: i64,
    /// Image file ID
    pub image: String,
    /// Last change (UNIX timestamp)
    pub change: Option<i64>,
    /// Post owner
    pub owner: String,
    /// Parent ID
    pub parent_id: Option<i64>,
    /// Rating (like nsfw/sfw)
    pub rating: String,
    /// Sample image present (thumbnail)
    pub sample: bool,
    /// Sample image height
    pub sample_height: Option<i64>,
    /// Sample image width
    pub sample_width: Option<i64>,
    /// Score
    pub score: Option<i64>,
    /// All post tags
    pub tags: String,
    /// Image width
    pub width: i64
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
