//! # Github API SDK

use std::collections::HashMap;

use crate::error::{Perror, Presult};
use json::JsonValue;
use reqwest::{blocking, Method};

pub struct Github<'a> {
    repositroy: &'a str,
    token: &'a str,
}

impl<'a> Github<'a> {
    pub fn new(repositroy: &'a str, token: &'a str) -> Github<'a> {
        Github { repositroy, token }
    }

    /// # Build reqwest client with gihub common configure
    /// [github doc](https://docs.github.com/cn/rest/git/)
    pub fn client(
        &self,
        method: Method,
        url: &str,
        body: Option<HashMap<&str, &str>>,
    ) -> Presult<JsonValue> {
        //dotenv()?;

        let client_inner = blocking::Client::builder().build()?;
        let mut auth = String::from("token ");
        auth.push_str(self.token);

        let mut full_url = String::from("https://api.github.com/repos/");
        full_url.push_str(self.repositroy);
        full_url.push('/');
        full_url.push_str(url);

        let mut request = client_inner
            .request(method, full_url)
            .header("Authorization", auth)
            .header("User-Agent", "tu6ge(772364230@qq.com)")
            .header("Accept", "application/vnd.github.v3+json");

        if let Some(body) = body {
            request = request.json(&body)
        }

        let response = request.send()?;

        if response.status() != 200 && response.status() != 201 && response.status() != 204 {
            return Err(Perror::Github(response.text()?));
        }

        if response.status() == 204 {
            return Ok(JsonValue::new_object());
        }

        let result = json::parse(&response.text()?)?;
        Ok(result)
    }

    /// # Get git sha of git head
    pub fn get_sha(&self, head: &str) -> Presult<String> {
        let url = String::from("git/matching-refs/heads/") + head;
        let json = self.client(Method::GET, &url, None)?;
        let sha: String = json[0]["object"]["sha"].to_string();
        Ok(sha)
    }

    /// # Set tag ref by git sha
    pub fn set_ref(&self, tag: &str, sha: &str) -> Presult<()> {
        let url = "git/refs";
        let mut body = HashMap::new();

        let mut tag_string = String::from("refs/tags/");
        tag_string.push_str(tag);

        body.insert("ref", tag_string.as_str());
        body.insert("sha", sha);

        self.client(Method::POST, url, Some(body))?;
        Ok(())
    }

    #[allow(dead_code)]
    /// # delete git ref
    pub fn del_ref(&self) -> Presult<()> {
        let url = "git/refs/tags/dev-0.2.0";

        self.client(Method::DELETE, url, None)?;
        Ok(())
    }
}
