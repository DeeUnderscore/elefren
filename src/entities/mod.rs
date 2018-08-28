pub mod account;
pub mod attachment;
pub mod card;
pub mod context;
pub mod instance;
pub(crate) mod itemsiter;
pub mod list;
pub mod mention;
pub mod notification;
pub mod relationship;
pub mod report;
pub mod search_result;
pub mod status;

/// An empty JSON object.
#[derive(Deserialize, Debug, Copy, Clone, PartialEq)]
pub struct Empty {}

pub mod prelude {
    //! The purpose of this module is to alleviate imports of many common
    //! structs by adding a glob import to the top of mastodon heavy
    //! modules:
    pub use super::{
        account::{Account, CredentialsBuilder, Source},
        attachment::{Attachment, MediaType},
        card::Card,
        context::Context,
        instance::*,
        list::List,
        mention::Mention,
        notification::Notification,
        relationship::Relationship,
        report::Report,
        search_result::SearchResult,
        status::{Application, Emoji, Status},
        Empty,
    };
}
