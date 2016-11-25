/// entiti:es.rs
/// data structures for the mastodon api
///
use std::fmt;


#[derive(Copy, Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct StatusId(u64);

impl fmt::Display for StatusId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Copy, Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct UserId(u64);

impl fmt::Display for UserId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Status {
    pub id: StatusId,
    uri: String,
    url: String,
    account: Account,
    in_reply_to_id: Option<StatusId>,
    reblog: Option<Box<Status>>,
    content: String,
    created_at: String,
    reblogs_count: u64,
    favourites_count: u64,
    reblogged: bool,
    favourited: bool,
    media_attachments: Vec<MediaAttachment>,
    mentions: Vec<Mention>
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub enum MediaAttachment {
    Image {
        url: String,
        preview_url: String,
    },
    Video {
        url: String,
        preview_url: String,
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Mention {
    pub id: UserId,
    url: String,
    acct: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Account {
    pub id: UserId,
    username: String,
    acct: String,
    display_name: String,
    note: String,
    url: String,
    avatar: String,
    header: String,
    followers_count: u64,
    following_count: u64,
    statuses_count: u64,
}





