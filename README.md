# imgdl-rs
FORMER `boosty-rs`<br/>
A Rust library for downloading images from such services like Boosty and Gelbooru-powered sites<br/>
[![Rust](https://github.com/crptmem/imgdl-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/crptmem/imgdl-rs/actions/workflows/rust.yml)
# Installation
```sh
$ cargo add imgdl-rs
```

# Examples
## Boosty
### Fetch all posts from blog
```rust
use std::error::Error;
use boosty_rs::boosty::request;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let response = request::Client::fetch_posts("boosty".to_string(), None).await?;
    println!("{:?}", response); 
    Ok(())
}
```

### Fetch one post from blog
```rust
use std::error::Error;
use boosty_rs::boosty::request;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let response = request::Client::fetch_post("boosty".to_string(), "a4dc61c8-4ff9-495b-946b-3982efef68fe".to_string(), None).await?;
    println!("{:?}", response); 
    Ok(())
}
```

### Fetch all posts from blog with authorization
```rust
use std::error::Error;
use boosty_rs::boosty::request;
use boosty_rs::boosty::auth::Auth;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let response = request::Client::fetch_posts("boosty".to_string(), Auth::new("access_token".to_string())).await?;
    println!("{:?}", response); 
    Ok(())
}
```
## Gelbooru
### Fetch posts from tags
```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = imgdl_rs::gelbooru::request::Client::new(None);
    let posts = client.fetch_posts(
        "rating:general blue_sky", 3
    ).await?; // Fetch all posts with tags `rating:general blue_sky` from page 3
    println!("{:?}", posts); 
    Ok(())
}
```
