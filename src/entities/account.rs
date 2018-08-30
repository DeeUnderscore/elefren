//! A module containing everything relating to a account returned from the api.

use chrono::prelude::*;
use reqwest::multipart::Form;
use serde::de::{self, Deserialize, Deserializer, Unexpected};
use std::path::Path;
use Result;

/// A struct representing an Account.
#[derive(Debug, Clone, Deserialize)]
pub struct Account {
    /// Equals `username` for local users, includes `@domain` for remote ones.
    pub acct: String,
    /// URL to the avatar image
    pub avatar: String,
    /// URL to the avatar static image (gif)
    pub avatar_static: String,
    /// The time the account was created.
    pub created_at: DateTime<Utc>,
    /// The account's display name.
    pub display_name: String,
    /// The number of followers for the account.
    pub followers_count: u64,
    /// The number of accounts the given account is following.
    pub following_count: u64,
    /// URL to the header image.
    pub header: String,
    /// URL to the header static image (gif).
    pub header_static: String,
    /// The ID of the account.
    pub id: String,
    /// Boolean for when the account cannot be followed without waiting for
    /// approval first.
    pub locked: bool,
    /// Biography of user.
    pub note: String,
    /// The number of statuses the account has made.
    pub statuses_count: u64,
    /// URL of the user's profile page (can be remote).
    pub url: String,
    /// The username of the account.
    pub username: String,
    /// An extra attribute given from `verify_credentials` giving defaults about
    /// a user
    pub source: Option<Source>,
    /// If the owner decided to switch accounts, new account is in
    /// this attribute
    pub moved: Option<Box<Account>>,
}

/// An extra object given from `verify_credentials` giving defaults about a user
#[derive(Debug, Clone, Deserialize)]
pub struct Source {
    privacy: ::status_builder::Visibility,
    #[serde(deserialize_with = "string_or_bool")]
    sensitive: bool,
    note: String,
}

fn string_or_bool<'de, D: Deserializer<'de>>(val: D) -> ::std::result::Result<bool, D::Error> {
    #[derive(Clone, Debug, Deserialize)]
    #[serde(untagged)]
    pub enum BoolOrString {
        Bool(bool),
        Str(String),
    }

    Ok(match BoolOrString::deserialize(val)? {
        BoolOrString::Bool(b) => b,
        BoolOrString::Str(ref s) => {
            if s == "true" {
                true
            } else if s == "false" {
                false
            } else {
                return Err(de::Error::invalid_value(
                    Unexpected::Str(s),
                    &"true or false",
                ));
            }
        },
    })
}

/// Data structure used for updating user credentials
#[derive(Debug)]
pub struct CredentialsBuilder<'a> {
    display_name: Option<&'a str>,
    note: Option<&'a str>,
    avatar: Option<&'a Path>,
    header: Option<&'a Path>,
}

impl<'a> CredentialsBuilder<'a> {
    /// Turns a `CredentialsForm` into a form suitable for PUTing to the
    /// endpoint
    pub fn into_form(self) -> Result<Form> {
        let mut form = Form::new();
        macro_rules! add_to_form {
            ($key:ident : Text; $($rest:tt)*) => {{
                if let Some(val) = self.$key {
                    form = form.text(stringify!($key), val.to_owned());
                }

                add_to_form!{$($rest)*}
            }};

            ($key:ident : File; $($rest:tt)*) => {{
                if let Some(val) = self.$key {
                    form = form.file(stringify!($key), val)?;
                }

                add_to_form!{$($rest)*}
            }};

            () => {}
        }

        add_to_form! {
            display_name: Text;
            note: Text;
            avatar: File;
            header: File;
        }

        Ok(form)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_credentials_builder_to_form() {
        let avatar = NamedTempFile::new().expect("Couldn't make avatar file");
        let header = NamedTempFile::new().expect("Couldn't make header file");
        let tests = [
            (None, None, None, None),
            (Some("my-display-name"), None, None, None),
            (None, Some("my-note"), None, None),
            (None, None, Some(avatar.path().clone()), None),
            (None, None, None, Some(header.path().clone())),
            (Some("my-display-name"), Some("my-note"), None, None),
            (
                Some("my-display-name"),
                None,
                Some(avatar.path().clone()),
                None,
            ),
            (None, Some("my-note"), Some(avatar.path().clone()), None),
            (None, Some("my-note"), None, Some(header.path().clone())),
            (
                None,
                None,
                Some(avatar.path().clone()),
                Some(header.path().clone()),
            ),
            (
                Some("my-display-name"),
                None,
                None,
                Some(header.path().clone()),
            ),
            (
                Some("my-display-name"),
                Some("my-note"),
                Some(avatar.path().clone()),
                None,
            ),
            (
                Some("my-display-name"),
                Some("my-note"),
                None,
                Some(header.path().clone()),
            ),
            (
                Some("my-display-name"),
                None,
                Some(avatar.path().clone()),
                Some(header.path().clone()),
            ),
            (
                None,
                Some("my-note"),
                Some(avatar.path().clone()),
                Some(header.path().clone()),
            ),
            (
                Some("my-display-name"),
                Some("my-note"),
                Some(avatar.path().clone()),
                Some(header.path().clone()),
            ),
        ];

        for test in tests.into_iter() {
            let (display_name, note, avatar, header) = test;
            let credentials_builder = CredentialsBuilder {
                display_name: *display_name,
                note: *note,
                avatar: *avatar,
                header: *header,
            };
            let _form = credentials_builder
                .into_form()
                .expect("could not create form");
        }
    }
}
