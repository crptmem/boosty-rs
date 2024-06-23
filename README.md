# boosty-rs
A Rust library for Boosty closed API
[![Rust](https://github.com/crptmem/boosty-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/crptmem/boosty-rs/actions/workflows/rust.yml)
# Installation
```sh
$ cargo add boosty-rs
```

# Examples
### Fetch all posts from blog
```rust
use std::error::Error;
use boosty_rs::request;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let response = request::fetch_posts("boosty".to_string(), None).await?;
    println!("{:?}", response); 
    Ok(())
}
```

### Fetch one post from blog
```rust
use std::error::Error;
use boosty_rs::request;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let response = request::fetch_post("boosty".to_string(), "a4dc61c8-4ff9-495b-946b-3982efef68fe".to_string(), None).await?;
    println!("{:?}", response); 
    Ok(())
}
```

### Fetch all posts from blog with authorization
```rust
use std::error::Error;
use boosty_rs::request;
use boosty_rs::auth::Auth;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let response = request::fetch_posts("boosty".to_string(), Auth::new("access_token".to_string())).await?;
    println!("{:?}", response); 
    Ok(())
}
```

