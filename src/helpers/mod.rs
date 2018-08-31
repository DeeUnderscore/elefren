#[cfg(feature = "toml")]
/// Helpers for serializing to/deserializing from toml
///
/// In order to use this module, set the "toml" feature in your Cargo.toml:
///
/// ```toml,ignore
/// [dependencies.elefren]
/// version = "0.14"
/// features = ["toml"]
/// ```
pub mod toml;

#[cfg(feature = "json")]
/// Helpers for serializing to/deserializing from json
///
/// In order to use this module, set the "json" feature in your Cargo.toml:
///
/// ```toml,ignore
/// [dependencies.elefen]
/// version = "0.14"
/// features = ["json"]
/// ```
pub mod json;

/// Helpers for working with the command line
pub mod cli;
