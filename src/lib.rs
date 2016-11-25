#![feature(proc_macro)]

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

#[cfg(test)]
mod tests {
    use super::mastodon::*;
    use super::entities::*;
    use std::io::{self, Read};
    use std::fs::File;
    use serde_json;

    #[test]
    fn test_serde() {
        let f = File::open("output.json").unwrap();
        let value: Vec<Status> = serde_json::from_reader(f).unwrap();

        println!("{:#?}", value[0]);
        
    }

    #[test]
    fn test_fetching_data() {
        let mastodon = Mastodon::from_access_token("ACCESS_TOKEN_HERE").unwrap();

        let recent_statuses = mastodon.home_timeline().unwrap();

        println!("{:#?}", recent_statuses[0]);
    }
}
