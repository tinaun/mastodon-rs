use std::env;

use hyper::{self, Client};
use hyper::client::response::Response;
use hyper::header::{Authorization, Bearer};

use serde_json;

use entities::*;

pub struct Mastodon {
    access_token: String,
    domain: String,
    client: Client,
}

impl Mastodon {
    /// create mastodon instance from access_token (as environment variable)
    pub fn from_access_token(envar: &str) -> Option<Self> {
        let token = env::var(envar).ok();

        token.map(|access| {
            Mastodon {
                access_token: access,
                domain: "mastodon.social".to_string(),
                client: Client::new()
            }
        })
    }

    fn _get_request(&self, endpoint: String, queryparams: Option<String>) -> hyper::Result<Response> {
        self.client.get(&format!("https://{}/api/v1{}", &self.domain, endpoint))
            .header( Authorization(
                        Bearer {
                            token: self.access_token.clone(),
                        }
            )).body(&queryparams.unwrap_or(String::new()))
            .send()
    }

    fn _post_request(&self, endpoint: String, queryparams: Option<String>) -> hyper::Result<Response> {
        self.client.post(&format!("https://{}/api/v1{}", &self.domain, endpoint))
            .header( Authorization(
                        Bearer {
                            token: self.access_token.clone(),
                        }
            )).body(&queryparams.unwrap_or(String::new()))
            .send()
    }

    /// get status object with id `id`
    pub fn get_status(&self, id: StatusId) -> Option<Status> {
        self._get_request(format!("/statuses/{}", id), None).ok().and_then(|r| {
            serde_json::from_reader(r).ok()
        })
    }

    /// get user object with id `id`
    pub fn get_account(&self, id: UserId) -> Option<Account> {
        self._get_request(format!("/accounts/{}", id), None).ok().and_then(|r| {
            serde_json::from_reader(r).ok()
        })
    }

    /// get latest 20 statuses for user `id`
    pub fn get_user_statuses(&self, id: UserId) -> Option<Vec<Status>> {
        self._get_request(format!("/accounts/{}/statuses", id), None).ok().and_then(|r| {
            serde_json::from_reader(r).ok()
        })
    }
    /// get users `id` is following
    pub fn get_following(&self, id: UserId) -> Option<Vec<Account>> {
        self._get_request(format!("/accounts/{}/following", id), None).ok().and_then(|r| {
            serde_json::from_reader(r).ok()
        })
    }
    /// get latest 20 statuses for `id`
    pub fn get_followers(&self, id: UserId) -> Option<Vec<Account>> {
        self._get_request(format!("/accounts/{}/followers", id), None).ok().and_then(|r| {
            serde_json::from_reader(r).ok()
        })
    }

    /// get latest statuses in home timeline
    pub fn home_timeline(&self) -> Option<Vec<Status>> {
        self._get_request(format!("/timelines/home"), None).ok().and_then(|r| {
            serde_json::from_reader(r).ok()
        })
    }

    /// get latest statuses in public timeline
    pub fn public_timeline(&self) -> Option<Vec<Status>> {
        self._get_request(format!("/timelines/public"), None).ok().and_then(|r| {
            serde_json::from_reader(r).ok()
        })
    }

    /// get latest 20 statuses for `id`
    pub fn mentions(&self) -> Option<Vec<Status>> {
        self._get_request(format!("/timelines/mentions"), None).ok().and_then(|r| {
            serde_json::from_reader(r).ok()
        })
    }

}
