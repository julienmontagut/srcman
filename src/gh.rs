use reqwest::blocking::Client;
use reqwest::{header::USER_AGENT, header::ACCEPT, Error};

// The github module where the GitHub Client resides
pub struct GitHubClient {
    base_url: String,
    client: Client,
    token: String,
}

impl GitHubClient {
    pub fn new(token: &str) -> Self {
        GitHubClient {
            base_url: "https://api.github.com".to_string(),
            client: Client::new(),
            token: token.to_string(),
        }
    }

    pub fn get(&self, path: &str) -> Result<String, Error> {
        let url = format!("{}/{}", self.base_url, path);
        let request = self.client.get(&url)
            .bearer_auth(&self.token)
            .header(USER_AGENT, env!("CARGO_PKG_NAME"))
            .header(ACCEPT, "application/vnd.github+json")
            .header("X-GitHub-Api-Version", "2022-11-28");
        let response = request.send()?;
        Ok(response.text()?)
    }

    pub fn get_user(&self) -> Result<String, reqwest::Error> {
        self.get("user")
    }

    pub fn get_user_repos(&self) -> Result<String, reqwest::Error> {
        self.get("user/repos")
    }

    pub fn get_user_orgs(&self) -> Result<String, reqwest::Error> {
        self.get("user/orgs")
    }

    pub fn get_user_starred(&self) -> Result<String, reqwest::Error> {
        self.get("user/starred")
    }

    pub fn get_user_subscriptions(&self) -> Result<String, reqwest::Error> {
        self.get("user/subscriptions")
    }

    pub fn get_octocat(&self) -> Result<String, reqwest::Error> {
        self.get("octocat")
    }
}
