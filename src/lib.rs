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
    use crate::request;

    /// Test that post title is equal to hard-coded here and test is Boosty API still the same
    #[tokio::test]
    async fn fetch_post_test() -> Result<(), Box<dyn std::error::Error>> {
        let response = request::fetch_post("crptmem".to_string(), "64c7b376-5825-42ae-90d1-8c28fac5f6ab".to_string(), None).await?;
        assert_eq!(response.title, "boosty-rs unit test fetch_post");
        Ok(())
    }

    /// Test that another post title is equal to hard-coded here and test is Boosty API still the same
    #[tokio::test]
    async fn fetch_post_second_test() -> Result<(), Box<dyn std::error::Error>> {
        let response = request::fetch_post("boosty".to_string(), "8c2ba2c5-da5c-4a64-94c4-4fef6147333a".to_string(), None).await?;
        assert_eq!(response.title, "Boosty.to VS Patreon");
        Ok(())
    }

    #[tokio::test]
    async fn fetch_posts() -> Result<(), Box<dyn std::error::Error>> {
        let response = request::fetch_posts("crptmem".to_string(), None).await?;
        println!("{:#?}", response); 
        Ok(())
    }
}
