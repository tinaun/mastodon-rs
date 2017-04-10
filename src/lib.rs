#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;


#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;
extern crate hyper;

pub mod entities;
pub mod mastodon;

/// # mastodon - api for interacting with [mastodon](https://github.com/Gargron/mastodon) instances
///
/// this is very early in development, with only a few basic api endpoints implemented
/// you need to [generate an access token](https://github.com/Gargron/mastodon/wiki/Testing-with-cURL) manually, for now
///
///
/// see mod `tests` in src/lib.rs for basic use examples 

mod errors {
    use hyper;
    use serde_json;
    use std::io;

    error_chain! {
        foreign_links {
            Server(super::entities::ServerError);
            Hyper(hyper::Error);
            Serde(serde_json::Error);
            IO(io::Error);
        }
    }
}


#[cfg(test)]
mod tests {
    use super::mastodon::*;
    use super::entities::*;
    use std::io::{self, Read};
    use std::fs::File;
    use serde_json;

    #[test]
    fn test_fetching_data() {
        let mastodon = Mastodon::from_access_token("ACCESS_TOKEN_HERE").unwrap();

        let recent_statuses = mastodon.home_timeline().unwrap();

        println!("{:#?}", recent_statuses[0]);
    }
}
