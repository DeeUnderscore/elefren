use crate::entities::status::Emoji;
use serde::{Deserialize, Serialize};

/// Represents a poll attached to a status.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Poll {
    /// The ID of the poll in the database.
    pub id: String,
    /// When the poll ends.
    pub expires_at: String, // Datetime??
    /// Is the poll currently expired?
    pub expired: bool,
    /// Does the poll allow multiple-choice answers?
    pub multiple: bool,
    /// How many votes have been received.
    pub votes_count: u64,
    /// How many unique accounts have voted on a multiple-choice poll.
    pub voters_count: Option<u64>,
    /// When called with a user token, has the authorized user voted?
    pub voted: Option<bool>,
    /// When called with a user token, which options has the authorized user
    /// chosen? Contains an array of index values for options
    pub own_votes: Option<Vec<u64>>,
    /// Possible answers for the poll.
    pub options: Vec<PollOption>,
    /// Custom emoji to be used for rendering poll options.
    pub emojis: Vec<Emoji>,
}

/// Possible answers for the poll.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct PollOption {
    /// The text value of the poll option.
    pub title: String,
    /// The number of received votes for this option.
    pub votes_count: Option<u64>,
}
