//! This crate provides asynchronous functions to access Boosty not documented, but accessible API

/// API calls
pub mod request;

/// API authorization
pub mod auth;

/// API types
pub mod types;

/// Tests
#[cfg(test)]
mod tests {
    use crate::{auth::Auth, request};

    /// Test that post title is equal to hard-coded here
    #[tokio::test]
    async fn fetch_post() -> Result<(), Box<dyn std::error::Error>> {
        let response = request::fetch_post("crptmem".to_string(), "64c7b376-5825-42ae-90d1-8c28fac5f6ab".to_string(), None).await?;
        assert_eq!(response.title, "boosty-rs unit test fetch_post"); 
        Ok(())
    }

    /// Test that two posts titles is equal to hard-coded here
    #[tokio::test]
    async fn fetch_posts() -> Result<(), Box<dyn std::error::Error>> {
        let response = request::fetch_posts("crptmem".to_string(), None).await?;
        let titles = vec!["boosty-rs unit test fetch_posts 2", "boosty-rs unit test fetch_post"];
        for (i, element) in response.iter().enumerate() {
            assert_eq!(element.title, titles[i]);
        }
        Ok(())
    }
}
