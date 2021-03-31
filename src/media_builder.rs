use std::borrow::Cow;
use std::fmt;
use std::io::Read;

#[derive(Debug)]
/// A builder pattern struct for preparing a single attachment for upload.
/// 
/// For more details, see [`new_media()`](struct.Mastodon.html#method.new_media).
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
pub enum MediaBuilderData {
    /// An arbitrary reader. It is useful for reading from media already in memory.
    Reader(Box<dyn Read + Send>),

    /// Variant represening a file path of the file to attach.
    File(Cow<'static, str>),
}

impl fmt::Debug for MediaBuilderData {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MediaBuilderData::File(f) => fmt.debug_tuple("File").field(&f).finish(),
            MediaBuilderData::Reader(_) => fmt
                .debug_tuple("Reader")
                .field(&format_args!("..."))
                .finish(),
        }
    }
}

impl MediaBuilder {
    /// Create a new MediaBuilder from a reader `data`
    pub fn from_reader<R: Read + Send + 'static>(data: R) -> MediaBuilder {
        MediaBuilder {
            data: MediaBuilderData::Reader(Box::from(data)),
            filename: None,
            mimetype: None,
            description: None,
            focus: None,
        }
    }

    /// Create a new MediaBuilder from a file under `path`
    ///
    /// This function will not check whether the file exists or if it can be read. If the path is
    /// not valid, [`add_media()`](trait.MastodonClient.html#method.add_media) will return an error when called with the `MediaBuilder`.
    pub fn from_file(path: Cow<'static, str>) -> MediaBuilder {
        MediaBuilder {
            data: MediaBuilderData::File(path),
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
    use std::io::Cursor;


    #[test]
    fn test_from_reader() {
        let source = vec![0u8, 1, 2];
        let builder = MediaBuilder::from_reader(Cursor::new(source.clone()));

        assert_eq!(builder.filename, None);
        assert_eq!(builder.mimetype, None);
        assert_eq!(builder.description, None);
        assert_eq!(builder.focus, None);

        if let MediaBuilderData::Reader(r) = builder.data {
            assert_eq!(r.bytes().map(|b| b.unwrap()).collect::<Vec<u8>>(), source);
        } else {
            panic!("Unable to destructure MediaBuilder.data into a reader");
        }
    }

    #[test]
    fn test_from_file() {
        let builder = MediaBuilder::from_file("/fake/file/path.png".into());

        assert_eq!(builder.filename, None);
        assert_eq!(builder.mimetype, None);
        assert_eq!(builder.description, None);
        assert_eq!(builder.focus, None);

        if let MediaBuilderData::File(f) = builder.data {
            assert_eq!(f, "/fake/file/path.png");
        } else {
            panic!("Unable to destructure MediaBuilder.data into a filepath");
        }
    }
}
