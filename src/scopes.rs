use std::fmt;

/// Permission scope of the application.
/// [Details on what each permission provides][1]
/// [1]: https://github.com/tootsuite/documentation/blob/master/Using-the-API/OAuth-details.md)
#[derive(Debug, Clone, Copy, PartialEq, Hash, Serialize)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scopes_display() {
        let values = [
            Scopes::All,
            Scopes::Follow,
            Scopes::Read,
            Scopes::ReadFollow,
            Scopes::ReadWrite,
            Scopes::Write,
            Scopes::WriteFollow,
        ];

        let expecteds = [
            "read%20write%20follow".to_string(),
            "follow".to_string(),
            "read".to_string(),
            "read%20follow".to_string(),
            "read%20write".to_string(),
            "write".to_string(),
            "write%20follow".to_string(),
        ];

        let tests = values.into_iter().zip(expecteds.into_iter());

        for (value, expected) in tests {
            let result = value.to_string();
            assert_eq!(&result, expected);
        }
    }

    #[test]
    fn test_scopes_default() {
        let default: Scopes = Default::default();
        assert_eq!(default, Scopes::Read);
    }
}
