use reqwest::header::{HeaderMap, AUTHORIZATION};

/// Authorization data struct
#[derive(Debug, Clone)]
pub struct Auth {
    /// Request headers with all required data to auth in Boosty API
    pub headers: HeaderMap,
}

/// Implementation of Auth struct
impl Auth {
    /// Returns a new, mutable Auth struct
    pub fn new(access_token: String) -> Auth {
        let mut auth = Auth {
            headers: HeaderMap::new(),
        };
        auth.headers.insert(AUTHORIZATION, format!("Bearer {}", access_token.clone()).parse().unwrap());
        auth
    }
}
