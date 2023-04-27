pub struct Client {
    base_url: String,
    client: HttpClient,
    token: String,
}

impl Client {
    fn new(token: &str) {
        Client {
            base_url: "https://gitlab.com/api/v4".to_string(),
            client: HttpClient::new(),
            token: token.to_string(),
        }
    }
}
