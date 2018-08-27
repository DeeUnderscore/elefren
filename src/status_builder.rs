/// A builder pattern struct for constructing a status.
#[derive(Debug, Default, Clone, Serialize, PartialEq)]
pub struct StatusBuilder {
    /// The text of the status.
    pub status: String,
    /// Ids of accounts being replied to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_reply_to_id: Option<u64>,
    /// Ids of media attachments being attached to the status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_ids: Option<Vec<u64>>,
    /// Whether current status is sensitive.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sensitive: Option<bool>,
    /// Text to precede the normal status text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spoiler_text: Option<String>,
    /// Visibility of the status, defaults to `Public`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<Visibility>,
}

/// The visibility of a status.
#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Visibility {
    /// A Direct message to a user
    Direct,
    /// Only available to followers
    Private,
    /// Not shown in public timelines
    Unlisted,
    /// Posted to public timelines
    Public,
}

impl StatusBuilder {
    /// Create a new status with text.
    /// ```
    /// use elefren::prelude::*;
    ///
    /// let status = StatusBuilder::new("Hello World!");
    /// ```
    pub fn new<I: Into<String>>(status: I) -> Self {
        StatusBuilder {
            status: status.into(),
            ..Self::default()
        }
    }
}

impl Default for Visibility {
    fn default() -> Self {
        Visibility::Public
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let s = StatusBuilder::new("a status");
        let expected = StatusBuilder {
            status: "a status".to_string(),
            in_reply_to_id: None,
            media_ids: None,
            sensitive: None,
            spoiler_text: None,
            visibility: None,
        };
        assert_eq!(s, expected);
    }

    #[test]
    fn test_default_visibility() {
        let v = Visibility::default();
        assert_eq!(v, Visibility::Public);
    }
}
