# boosty-rs
A Rust library for Boosty closed API

# Installation
```sh
$ cargo add boosty-rs
```

# Examples
### Fetch all posts from blog
```rust
let posts = boosty::request::fetch_posts("boosty").await?;
println!("{:?}", posts); 
```

### Fetch one post from blog
```rust
let post = boosty::request::fetch_post("boosty", "a4dc61c8-4ff9-495b-946b-3982efef68fe").await?;
println!("{:?}", post); 
```
