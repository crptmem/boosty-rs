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
        let response = request::fetch_post("boosty".to_string(), "f3f8055a-e7ff-4599-b62e-304c8b4b936e".to_string(), None).await?;
        assert_eq!(response.title, "Boosty для иллюстраторов");
        println!("{:#?}", response); 
        Ok(())
    }

    /// Test that two posts titles is equal to hard-coded here and test is Boosty API still the
    /// same
    #[tokio::test]
    async fn fetch_posts_test() -> Result<(), Box<dyn std::error::Error>> {
        let response = request::fetch_posts("crptmem".to_string(), None).await?;
        let titles = vec!["boosty-rs unit test fetch_posts 2", "boosty-rs unit test fetch_post"];
        for (i, element) in response.iter().enumerate() {
            assert_eq!(element.title, titles[i]);
        }
        Ok(())
    }
}
