use std::env;
use std::io::Read;

use hyper::{self, Client};
use hyper::client::response::Response;
use hyper::header::{Authorization, Bearer};

use serde_json;

use entities::*;
use errors::*;

macro_rules! parse_response {
    ($reader:ident) => { {
        let mut r = $reader;
        let mut s = String::new();
        r.read_to_string(&mut s)?;

        let se: Result<ServerError> = serde_json::from_str(&s).chain_err(|| "invalid json");
        match se {
            Err(_) => serde_json::from_str(&s).chain_err(|| "invalid json"),
            Ok(e) => Err(ErrorKind::Server(e).into()),
        }
    } }
}

pub struct Mastodon {
    access_token: String,
    domain: String,
    client: Client,
}

impl Mastodon {
    /// create mastodon instance from access_token (as environment variable)
    pub fn from_access_token(envar: &str) -> Result<Self> {
        let token = env::var(envar).chain_err(|| "missing environment variable");

        token.map(|access| {
            Mastodon {
                access_token: access,
                domain: "mastodon.social".to_string(),
                client: Client::new()
            }
        })
    }

    fn _get_request(&self, endpoint: String, queryparams: Option<String>) -> Result<Response> {
        self.client.get(&format!("https://{}/api/v1{}", &self.domain, endpoint))
            .header( Authorization(
                        Bearer {
                            token: self.access_token.clone(),
                        }
            )).body(&queryparams.unwrap_or(String::new()))
            .send().chain_err(|| "unable to send get request")
    }

    fn _post_request(&self, endpoint: String, queryparams: Option<String>) -> Result<Response> {
        self.client.post(&format!("https://{}/api/v1{}", &self.domain, endpoint))
            .header( Authorization(
                        Bearer {
                            token: self.access_token.clone(),
                        }
            )).body(&queryparams.unwrap_or(String::new()))
            .send().chain_err(|| "unable to send post request")
    }

    /// get status object with id `id`
    pub fn get_status(&self, id: StatusId) -> Result<Status> {
        self._get_request(format!("/statuses/{}", id), None).and_then(|r| {
            parse_response!(r)
        })
    }

    /// get user object with id `id`
    pub fn get_account(&self, id: UserId) -> Result<Account> {
        self._get_request(format!("/accounts/{}", id), None).and_then(|r| {
            parse_response!(r)
        })
    }

    /// get latest 20 statuses for user `id`
    pub fn get_user_statuses(&self, id: UserId) -> Result<Vec<Status>> {
        self._get_request(format!("/accounts/{}/statuses", id), None).and_then(|r| {
            parse_response!(r)
        })
    }
    /// get users `id` is following
    pub fn get_following(&self, id: UserId) -> Result<Vec<Account>> {
        self._get_request(format!("/accounts/{}/following", id), None).and_then(|r| {
            parse_response!(r)
        })
    }
    /// get latest 20 statuses for `id`
    pub fn get_followers(&self, id: UserId) -> Result<Vec<Account>> {
        self._get_request(format!("/accounts/{}/followers", id), None).and_then(|r| {
            parse_response!(r)
        })
    }

    /// get latest statuses in home timeline
    pub fn home_timeline(&self) -> Result<Vec<Status>> {
        self._get_request(format!("/timelines/home"), None).and_then(|r| {
            parse_response!(r)
        })
    }

    /// get latest statuses in public timeline
    pub fn public_timeline(&self) -> Result<Vec<Status>> {
        self._get_request(format!("/timelines/public"), None).and_then(|r| {
            parse_response!(r)
        })
    }

    /// get latest 20 statuses for `id`
    pub fn mentions(&self) -> Result<Vec<Status>> {
        self._get_request(format!("/timelines/mentions"), None).and_then(|r| {
            parse_response!(r)
        })
    }

}
