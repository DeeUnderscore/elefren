use std::path::Path;
use std::borrow::Cow;

use reqwest;


/// A builder pattern struct for preparing a single attachment for upload.
/// 
/// For more details, see [`new_media()`](struct.Mastodon.html#method.new_media).
#[derive(Debug)]
pub struct MediaBuilder {
    /// The media attachment itself
    pub data: MediaBuilderData,

    /// The filename to send to the server
    pub filename: Option<String>,

    /// Mimetype to send to the server, identifying what is in the attachment.
    /// 
    /// The string should be a valid mimetype.
    pub mimetype: Option<String>,

    /// Plain text description of the attached piece of media, for accessibility
    pub description: Option<String>,

    /// (x, y) focus point, used by clients to determine how to crop an image
    pub focus: Option<(f64, f64)>,
}

/// Enum representing possible sources of attachments to upload
#[derive(Debug)]
pub enum MediaBuilderData{
    /// Arbitrary data, wrapped as reqwest's Body
    Body(reqwest::Body),

    /// Variant represening a file path of the file to attach.
    File(Cow<'static, Path>),
}

impl MediaBuilder {
    /// Create a new MediaBuilder from a reader `data`
    pub fn from_body<T: Into<reqwest::Body>>(data: T) -> MediaBuilder {
        MediaBuilder {
            data: MediaBuilderData::Body(data.into()),
            filename: None,
            mimetype: None,
            description: None,
            focus: None,
        }
    }

    /// Create a new MediaBuilder from a file under `path`
    ///
    /// This function will not check whether the file exists or if it can be read. If the path is
    /// not valid, [`media()`](trait.MastodonClient.html#method.media) will return an error when called with the `MediaBuilder`.
    pub fn from_file<T: Into<Cow<'static, Path>>>(path: T) -> MediaBuilder {
        MediaBuilder {
            data: MediaBuilderData::File(path.into()),
            filename: None,
            mimetype: None,
            description: None,
            focus: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_reader() {
        let source = vec![0u8, 1, 2];
        let builder = MediaBuilder::from_body(source.clone());

        assert_eq!(builder.filename, None);
        assert_eq!(builder.mimetype, None);
        assert_eq!(builder.description, None);
        assert_eq!(builder.focus, None);

        if let MediaBuilderData::Body(r) = builder.data {
            assert_eq!(r.as_bytes(), Some(source.as_slice()));
        } else {
            panic!("Unable to destructure MediaBuilder.data into a reader");
        }
    }

    #[test]
    fn test_from_file() {
        let builder = MediaBuilder::from_file(Path::new("/fake/file/path.png"));

        assert_eq!(builder.filename, None);
        assert_eq!(builder.mimetype, None);
        assert_eq!(builder.description, None);
        assert_eq!(builder.focus, None);

        if let MediaBuilderData::File(f) = builder.data {
            assert_eq!(f.to_str(), Some("/fake/file/path.png"));
        } else {
            panic!("Unable to destructure MediaBuilder.data into a filepath");
        }
    }
}
