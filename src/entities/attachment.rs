//! Module containing everything related to media attachements.
use serde::Deserialize;

/// A struct representing a media attachment.
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct Attachment {
    /// ID of the attachment.
    pub id: String,
    /// The media type of an attachment.
    #[serde(rename = "type")]
    pub media_type: MediaType,
    /// URL of the locally hosted version of the image.
    pub url: String,
    /// For remote images, the remote URL of the original image.
    pub remote_url: Option<String>,
    /// URL of the preview image, can be null for audio files.
    pub preview_url: Option<String>,
    /// Shorter URL for the image, for insertion into text
    /// (only present on local images)
    pub text_url: Option<String>,
    /// Meta information about the attachment.
    pub meta: Option<Meta>,
    /// Noop will be removed.
    pub description: Option<String>,
}

/// Information about the attachment itself.
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Meta {
    /// Original version.
    pub original: Option<ImageDetails>,
    /// Smaller version.
    pub small: Option<ImageDetails>,
}

/// Dimensions of an attachement.
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct ImageDetails {
    /// width of attachment, can be null for audio files.
    width: Option<u64>,
    /// height of attachment, can be null for audio files.
    height: Option<u64>,
    /// A string of `widthxheight`.
    size: Option<String>,
    /// The aspect ratio of the attachment.
    aspect: Option<f64>,
    /// Duration if this is a video/audio file in seconds.
    duration: Option<f64>,
}

/// The type of media attachment.
#[derive(Debug, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum MediaType {
    /// An image.
    #[serde(rename = "image")]
    Image,
    /// A video file.
    #[serde(rename = "video")]
    Video,
    /// A gifv format file.
    #[serde(rename = "gifv")]
    Gifv,
    /// A audio file.
    #[serde(rename = "audio")]
    Audio,
    /// Unknown format.
    #[serde(rename = "unknown")]
    Unknown,
}
