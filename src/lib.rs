//! # Elefren: API Wrapper around the Mastodon API.
//!
//! Most of the api is documented on [Mastodon's
//! github](https://github.com/tootsuite/mastodon/blob/master/docs/Using-the-API/API.md#tag)
//!
//! ```no_run
//! # extern crate elefren;
//! # fn main() {
//! #    try().unwrap();
//! # }
//! # fn try() -> elefren::Result<()> {
//! use elefren::{helpers::cli, prelude::*};
//!
//! let registration = Registration::new("https://mastodon.social")
//!     .client_name("elefren_test")
//!     .build()?;
//! let mastodon = cli::authenticate(registration)?;
//!
//! println!(
//!     "{:?}",
//!     mastodon
//!         .get_home_timeline()?
//!         .items_iter()
//!         .take(100)
//!         .collect::<Vec<_>>()
//! );
//! # Ok(())
//! # }
//! ```

#![deny(
    missing_docs,
    warnings,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications
)]
#![allow(intra_doc_link_resolution_failure)]

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate doc_comment;
extern crate hyper_old_types;
extern crate isolang;
#[macro_use]
extern crate serde_json;
extern crate chrono;
extern crate reqwest;
extern crate serde;
extern crate serde_urlencoded;
extern crate tap_reader;
extern crate try_from;
extern crate url;

#[cfg(feature = "toml")]
extern crate toml as tomlcrate;

#[cfg(test)]
extern crate tempfile;

#[cfg(test)]
#[cfg_attr(
    all(test, any(feature = "toml", feature = "json")),
    macro_use
)]
extern crate indoc;

use std::{borrow::Cow, ops};

use reqwest::{multipart, Client, RequestBuilder, Response};
use tap_reader::Tap;

use entities::prelude::*;
use http_send::{HttpSend, HttpSender};
use page::Page;

pub use data::Data;
pub use errors::{ApiError, Error, Result};
pub use isolang::Language;
pub use mastodon_client::MastodonClient;
pub use media_builder::MediaBuilder;
pub use registration::Registration;
pub use requests::{
    AddFilterRequest,
    AddPushRequest,
    StatusesRequest,
    UpdateCredsRequest,
    UpdatePushRequest,
};
pub use status_builder::StatusBuilder;

/// Registering your App
pub mod apps;
/// Contains the struct that holds the client auth data
pub mod data;
/// Entities returned from the API
pub mod entities;
/// Errors
pub mod errors;
/// Collection of helpers for serializing/deserializing `Data` objects
pub mod helpers;
/// Contains trait for converting `reqwest::Request`s to `reqwest::Response`s
pub mod http_send;
mod mastodon_client;
/// Constructing a media attachment for upload
pub mod media_builder;
/// Handling multiple pages of entities.
pub mod page;
/// Registering your app.
pub mod registration;
/// Requests
pub mod requests;
/// OAuth Scopes
pub mod scopes;
/// Constructing a status
pub mod status_builder;

#[macro_use]
mod macros;
/// Automatically import the things you need
pub mod prelude {
    pub use scopes::Scopes;
    pub use Data;
    pub use Mastodon;
    pub use MastodonClient;
    pub use Registration;
    pub use StatusBuilder;
    pub use StatusesRequest;
}

/// Your mastodon application client, handles all requests to and from Mastodon.
#[derive(Clone, Debug)]
pub struct Mastodon<H: HttpSend = HttpSender> {
    client: Client,
    http_sender: H,
    /// Raw data about your mastodon instance.
    pub data: Data,
}

impl<H: HttpSend> Mastodon<H> {
    methods![get, post, delete,];

    fn route(&self, url: &str) -> String {
        format!("{}{}", self.base, url)
    }

    pub(crate) fn send(&self, req: RequestBuilder) -> Result<Response> {
        Ok(self
            .http_sender
            .send(&self.client, req.bearer_auth(&self.token))?)
    }
}

impl From<Data> for Mastodon<HttpSender> {
    /// Creates a mastodon instance from the data struct.
    fn from(data: Data) -> Mastodon<HttpSender> {
        let mut builder = MastodonBuilder::new(HttpSender);
        builder.data(data);
        builder
            .build()
            .expect("We know `data` is present, so this should be fine")
    }
}

impl<H: HttpSend> MastodonClient<H> for Mastodon<H> {
    paged_routes! {
        (get) favourites: "favourites" => Status,
        (get) blocks: "blocks" => Account,
        (get) domain_blocks: "domain_blocks" => String,
        (get) follow_requests: "follow_requests" => Account,
        (get) get_home_timeline: "timelines/home" => Status,
        (get) get_emojis: "custom_emojis" => Emoji,
        (get) mutes: "mutes" => Account,
        (get) notifications: "notifications" => Notification,
        (get) reports: "reports" => Report,
        (get (q: &'a str, #[serde(skip_serializing_if = "Option::is_none")] limit: Option<u64>, following: bool,)) search_accounts: "accounts/search" => Account,
        (get) get_endorsements: "endorsements" => Account,
    }

    paged_routes_with_id! {
        (get) followers: "accounts/{}/followers" => Account,
        (get) following: "accounts/{}/following" => Account,
        (get) reblogged_by: "statuses/{}/reblogged_by" => Account,
        (get) favourited_by: "statuses/{}/favourited_by" => Account,
    }

    route! {
        (delete (domain: String,)) unblock_domain: "domain_blocks" => Empty,
        (get) instance: "instance" => Instance,
        (get) verify_credentials: "accounts/verify_credentials" => Account,
        (post (account_id: &str, status_ids: Vec<&str>, comment: String,)) report: "reports" => Report,
        (post (domain: String,)) block_domain: "domain_blocks" => Empty,
        (post (id: &str,)) authorize_follow_request: "accounts/follow_requests/authorize" => Empty,
        (post (id: &str,)) reject_follow_request: "accounts/follow_requests/reject" => Empty,
        (get  (q: &'a str, resolve: bool,)) search: "search" => SearchResult,
        (get  (local: bool,)) get_public_timeline: "timelines/public" => Vec<Status>,
        (post (uri: Cow<'static, str>,)) follows: "follows" => Account,
        (post multipart (file: Cow<'static, str>,)) media: "media" => Attachment,
        (post) clear_notifications: "notifications/clear" => Empty,
        (get) get_push_subscription: "push/subscription" => Subscription,
        (delete) delete_push_subscription: "push/subscription" => Empty,
        (get) get_filters: "filters" => Vec<Filter>,
        (get) get_follow_suggestions: "suggestions" => Vec<Account>,
    }

    route_v2! {
        (get (q: &'a str, resolve: bool,)) search_v2: "search" => SearchResultV2,
    }

    route_id! {
        (get) get_account: "accounts/{}" => Account,
        (post) follow: "accounts/{}/follow" => Account,
        (post) unfollow: "accounts/{}/unfollow" => Relationship,
        (get) block: "accounts/{}/block" => Account,
        (get) unblock: "accounts/{}/unblock" => Account,
        (get) mute: "accounts/{}/mute" => Account,
        (get) unmute: "accounts/{}/unmute" => Account,
        (get) get_notification: "notifications/{}" => Notification,
        (get) get_status: "statuses/{}" => Status,
        (get) get_context: "statuses/{}/context" => Context,
        (get) get_card: "statuses/{}/card" => Card,
        (post) reblog: "statuses/{}/reblog" => Status,
        (post) unreblog: "statuses/{}/unreblog" => Status,
        (post) favourite: "statuses/{}/favourite" => Status,
        (post) unfavourite: "statuses/{}/unfavourite" => Status,
        (delete) delete_status: "statuses/{}" => Empty,
        (get) get_filter: "filters/{}" => Filter,
        (delete) delete_filter: "filters/{}" => Empty,
        (delete) delete_from_suggestions: "suggestions/{}" => Empty,
        (post) endorse_user: "accounts/{}/pin" => Relationship,
        (post) unendorse_user: "accounts/{}/unpin" => Relationship,
    }

    fn add_filter(&self, request: &mut AddFilterRequest) -> Result<Filter> {
        let url = self.route("/api/v1/filters");
        let response = self.send(self.client.post(&url).json(&request))?;

        let status = response.status();

        if status.is_client_error() {
            return Err(Error::Client(status.clone()));
        } else if status.is_server_error() {
            return Err(Error::Server(status.clone()));
        }

        deserialise(response)
    }

    /// PUT /api/v1/filters/:id
    fn update_filter(&self, id: &str, request: &mut AddFilterRequest) -> Result<Filter> {
        let url = self.route(&format!("/api/v1/filters/{}", id));
        let response = self.send(self.client.put(&url).json(&request))?;

        let status = response.status();

        if status.is_client_error() {
            return Err(Error::Client(status.clone()));
        } else if status.is_server_error() {
            return Err(Error::Server(status.clone()));
        }

        deserialise(response)
    }

    fn update_credentials(&self, builder: &mut UpdateCredsRequest) -> Result<Account> {
        let changes = builder.build()?;
        let url = self.route("/api/v1/accounts/update_credentials");
        let response = self.send(self.client.patch(&url).json(&changes))?;

        let status = response.status();

        if status.is_client_error() {
            return Err(Error::Client(status.clone()));
        } else if status.is_server_error() {
            return Err(Error::Server(status.clone()));
        }

        deserialise(response)
    }

    /// Post a new status to the account.
    fn new_status(&self, status: StatusBuilder) -> Result<Status> {
        let response = self.send(
            self.client
                .post(&self.route("/api/v1/statuses"))
                .json(&status),
        )?;

        deserialise(response)
    }

    /// Get timeline filtered by a hashtag(eg. `#coffee`) either locally or
    /// federated.
    fn get_tagged_timeline(&self, hashtag: String, local: bool) -> Result<Vec<Status>> {
        let base = "/api/v1/timelines/tag/";
        let url = if local {
            self.route(&format!("{}{}?local=1", base, hashtag))
        } else {
            self.route(&format!("{}{}", base, hashtag))
        };

        self.get(url)
    }

    /// Get statuses of a single account by id. Optionally only with pictures
    /// and or excluding replies.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # extern crate elefren;
    /// # use elefren::prelude::*;
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<Error>> {
    /// # let data = Data {
    /// #   base: "".into(),
    /// #   client_id: "".into(),
    /// #   client_secret: "".into(),
    /// #   redirect: "".into(),
    /// #   token: "".into(),
    /// # };
    /// let client = Mastodon::from(data);
    /// let statuses = client.statuses("user-id", None)?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// ```no_run
    /// # extern crate elefren;
    /// # use elefren::prelude::*;
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<Error>> {
    /// # let data = Data {
    /// #   base: "".into(),
    /// #   client_id: "".into(),
    /// #   client_secret: "".into(),
    /// #   redirect: "".into(),
    /// #   token: "".into(),
    /// # };
    /// let client = Mastodon::from(data);
    /// let mut request = StatusesRequest::new();
    /// request.only_media();
    /// let statuses = client.statuses("user-id", request)?;
    /// # Ok(())
    /// # }
    /// ```
    fn statuses<'a, 'b: 'a, S>(&'b self, id: &'b str, request: S) -> Result<Page<Status, H>>
    where
        S: Into<Option<StatusesRequest<'a>>>,
    {
        let mut url = format!("{}/api/v1/accounts/{}/statuses", self.base, id);

        if let Some(request) = request.into() {
            url = format!("{}{}", url, request.to_querystring());
        }

        let response = self.send(self.client.get(&url))?;

        Page::new(self, response)
    }

    /// Returns the client account's relationship to a list of other accounts.
    /// Such as whether they follow them or vice versa.
    fn relationships(&self, ids: &[&str]) -> Result<Page<Relationship, H>> {
        let mut url = self.route("/api/v1/accounts/relationships?");

        if ids.len() == 1 {
            url += "id=";
            url += &ids[0];
        } else {
            for id in ids {
                url += "id[]=";
                url += &id;
                url += "&";
            }
            url.pop();
        }

        let response = self.send(self.client.get(&url))?;

        Page::new(self, response)
    }

    /// Add a push notifications subscription
    fn add_push_subscription(&self, request: &AddPushRequest) -> Result<Subscription> {
        let request = request.build()?;
        let response = self.send(
            self.client
                .post(&self.route("/api/v1/push/subscription"))
                .json(&request),
        )?;

        deserialise(response)
    }

    /// Update the `data` portion of the push subscription associated with this
    /// access token
    fn update_push_data(&self, request: &UpdatePushRequest) -> Result<Subscription> {
        let request = request.build();
        let response = self.send(
            self.client
                .put(&self.route("/api/v1/push/subscription"))
                .json(&request),
        )?;

        deserialise(response)
    }

    /// Get all accounts that follow the authenticated user
    fn follows_me(&self) -> Result<Page<Account, H>> {
        let me = self.verify_credentials()?;
        Ok(self.followers(&me.id)?)
    }

    /// Get all accounts that the authenticated user follows
    fn followed_by_me(&self) -> Result<Page<Account, H>> {
        let me = self.verify_credentials()?;
        Ok(self.following(&me.id)?)
    }

    /// Upload some media to the server for possible attaching to a new status
    ///
    /// Upon successful upload of a media attachment, the server will assign it an id. To actually
    /// use the attachment in a new status, you can use the `media_ids` field of
    /// [`StatusBuilder`](status_builder/struct.StatusBuilder.html)
    ///
    /// There are two ways of providing the data to be attached: by reading a file, or by using a
    /// reader.
    ///
    /// ## Files
    /// If the `MediaBuilder` was supplied with a file path, the filename and mimetype will be
    /// automatically populated from that file; their values set in the `MediaBuilder` will be
    /// ignored. For example:
    ///
    /// ```no_run
    /// let client = Mastodon::from(data);
    /// let builder = MediaBuilder::from_file("/tmp/my_image.png".into());
    ///
    /// let attachment = client.add_media(builder);
    /// ```
    ///
    /// ## Readers
    /// The `MediaBuilder` can also be supplied with a reader. This is useful for uploading data
    /// already in memory, for example from a `Vec<u8>` containing some image data. For example:
    ///
    /// ```no_run
    /// use std::io::Cursor;
    /// let client = Mastodon::from(data);
    ///
    /// let mut image_data: Vec<u8> = Vec::new();
    /// populate_image_data(&mut image_data);
    ///
    /// let builder = MediaBuilder::from_reader(Cursor::new(image_data));
    /// let attachment = client.add_media(builder):
    ///
    /// ```
    ///
    /// ## Errors
    /// This function may return an `Error::Http` before sending anything over the network if the
    /// `MediaBuilder` was supplied with a reader and a `mimetype` string which cannot be pasrsed. 
    fn new_media(&self, media: MediaBuilder) -> Result<Attachment> {
        use media_builder::MediaBuilderData;

        let mut form = multipart::Form::new();
        form = match media.data {
            MediaBuilderData::Reader(reader) => {
                let mut part = multipart::Part::reader(reader);

                if let Some(filename) = media.filename {
                    part = part.file_name(filename);
                }

                if let Some(mimetype) = media.mimetype {
                    part = part.mime_str(&mimetype)?;
                }

                form.part("file", part)
            }
            MediaBuilderData::File(file) => form.file("file", file.as_ref())?,
        };

        if let Some(description) = media.description {
            form = form.text("description", description);
        }

        if let Some((x, y)) = media.focus {
            form = form.text("focus", format!("{},{}", x, y));
        }

        let response = self.send(
            self.client
                .post(&self.route("/api/v1/media"))
                .multipart(form),
        )?;

        deserialise(response)
    }
}

impl<H: HttpSend> ops::Deref for Mastodon<H> {
    type Target = Data;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

struct MastodonBuilder<H: HttpSend> {
    client: Option<Client>,
    http_sender: H,
    data: Option<Data>,
}

impl<H: HttpSend> MastodonBuilder<H> {
    pub fn new(sender: H) -> Self {
        MastodonBuilder {
            http_sender: sender,
            client: None,
            data: None,
        }
    }

    pub fn client(&mut self, client: Client) -> &mut Self {
        self.client = Some(client);
        self
    }

    pub fn data(&mut self, data: Data) -> &mut Self {
        self.data = Some(data);
        self
    }

    pub fn build(self) -> Result<Mastodon<H>> {
        Ok(if let Some(data) = self.data {
            Mastodon {
                client: self.client.unwrap_or_else(|| Client::new()),
                http_sender: self.http_sender,
                data,
            }
        } else {
            return Err(Error::MissingField("missing field 'data'"));
        })
    }
}

// Convert the HTTP response body from JSON. Pass up deserialization errors
// transparently.
fn deserialise<T: for<'de> serde::Deserialize<'de>>(response: Response) -> Result<T> {
    let mut reader = Tap::new(response);

    match serde_json::from_reader(&mut reader) {
        Ok(t) => Ok(t),
        // If deserializing into the desired type fails try again to
        // see if this is an error response.
        Err(e) => {
            if let Ok(error) = serde_json::from_slice(&reader.bytes) {
                return Err(Error::Api(error));
            }
            Err(e.into())
        },
    }
}
