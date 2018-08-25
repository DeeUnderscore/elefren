use std::{borrow::Cow, fmt};

use errors::{Error, Result};

/// Represents an application that can be registered with a mastodon instance
#[derive(Debug, Default, Serialize)]
pub struct App {
    client_name: String,
    redirect_uris: String,
    scopes: Scopes,
    #[serde(skip_serializing_if = "Option::is_none")]
    website: Option<String>,
}

impl App {
    pub fn builder<'a>() -> AppBuilder<'a> {
        AppBuilder::new()
    }

    pub fn scopes(&self) -> Scopes {
        self.scopes
    }
}

/// Builder struct for defining your application.
/// ```
/// use elefren::apps::App;
/// use std::error::Error;
///
/// # fn main() -> Result<(), Box<Error>> {
/// let mut builder = App::builder();
/// builder.client_name("elefren_test");
/// let app = builder.build()?;
/// #   Ok(())
/// # }
/// ```
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct AppBuilder<'a> {
    client_name: Option<Cow<'a, str>>,
    redirect_uris: Option<Cow<'a, str>>,
    scopes: Option<Scopes>,
    website: Option<Cow<'a, str>>,
}

impl<'a> AppBuilder<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// Name of the application. Will be displayed when the user is deciding to
    /// grant permission.
    ///
    /// In order to turn this builder into an App, this needs to be provided
    pub fn client_name<I: Into<Cow<'a, str>>>(&mut self, name: I) -> &mut Self {
        self.client_name = Some(name.into());
        self
    }

    /// Where the user should be redirected after authorization
    ///
    /// If none is specified, the default is `urn:ietf:wg:oauth:2.0:oob`
    pub fn redirect_uris<I: Into<Cow<'a, str>>>(&mut self, uris: I) -> &mut Self {
        self.redirect_uris = Some(uris.into());
        self
    }

    /// Permission scope of the application.
    ///
    /// IF none is specified, the default is Scopes::Read
    pub fn scopes(&mut self, scopes: Scopes) -> &mut Self {
        self.scopes = Some(scopes);
        self
    }

    /// URL to the homepage of your application.
    pub fn website<I: Into<Cow<'a, str>>>(&mut self, website: I) -> &mut Self {
        self.website = Some(website.into());
        self
    }

    /// Attempts to convert this build into an `App`
    ///
    /// Will fail if no `client_name` was provided
    pub fn build(self) -> Result<App> {
        Ok(App {
            client_name: self
                .client_name
                .ok_or_else(|| Error::MissingField("client_name"))?
                .into(),
            redirect_uris: self
                .redirect_uris
                .unwrap_or_else(|| "urn:ietf:wg:oauth:2.0:oob".into())
                .into(),
            scopes: self.scopes.unwrap_or_else(|| Scopes::Read),
            website: self.website.map(|s| s.into()),
        })
    }
}

/// Permission scope of the application.
/// [Details on what each permission provides][1]
/// [1]: https://github.com/tootsuite/documentation/blob/master/Using-the-API/OAuth-details.md)
#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub enum Scopes {
    /// All Permissions, equivalent to `read write follow`
    #[serde(rename = "read write follow")]
    All,
    /// Only permission to add and remove followers.
    #[serde(rename = "follow")]
    Follow,
    /// Read only permissions.
    #[serde(rename = "read")]
    Read,
    /// Read & Follow permissions.
    #[serde(rename = "read follow")]
    ReadFollow,
    /// Read & Write permissions.
    #[serde(rename = "read write")]
    ReadWrite,
    /// Write only permissions.
    #[serde(rename = "write")]
    Write,
    /// Write & Follow permissions.
    #[serde(rename = "write follow")]
    WriteFollow,
}

impl fmt::Display for Scopes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Scopes::*;
        write!(
            f,
            "{}",
            match *self {
                All => "read%20write%20follow",
                Follow => "follow",
                Read => "read",
                ReadFollow => "read%20follow",
                ReadWrite => "read%20write",
                Write => "write",
                WriteFollow => "write%20follow",
            }
        )
    }
}

impl Default for Scopes {
    fn default() -> Self {
        Scopes::Read
    }
}
