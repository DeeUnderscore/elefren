use std::{error, fmt, io::Error as IoError};

use json::Error as SerdeError;
use reqwest::{Error as HttpError, StatusCode};
#[cfg(feature = "toml")]
use tomlcrate::de::Error as TomlDeError;
#[cfg(feature = "toml")]
use tomlcrate::ser::Error as TomlSerError;
use url::ParseError as UrlError;

/// Convience type over `std::result::Result` with `Error` as the error type.
pub type Result<T> = ::std::result::Result<T, Error>;

/// enum of possible errors encountered using the mastodon API.
#[derive(Debug)]
pub enum Error {
    /// Error from the Mastodon API. This typically means something went
    /// wrong with your authentication or data.
    Api(ApiError),
    /// Error deserialising to json. Typically represents a breaking change in
    /// the Mastodon API
    Serde(SerdeError),
    /// Error encountered in the HTTP backend while requesting a route.
    Http(HttpError),
    /// Wrapper around the `std::io::Error` struct.
    Io(IoError),
    /// Wrapper around the `url::ParseError` struct.
    Url(UrlError),
    /// Missing Client Id.
    ClientIdRequired,
    /// Missing Client Secret.
    ClientSecretRequired,
    /// Missing Access Token.
    AccessTokenRequired,
    /// Generic client error.
    Client(StatusCode),
    /// Generic server error.
    Server(StatusCode),
    /// MastodonBuilder error.
    DataMissing,
    /// AppBuilder error
    MissingField(&'static str),
    #[cfg(feature = "toml")]
    /// Error serializing to toml
    TomlSer(TomlSerError),
    #[cfg(feature = "toml")]
    /// Error deserializing from toml
    TomlDe(TomlDeError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Api(ref e) => e
                .error_description
                .as_ref()
                .map(|i| &**i)
                .or(e.error.as_ref().map(|i| &**i))
                .unwrap_or("Unknown API Error"),
            Error::Serde(ref e) => e.description(),
            Error::Http(ref e) => e.description(),
            Error::Io(ref e) => e.description(),
            Error::Url(ref e) => e.description(),
            Error::Client(ref status) | Error::Server(ref status) => {
                status.canonical_reason().unwrap_or("Unknown Status code")
            },
            Error::ClientIdRequired => "ClientIdRequired",
            Error::ClientSecretRequired => "ClientSecretRequired",
            Error::AccessTokenRequired => "AccessTokenRequired",
            Error::DataMissing => "DataMissing",
            Error::MissingField(_) => "MissingField",
            #[cfg(feature = "toml")]
            Error::TomlSer(ref e) => e.description(),
            #[cfg(feature = "toml")]
            Error::TomlDe(ref e) => e.description(),
        }
    }
}

/// Error returned from the Mastodon API.
#[derive(Clone, Debug, Deserialize)]
pub struct ApiError {
    /// The type of error.
    pub error: Option<String>,
    /// The description of the error.
    pub error_description: Option<String>,
}

macro_rules! from {
    ($($(#[$met:meta])* $typ:ident, $variant:ident,)*) => {
        $(
            $(#[$met])*
            impl From<$typ> for Error {
                fn from(from: $typ) -> Self {
                    use Error::*;
                    $variant(from)
                }
            }
        )*
    }
}

from! {
    HttpError, Http,
    IoError, Io,
    SerdeError, Serde,
    UrlError, Url,
    ApiError, Api,
    #[cfg(feature = "toml")] TomlSerError, TomlSer,
    #[cfg(feature = "toml")] TomlDeError, TomlDe,
}
