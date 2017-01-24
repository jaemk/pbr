#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate json;
extern crate reqwest;
extern crate time;

pub mod errors { error_chain! { } }

use std::io::Read;
use reqwest::header::{Authorization, Bearer};
use errors::*;


static URL: &'static str = "https://api.pushbullet.com/v2/";


/// Push key, val pair onto the given url
fn push_pair(url: &mut String, pair: &(&str, Option<String>)) {
    url.push_str(pair.0);
    url.push('=');
    match pair.1 {
        Some(ref v) => url.push_str(v),
        _ => unreachable!(),
    }
}


/// Build our url & querystring
fn build_url(base: &str, endpoint: &str, params: &[(&str, Option<String>)]) -> String {
    let mut url = base.to_string();
    url.push_str(endpoint);
    url.push('?');
    let mut and = false;
    for pair in params.iter() {
        if pair.1.is_some() {
            if and {
                url.push('&');
            }
            push_pair(&mut url, pair);
            and = true;
        }
    }
    url
}


/// Return the current time in seconds
pub fn now_sec() -> u64 {
    let now = time::get_time();
    now.sec as u64
}


/// General Client
pub struct Client {
    token: String,
}

impl Client {
    /// Create a new pbr client
    pub fn new(token: &str) -> Client {
        Client { token: token.into() }
    }

    /// Return json::JsonValue containing account info
    pub fn whoami(&self) -> Result<json::JsonValue> {
        let mut url = String::from(URL);
        url.push_str("users/me");
        let client = reqwest::Client::new()
                         .chain_err(|| "failed to build request client")?;
        let mut resp = client.get(&url)
                         .header(Authorization(Bearer{token: self.token.to_string()}))
                         .send().chain_err(|| "request error")?;
        let mut content = String::new();
        let _ = resp.read_to_string(&mut content).chain_err(|| "error reading to string")?;
        Ok(json::parse(&content).chain_err(|| "parse error")?)
    }

    /// Create a new pushes request
    pub fn pushes(&self) -> Pushes {
        Pushes::new(&self.token)
    }
}


/// Params for a pushes request
pub struct Pushes {
    token: String,
    limit: Option<String>,
    modified_after: Option<String>,
}

impl Pushes {
    pub fn new(token: &str) -> Pushes {
        Pushes {
            token: token.to_string(),
            modified_after: None,
            limit: None,
        }
    }

    /// set modified_after param
    pub fn modified_after(mut self, seconds: u64) -> Pushes {
        self.modified_after = Some(seconds.to_string());
        self
    }

    /// set limit param
    pub fn limit(mut self, limit: u32) -> Pushes {
        self.limit = Some(limit.to_string());
        self
    }

    /// send request and return deserialized response
    pub fn send(self) -> Result<json::JsonValue> {
        let params = vec![
            ("limit", self.limit),
            ("modified_after", self.modified_after),
        ];
        let url = build_url(URL, "pushes", &params);
        let client= reqwest::Client::new()
                            .chain_err(|| "failed to build request client")?;
        let mut resp = client.get(&url)
                             .header(Authorization(Bearer{token: self.token}))
                             .send().chain_err(|| "request error")?;
        let mut content = String::new();
        let _ = resp.read_to_string(&mut content).chain_err(|| "error reading to string")?;
        Ok(json::parse(&content).chain_err(|| "parse error")?)
    }
}
