use std::borrow::Cow;

/// Raw data about mastodon app. Save `Data` using `serde` to prevent needing
/// to authenticate on every run.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Data {
    /// Base url of instance eg. `https://mastodon.social`.
    pub base: Cow<'static, str>,
    /// The client's id given by the instance.
    pub client_id: Cow<'static, str>,
    /// The client's secret given by the instance.
    pub client_secret: Cow<'static, str>,
    /// Url to redirect back to your application from the instance signup.
    pub redirect: Cow<'static, str>,
    /// The client's access token.
    pub token: Cow<'static, str>,
}

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
pub mod toml {
    use super::Data;
    use std::{
        fs::{File, OpenOptions},
        io::{BufWriter, Read, Write},
        path::Path,
    };
    use Result;

    use tomlcrate;

    /// Attempts to deserialize a Data struct from a string
    pub fn from_str(s: &str) -> Result<Data> {
        Ok(tomlcrate::from_str(s)?)
    }

    /// Attempts to deserialize a Data struct from a slice of bytes
    pub fn from_slice(s: &[u8]) -> Result<Data> {
        Ok(tomlcrate::from_slice(s)?)
    }

    /// Attempts to deserialize a Data struct from something that implements
    /// the std::io::Read trait
    pub fn from_reader<R: Read>(mut r: R) -> Result<Data> {
        let mut buffer = Vec::new();
        r.read_to_end(&mut buffer)?;
        from_slice(&buffer)
    }

    /// Attempts to deserialize a Data struct from a file
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Data> {
        let path = path.as_ref();
        let file = File::open(path)?;
        Ok(from_reader(file)?)
    }

    /// Attempts to serialize a Data struct to a String
    pub fn to_string(data: &Data) -> Result<String> {
        Ok(tomlcrate::to_string_pretty(data)?)
    }

    /// Attempts to serialize a Data struct to a Vec of bytes
    pub fn to_vec(data: &Data) -> Result<Vec<u8>> {
        Ok(tomlcrate::to_vec(data)?)
    }

    /// Attempts to serialize a Data struct to something that implements the
    /// std::io::Write trait
    pub fn to_writer<W: Write>(data: &Data, writer: W) -> Result<()> {
        let mut buf_writer = BufWriter::new(writer);
        let vec = to_vec(data)?;
        buf_writer.write(&vec)?;
        Ok(())
    }

    /// Attempts to serialize a Data struct to a file
    ///
    /// When opening the file, this will set the `.write(true)` and
    /// `.truncate(true)` options, use the next method for more
    /// fine-grained control
    pub fn to_file<P: AsRef<Path>>(data: &Data, path: P) -> Result<()> {
        let mut options = OpenOptions::new();
        options.write(true).truncate(true);
        to_file_with_options(data, path, options)?;
        Ok(())
    }

    /// Attempts to serialize a Data struct to a file
    pub fn to_file_with_options<P: AsRef<Path>>(
        data: &Data,
        path: P,
        options: OpenOptions,
    ) -> Result<()> {
        let path = path.as_ref();
        let file = options.open(path)?;
        to_writer(data, file)?;
        Ok(())
    }
}
