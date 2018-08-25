#[cfg(feature = "toml")]
/// Helpers for serializing to/deserializing from toml
///
/// In order to use this module, set the "toml" feature in your Cargo.toml:
///
/// ```toml,ignore
/// [dependencies.elefren]
/// version = "0.12"
/// features = ["toml"]
/// ```
pub mod toml;
