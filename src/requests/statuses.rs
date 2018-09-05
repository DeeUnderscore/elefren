use std::{borrow::Cow, convert::Into};

/// Builder for making a client.statuses() call
///
/// # Example
///
/// ```
/// # extern crate elefren;
/// # use elefren::StatusesRequest;
/// let mut request = StatusesRequest::new();
/// request.only_media().pinned().since_id("foo");
/// # assert_eq!(&request.to_querystring()[..], "?only_media=1&pinned=1&since_id=foo");
/// ```
#[derive(Clone, Debug, Default, PartialEq)]
pub struct StatusesRequest<'a> {
    only_media: bool,
    exclude_replies: bool,
    pinned: bool,
    max_id: Option<Cow<'a, str>>,
    since_id: Option<Cow<'a, str>>,
    limit: Option<usize>,
}

impl<'a> Into<Option<StatusesRequest<'a>>> for &'a mut StatusesRequest<'a> {
    fn into(self) -> Option<StatusesRequest<'a>> {
        Some(StatusesRequest {
            only_media: self.only_media,
            exclude_replies: self.exclude_replies,
            pinned: self.pinned,
            max_id: self.max_id.clone(),
            since_id: self.since_id.clone(),
            limit: self.limit.clone(),
        })
    }
}

impl<'a> StatusesRequest<'a> {
    /// Construct a new `StatusesRequest` object
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate elefren;
    /// # use elefren::StatusesRequest;
    /// let request = StatusesRequest::new();
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the `?only_media=1` flag for the .statuses() request
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate elefren;
    /// # use elefren::StatusesRequest;
    /// let mut request = StatusesRequest::new();
    /// assert_eq!(&request.only_media().to_querystring(), "?only_media=1");
    pub fn only_media(&mut self) -> &mut Self {
        self.only_media = true;
        self
    }

    /// Set the `?exclude_replies=1` flag for the .statuses() request
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate elefren;
    /// # use elefren::StatusesRequest;
    /// let mut request = StatusesRequest::new();
    /// assert_eq!(
    ///     &request.exclude_replies().to_querystring(),
    ///     "?exclude_replies=1"
    /// );
    /// ```
    pub fn exclude_replies(&mut self) -> &mut Self {
        self.exclude_replies = true;
        self
    }

    /// Set the `?pinned=1` flag for the .statuses() request
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate elefren;
    /// # use elefren::StatusesRequest;
    /// let mut request = StatusesRequest::new();
    /// assert_eq!(&request.pinned().to_querystring(), "?pinned=1");
    /// ```
    pub fn pinned(&mut self) -> &mut Self {
        self.pinned = true;
        self
    }

    /// Set the `?max_id=:max_id` flag for the .statuses() request
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate elefren;
    /// # use elefren::StatusesRequest;
    /// let mut request = StatusesRequest::new();
    /// assert_eq!(&request.max_id("foo").to_querystring(), "?max_id=foo");
    /// ```
    pub fn max_id<S: Into<Cow<'a, str>>>(&mut self, max_id: S) -> &mut Self {
        self.max_id = Some(max_id.into());
        self
    }

    /// Set the `?since_id=:since_id` flag for the .statuses() request
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate elefren;
    /// # use elefren::StatusesRequest;
    /// let mut request = StatusesRequest::new();
    /// assert_eq!(&request.since_id("foo").to_querystring(), "?since_id=foo");
    /// ```
    pub fn since_id<S: Into<Cow<'a, str>>>(&mut self, since_id: S) -> &mut Self {
        self.since_id = Some(since_id.into());
        self
    }

    /// Set the `?limit=:limit` flag for the .statuses() request
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate elefren;
    /// # use elefren::StatusesRequest;
    /// let mut request = StatusesRequest::new();
    /// assert_eq!(&request.limit(10).to_querystring(), "?limit=10");
    /// ```
    pub fn limit(&mut self, limit: usize) -> &mut Self {
        self.limit = Some(limit);
        self
    }

    /// Turns this builder into a querystring
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate elefren;
    /// # use elefren::StatusesRequest;
    /// let mut request = StatusesRequest::new();
    /// assert_eq!(
    ///     &request.limit(10).pinned().to_querystring(),
    ///     "?pinned=1&limit=10"
    /// );
    /// ```
    pub fn to_querystring(&self) -> String {
        let mut opts = vec![];

        if self.only_media {
            opts.push("only_media=1".into());
        }

        if self.exclude_replies {
            opts.push("exclude_replies=1".into());
        }

        if self.pinned {
            opts.push("pinned=1".into());
        }

        if let Some(ref max_id) = self.max_id {
            opts.push(format!("max_id={}", max_id));
        }

        if let Some(ref since_id) = self.since_id {
            opts.push(format!("since_id={}", since_id));
        }

        if let Some(limit) = self.limit {
            opts.push(format!("limit={}", limit));
        }

        if opts.is_empty() {
            String::new()
        } else {
            format!("?{}", opts.join("&"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let request = StatusesRequest::new();
        assert_eq!(
            request,
            StatusesRequest {
                only_media: false,
                exclude_replies: false,
                pinned: false,
                max_id: None,
                since_id: None,
                limit: None,
            }
        );
    }

    #[test]
    fn test_only_media() {
        let mut request = StatusesRequest::new();
        request.only_media();
        assert_eq!(
            request,
            StatusesRequest {
                only_media: true,
                exclude_replies: false,
                pinned: false,
                max_id: None,
                since_id: None,
                limit: None,
            }
        );
    }

    #[test]
    fn test_exclude_replies() {
        let mut request = StatusesRequest::new();
        request.exclude_replies();
        assert_eq!(
            request,
            StatusesRequest {
                only_media: false,
                exclude_replies: true,
                pinned: false,
                max_id: None,
                since_id: None,
                limit: None,
            }
        );
    }
    #[test]
    fn test_pinned() {
        let mut request = StatusesRequest::new();
        request.pinned();
        assert_eq!(
            request,
            StatusesRequest {
                only_media: false,
                exclude_replies: false,
                pinned: true,
                max_id: None,
                since_id: None,
                limit: None,
            }
        );
    }
    #[test]
    fn test_max_id() {
        let mut request = StatusesRequest::new();
        request.max_id("foo");
        assert_eq!(
            request,
            StatusesRequest {
                only_media: false,
                exclude_replies: false,
                pinned: false,
                max_id: Some("foo".into()),
                since_id: None,
                limit: None,
            }
        );
    }
    #[test]
    fn test_since_id() {
        let mut request = StatusesRequest::new();
        request.since_id("foo");
        assert_eq!(
            request,
            StatusesRequest {
                only_media: false,
                exclude_replies: false,
                pinned: false,
                max_id: None,
                since_id: Some("foo".into()),
                limit: None,
            }
        );
    }
    #[test]
    fn test_limit() {
        let mut request = StatusesRequest::new();
        request.limit(42);
        assert_eq!(
            request,
            StatusesRequest {
                only_media: false,
                exclude_replies: false,
                pinned: false,
                max_id: None,
                since_id: None,
                limit: Some(42),
            }
        );
    }
    #[test]
    fn test_to_querystring() {
        macro_rules! qs_test {
            (|$r:ident| $b:block, $expected:expr) => {
                {
                    let mut $r = StatusesRequest::new();
                    $b
                    let qs = $r.to_querystring();
                    assert_eq!(&qs, $expected);
                }
            }
        }

        qs_test!(
            |request| {
                request.only_media();
            },
            "?only_media=1"
        );
        qs_test!(
            |request| {
                request.exclude_replies();
            },
            "?exclude_replies=1"
        );
        qs_test!(
            |request| {
                request.pinned();
            },
            "?pinned=1"
        );
        qs_test!(
            |request| {
                request.max_id("foo");
            },
            "?max_id=foo"
        );
        qs_test!(
            |request| {
                request.since_id("foo");
            },
            "?since_id=foo"
        );
        qs_test!(
            |request| {
                request.limit(42);
            },
            "?limit=42"
        );
        qs_test!(
            |request| {
                request.only_media().exclude_replies();
            },
            "?only_media=1&exclude_replies=1"
        );
        qs_test!(
            |request| {
                request.only_media().pinned();
            },
            "?only_media=1&pinned=1"
        );
        qs_test!(
            |request| {
                request.only_media().max_id("foo");
            },
            "?only_media=1&max_id=foo"
        );
        qs_test!(
            |request| {
                request.only_media().since_id("foo");
            },
            "?only_media=1&since_id=foo"
        );
        qs_test!(
            |request| {
                request.only_media().limit(42);
            },
            "?only_media=1&limit=42"
        );
        qs_test!(
            |request| {
                request.exclude_replies().only_media();
            },
            "?only_media=1&exclude_replies=1"
        );
        qs_test!(
            |request| {
                request.exclude_replies().pinned();
            },
            "?exclude_replies=1&pinned=1"
        );
        qs_test!(
            |request| {
                request.exclude_replies().max_id("foo");
            },
            "?exclude_replies=1&max_id=foo"
        );
        qs_test!(
            |request| {
                request.exclude_replies().since_id("foo");
            },
            "?exclude_replies=1&since_id=foo"
        );
        qs_test!(
            |request| {
                request.exclude_replies().limit(42);
            },
            "?exclude_replies=1&limit=42"
        );
        qs_test!(
            |request| {
                request.pinned().only_media();
            },
            "?only_media=1&pinned=1"
        );
        qs_test!(
            |request| {
                request.pinned().exclude_replies();
            },
            "?exclude_replies=1&pinned=1"
        );
        qs_test!(
            |request| {
                request.pinned().max_id("foo");
            },
            "?pinned=1&max_id=foo"
        );
        qs_test!(
            |request| {
                request.pinned().since_id("foo");
            },
            "?pinned=1&since_id=foo"
        );
        qs_test!(
            |request| {
                request.pinned().limit(42);
            },
            "?pinned=1&limit=42"
        );
        qs_test!(
            |request| {
                request.max_id("foo").only_media();
            },
            "?only_media=1&max_id=foo"
        );
        qs_test!(
            |request| {
                request.max_id("foo").exclude_replies();
            },
            "?exclude_replies=1&max_id=foo"
        );
        qs_test!(
            |request| {
                request.max_id("foo").pinned();
            },
            "?pinned=1&max_id=foo"
        );
        qs_test!(
            |request| {
                request.max_id("foo").since_id("foo");
            },
            "?max_id=foo&since_id=foo"
        );
        qs_test!(
            |request| {
                request.max_id("foo").limit(42);
            },
            "?max_id=foo&limit=42"
        );
        qs_test!(
            |request| {
                request.since_id("foo").only_media();
            },
            "?only_media=1&since_id=foo"
        );
        qs_test!(
            |request| {
                request.since_id("foo").exclude_replies();
            },
            "?exclude_replies=1&since_id=foo"
        );
        qs_test!(
            |request| {
                request.since_id("foo").pinned();
            },
            "?pinned=1&since_id=foo"
        );
        qs_test!(
            |request| {
                request.since_id("foo").max_id("foo");
            },
            "?max_id=foo&since_id=foo"
        );
        qs_test!(
            |request| {
                request.since_id("foo").limit(42);
            },
            "?since_id=foo&limit=42"
        );
        qs_test!(
            |request| {
                request.limit(42).only_media();
            },
            "?only_media=1&limit=42"
        );
        qs_test!(
            |request| {
                request.limit(42).exclude_replies();
            },
            "?exclude_replies=1&limit=42"
        );
        qs_test!(
            |request| {
                request.limit(42).pinned();
            },
            "?pinned=1&limit=42"
        );
        qs_test!(
            |request| {
                request.limit(42).max_id("foo");
            },
            "?max_id=foo&limit=42"
        );
        qs_test!(
            |request| {
                request.limit(42).since_id("foo");
            },
            "?since_id=foo&limit=42"
        );
    }
}
