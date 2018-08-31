/// Data structure for the MastodonClient::statuses method
pub use self::statuses::StatusesRequest;
/// Data structure for the MastodonClient::update_credentials method
pub use self::update_credentials::UpdateCredsRequest;

mod statuses;
mod update_credentials;
