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

#[derive(Copy, Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct NotificationId(u64);

impl fmt::Display for NotificationId { 
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
    in_reply_to_account_id: Option<UserId>,
    reblog: Option<Box<Status>>,
    content: String,
    created_at: String,
    reblogs_count: u64,
    favourites_count: u64,
    reblogged: bool,
    favourited: bool,
    sensitive: bool,
    spoiler_text: bool,
    visibility: Visibility, 
    media_attachments: Vec<MediaAttachment>,
    mentions: Vec<Mention>,
    tags: Vec<Tag>,
    application: Application
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub enum Visibility {
    Public,
    Unlisted,
    Private,
    Direct,
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
    },
    GifV {
        url: String,
        preview_url: String,
    }
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct Tag {
    name: String,
    url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Mention {
    pub id: UserId,
    url: String,
    username: String,
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
    locked: bool,
    created_at: String,
    followers_count: u64,
    following_count: u64,
    statuses_count: u64,
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct Application {
    name: String,
    website: String,
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct Instance {
    uri: String,
    title: String,
    description: String,
    email: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct RawNotification {
    #[serde(rename = "type")]
    tag: String,
    id: NotificationId,
    created_at: String,
    account: Account,
    status: Option<Status>,
}

#[derive(Debug)]
pub enum Notification {
    Mention {
        id: NotificationId,
        created_at: String,
        account: Account,
    },
    Follow {
        id: NotificationId,
        created_at: String,
        account: Account,
    },
    Favourite {
        id: NotificationId,
        created_at: String,
        account: Account,
        status: Status,
    },
    Reblog {
        id: NotificationId,
        created_at: String,
        account: Account,
        status: Status,
    },
    Unknown {
        id: NotificationId,
        created_at: String,
    }
}

impl From<RawNotification> for Notification {
    fn from(rn: RawNotification) -> Self {
        match rn.tag.as_str() {
            "mention" => Notification::Mention {
                id: rn.id,
                created_at: rn.created_at,
                account: rn.account,
            },
            "follow" => Notification::Follow {
                id: rn.id,
                created_at: rn.created_at,
                account: rn.account,
            },
            "favourite" => Notification::Favourite {
                id: rn.id,
                created_at: rn.created_at,
                account: rn.account,
                status: rn.status.unwrap(),
            },
            "reblog" => Notification::Reblog {
                id: rn.id,
                created_at: rn.created_at,
                account: rn.account,
                status: rn.status.unwrap(),
            },
            _ => Notification::Unknown {
                id: rn.id,
                created_at: rn.created_at,
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
/// a server sent error
/// in most cases these will be translated into results
pub struct ServerError {
    error: String
}