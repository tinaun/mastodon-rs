//! data structures for the mastodon api
//!
//! 
use std::fmt;
use std::error;

#[doc(hidden)]
mod nullbool {
    /// sometimes an instance returns null instead of false for boolean values
    /// which breaks the default deserializer
    use serde::Deserializer;
    use serde::de::Visitor;
    use std::fmt;
    use std::error::Error;

    struct NullBoolVisitor {}

    impl Visitor for NullBoolVisitor {
        type Value = bool;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("true or null")
        }

        fn visit_bool<E>(self, visitor: bool) -> Result<Self::Value, E>
            where E: Error 
        {
            Ok(visitor)
        }

        fn visit_unit<E>(self) -> Result<Self::Value, E>
            where E: Error
        {
            Ok(false)
        }
    }

    pub fn de<D>(deserializer: D) -> Result<bool, D::Error>
        where D: Deserializer
    {
        deserializer.deserialize(NullBoolVisitor {})
    }
}

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

#[derive(Debug, Clone, Serialize, Deserialize)]
/// a mastodon status
pub struct Status {
    pub id: StatusId,
    uri: String,
    url: String,
    account: Account,
    in_reply_to_id: Option<StatusId>,
    in_reply_to_account_id: Option<UserId>,
    reblog: Option<Box<Status>>,
    pub content: String,
    created_at: String,
    pub reblogs_count: u64,
    pub favourites_count: u64,
    #[serde(deserialize_with = "nullbool::de")]
    reblogged: bool,
    #[serde(deserialize_with = "nullbool::de")]
    favourited: bool,
    sensitive: bool,
    spoiler_text: Option<String>,
    visibility: Visibility, 
    media_attachments: Vec<MediaAttachment>,
    mentions: Vec<Mention>,
    tags: Vec<Tag>,
    application: Option<Application>,
}

impl Status {
    /// returns the original status if status is a reblog
    pub fn reblog(&self) -> Option<Self> {
        if self.reblog.is_some() {
            let r = self.reblog.clone().unwrap();
            Some(*r)
        } else {
            None
        }
    }
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
/// Status Visibility
pub enum Visibility {
    Public,
    Unlisted,
    Private,
    Direct,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum MediaAttachment {
    #[serde(rename = "image")]
    Image {
        url: String,
        preview_url: String,
    },
    #[serde(rename = "video")]
    Video {
        url: String,
        preview_url: String,
    },
    #[serde(rename = "gifv")]
    GifV {
        url: String,
        preview_url: String,
    }
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
/// a #hashtag
pub struct Tag {
    name: String,
    url: String,
}

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "#{}", self.name)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mention {
    pub id: UserId,
    url: String,
    username: String,
    acct: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Mastodon Account
pub struct Account {
    pub id: UserId,
    pub username: String,
    pub acct: String,
    pub display_name: String,
    pub note: String,
    url: String,
    avatar: String,
    header: String,
    locked: bool,
    created_at: String,
    pub followers_count: u64,
    pub following_count: u64,
    pub statuses_count: u64,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
/// the app from which the status was posted
pub struct Application {
    name: String,
    website: Option<String>,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
/// a masodon server
pub struct Instance {
    uri: String,
    pub title: String,
    pub description: String,
    email: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
/// a server sent error
/// in most cases these will be translated into results
pub struct ServerError {
    error: String
}

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "server error: {}", self.error)
    }
}

impl error::Error for ServerError {
    fn description(&self) -> &str {
        "server error"
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}