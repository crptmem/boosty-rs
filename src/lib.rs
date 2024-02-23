pub mod request;

#[cfg(test)]
mod tests {
    use crate::request;

    #[tokio::test]
    async fn fetch_post() -> Result<(), Box<dyn std::error::Error>> {
        let response = request::fetch_post("crptmem".to_string(), "64c7b376-5825-42ae-90d1-8c28fac5f6ab".to_string()).await?;
        assert_eq!(response.title, "boosty-rs unit test fetch_post"); 
        Ok(())
    }

    #[tokio::test]
    async fn fetch_posts() -> Result<(), Box<dyn std::error::Error>> {
        let response = request::fetch_posts("crptmem".to_string()).await?;
        let titles = vec!["boosty-rs unit test fetch_posts 2", "boosty-rs unit test fetch_post"];
        for (i, element) in response.iter().enumerate() {
            println!("{}: {:?}", i, element);
            assert_eq!(element.title, titles[i]);
        }
        Ok(())
    }
}
