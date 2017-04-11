#![recursion_limit = "1024"]
//! # mastodon 
//! ## api for interacting with [mastodon](https://github.com/tootsuite/mastodon) instances
//!
//! this is very early in development, with only a few basic api endpoints implemented.
//!
//! you need to [generate an access token](https://github.com/tootsuite/mastodon/wiki/Testing-with-cURL) manually, for now
//!
//!
//! see mod `tests` in src/lib.rs for basic use examples. 


#[macro_use]
extern crate error_chain;


#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;
extern crate hyper;
extern crate hyper_native_tls;

pub mod entities;
pub mod mastodon;

pub mod errors {
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
    //use std::io::{self, Read};
    //use std::fs::File;
    //use serde_json;

    #[test]
    fn test_fetching_data() {
        let mastodon = Mastodon::from_access_token("ACCESS_TOKEN_HERE").unwrap();

        //let recent_statuses = mastodon.home_timeline().unwrap();
        //assert_eq!(20, recent_statuses.len());

        //let status = mastodon.get_status(StatusId(2186739)).unwrap();
        //assert_eq!(2186739, status.id.0);
        let status = mastodon.status_favourited_by(StatusId(2186739));
        println!("{:?}", status);
        //let status = mastodon.get_status_context(StatusId(2186739));
        //println!("{:?}", status.unwrap());
    }
}
