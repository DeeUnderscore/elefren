use std::borrow::Cow;

use entities::prelude::*;
use errors::Result;
use http_send::{HttpSend, HttpSender};
use page::Page;
use requests::{StatusesRequest, UpdateCredsRequest};
use status_builder::StatusBuilder;

/// Represents the set of methods that a Mastodon Client can do, so that
/// implementations might be swapped out for testing
#[allow(unused)]
pub trait MastodonClient<H: HttpSend = HttpSender> {
    /// GET /api/v1/favourites
    fn favourites(&self) -> Result<Page<Status, H>> {
        unimplemented!("This method was not implemented");
    }
    /// GET /api/v1/blocks
    fn blocks(&self) -> Result<Page<Account, H>> {
        unimplemented!("This method was not implemented");
    }
    /// GET /api/v1/domain_blocks
    fn domain_blocks(&self) -> Result<Page<String, H>> {
        unimplemented!("This method was not implemented");
    }
    /// GET /api/v1/follow_requests
    fn follow_requests(&self) -> Result<Page<Account, H>> {
        unimplemented!("This method was not implemented");
    }
    /// GET /api/v1/timelines/home
    fn get_home_timeline(&self) -> Result<Page<Status, H>> {
        unimplemented!("This method was not implemented");
    }
    /// GET /api/v1/custom_emojis
    fn get_emojis(&self) -> Result<Page<Emoji, H>> {
        unimplemented!("This method was not implemented");
    }
    /// GET /api/v1/mutes
    fn mutes(&self) -> Result<Page<Account, H>> {
        unimplemented!("This method was not implemented");
    }
    /// GET /api/v1/notifications
    fn notifications(&self) -> Result<Page<Notification, H>> {
        unimplemented!("This method was not implemented");
    }
    /// GET /api/v1/reports
    fn reports(&self) -> Result<Page<Report, H>> {
        unimplemented!("This method was not implemented");
    }
    /// GET /api/v1/accounts/:id/followers
    fn followers(&self, id: &str) -> Result<Page<Account, H>> {
        unimplemented!("This method was not implemented");
    }
    /// GET /api/v1/accounts/:id/following
    fn following(&self, id: &str) -> Result<Page<Account, H>> {
        unimplemented!("This method was not implemented");
    }
    /// GET /api/v1/statuses/:id/reblogged_by
    fn reblogged_by(&self, id: &str) -> Result<Page<Account, H>> {
        unimplemented!("This method was not implemented");
    }
    /// GET /api/v1/statuses/:id/favourited_by
    fn favourited_by(&self, id: &str) -> Result<Page<Account, H>> {
        unimplemented!("This method was not implemented");
    }
    /// DELETE /api/v1/domain_blocks
    fn unblock_domain(&self, domain: String) -> Result<Empty> {
        unimplemented!("This method was not implemented");
    }
    /// GET /api/v1/instance
    fn instance(&self) -> Result<Instance> {
        unimplemented!("This method was not implemented");
    }
    /// GET /api/v1/accounts/verify_credentials
    fn verify_credentials(&self) -> Result<Account> {
        unimplemented!("This method was not implemented");
    }
    /// POST /api/v1/reports
    fn report(&self, account_id: &str, status_ids: Vec<&str>, comment: String) -> Result<Report> {
        unimplemented!("This method was not implemented");
    }
    /// POST /api/v1/domain_blocks
    fn block_domain(&self, domain: String) -> Result<Empty> {
        unimplemented!("This method was not implemented");
    }
    /// POST /api/v1/accounts/follow_requests/authorize
    fn authorize_follow_request(&self, id: &str) -> Result<Empty> {
        unimplemented!("This method was not implemented");
    }
    /// POST /api/v1/accounts/follow_requests/reject
    fn reject_follow_request(&self, id: &str) -> Result<Empty> {
        unimplemented!("This method was not implemented");
    }
    /// POST /api/v1/search
    fn search(&self, q: String, resolve: bool) -> Result<SearchResult> {
        unimplemented!("This method was not implemented");
    }
    /// POST /api/v1/follows
    fn follows(&self, uri: Cow<'static, str>) -> Result<Account> {
        unimplemented!("This method was not implemented");
    }
    /// POST /api/v1/media
    fn media(&self, file: Cow<'static, str>) -> Result<Attachment> {
        unimplemented!("This method was not implemented");
    }
    /// POST /api/v1/notifications/clear
    fn clear_notifications(&self) -> Result<Empty> {
        unimplemented!("This method was not implemented");
    }
    /// GET /api/v1/accounts/:id
    fn get_account(&self, id: u64) -> Result<Account> {
        unimplemented!("This method was not implemented");
    }
    /// POST /api/v1/accounts/:id/follow
    fn follow(&self, id: u64) -> Result<Account> {
        unimplemented!("This method was not implemented");
    }
    /// POST /api/v1/accounts/:id/unfollow
    fn unfollow(&self, id: u64) -> Result<Relationship> {
        unimplemented!("This method was not implemented");
    }
    /// GET /api/v1/accounts/:id/block
    fn block(&self, id: u64) -> Result<Account> {
        unimplemented!("This method was not implemented");
    }
    /// GET /api/v1/accounts/:id/unblock
    fn unblock(&self, id: u64) -> Result<Account> {
        unimplemented!("This method was not implemented");
    }
    /// GET /api/v1/accounts/:id/mute
    fn mute(&self, id: u64) -> Result<Account> {
        unimplemented!("This method was not implemented");
    }
    /// GET /api/v1/accounts/:id/unmute
    fn unmute(&self, id: u64) -> Result<Account> {
        unimplemented!("This method was not implemented");
    }
    /// GET /api/v1/notifications/:id
    fn get_notification(&self, id: u64) -> Result<Notification> {
        unimplemented!("This method was not implemented");
    }
    /// GET /api/v1/statuses/:id
    fn get_status(&self, id: u64) -> Result<Status> {
        unimplemented!("This method was not implemented");
    }
    /// GET /api/v1/statuses/:id/context
    fn get_context(&self, id: u64) -> Result<Context> {
        unimplemented!("This method was not implemented");
    }
    /// GET /api/v1/statuses/:id/card
    fn get_card(&self, id: u64) -> Result<Card> {
        unimplemented!("This method was not implemented");
    }
    /// POST /api/v1/statuses/:id/reblog
    fn reblog(&self, id: u64) -> Result<Status> {
        unimplemented!("This method was not implemented");
    }
    /// POST /api/v1/statuses/:id/unreblog
    fn unreblog(&self, id: u64) -> Result<Status> {
        unimplemented!("This method was not implemented");
    }
    /// POST /api/v1/statuses/:id/favourite
    fn favourite(&self, id: u64) -> Result<Status> {
        unimplemented!("This method was not implemented");
    }
    /// POST /api/v1/statuses/:id/unfavourite
    fn unfavourite(&self, id: u64) -> Result<Status> {
        unimplemented!("This method was not implemented");
    }
    /// DELETE /api/v1/statuses/:id
    fn delete_status(&self, id: u64) -> Result<Empty> {
        unimplemented!("This method was not implemented");
    }
    /// PATCH /api/v1/accounts/update_credentials
    fn update_credentials(&self, builder: &mut UpdateCredsRequest) -> Result<Account> {
        unimplemented!("This method was not implemented");
    }
    /// POST /api/v1/statuses
    fn new_status(&self, status: StatusBuilder) -> Result<Status> {
        unimplemented!("This method was not implemented");
    }
    /// GET /api/v1/timelines/public
    fn get_public_timeline(&self, local: bool) -> Result<Vec<Status>> {
        unimplemented!("This method was not implemented");
    }
    /// GET /api/v1/timelines/tag/:hashtag
    fn get_tagged_timeline(&self, hashtag: String, local: bool) -> Result<Vec<Status>> {
        unimplemented!("This method was not implemented");
    }
    /// GET /api/v1/accounts/:id/statuses
    fn statuses<'a, 'b: 'a, S>(&'b self, id: &'b str, request: S) -> Result<Page<Status, H>>
    where
        S: Into<Option<StatusesRequest<'a>>>,
    {
        unimplemented!("This method was not implemented");
    }
    /// GET /api/v1/accounts/relationships?
    fn relationships(&self, ids: &[&str]) -> Result<Page<Relationship, H>> {
        unimplemented!("This method was not implemented");
    }
    /// GET /api/v1/accounts/search?q=:query&limit=:limit&following=:following
    fn search_accounts(
        &self,
        query: &str,
        limit: Option<u64>,
        following: bool,
    ) -> Result<Page<Account, H>> {
        unimplemented!("This method was not implemented");
    }
}
