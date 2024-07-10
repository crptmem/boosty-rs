//! This crate provides asynchronous functions to access Boosty not documented, but accessible API

/// Boosty API module
pub mod boosty;

/// Gelbooru API module
pub mod gelbooru;

/// Boosty tests
#[cfg(test)]
mod boosty_tests {
    use crate::boosty::request;

    /// Test that post title is equal to hard-coded here and test is Boosty API still the same
    #[tokio::test]
    async fn boosty_fetch_post() -> Result<(), Box<dyn std::error::Error>> {
        let response = request::fetch_post("crptmem".to_string(), "64c7b376-5825-42ae-90d1-8c28fac5f6ab".to_string(), None).await?;
        assert_eq!(response.title, "boosty-rs unit test fetch_post");
        Ok(())
    }

    /// Test that another post title is equal to hard-coded here and test is Boosty API still the same
    #[tokio::test]
    async fn boosty_fetch_post_second() -> Result<(), Box<dyn std::error::Error>> {
        let response = request::fetch_post("boosty".to_string(), "8c2ba2c5-da5c-4a64-94c4-4fef6147333a".to_string(), None).await?;
        assert_eq!(response.title, "Boosty.to VS Patreon");
        Ok(())
    }
}
