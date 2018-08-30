use std::borrow::Cow;

/// Builder for making a client.statuses() call
///
/// # Example
///
/// ```
/// # extern crate elefren;
/// # use elefren::StatusesRequest;
/// let request = StatusesRequest::new().only_media().pinned().since_id("foo");
/// # assert_eq!(&request.to_querystring()[..], "?only_media=1&pinned=1&since_id=foo");
/// ```
#[derive(Clone, Debug, Default)]
pub struct StatusesRequest<'a> {
    only_media: bool,
    exclude_replies: bool,
    pinned: bool,
    max_id: Option<Cow<'a, str>>,
    since_id: Option<Cow<'a, str>>,
    limit: Option<usize>,
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
    /// let request = StatusesRequest::new().only_media();
    /// assert_eq!(&request.to_querystring(), "?only_media=1");
    pub fn only_media(mut self) -> Self {
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
    /// let request = StatusesRequest::new().exclude_replies();
    /// assert_eq!(&request.to_querystring(), "?exclude_replies=1");
    /// ```
    pub fn exclude_replies(mut self) -> Self {
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
    /// let request = StatusesRequest::new().pinned();
    /// assert_eq!(&request.to_querystring(), "?pinned=1");
    /// ```
    pub fn pinned(mut self) -> Self {
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
    /// let request = StatusesRequest::new().max_id("foo");
    /// assert_eq!(&request.to_querystring(), "?max_id=foo");
    /// ```
    pub fn max_id<S: Into<Cow<'a, str>>>(mut self, max_id: S) -> Self {
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
    /// let request = StatusesRequest::new().since_id("foo");
    /// assert_eq!(&request.to_querystring(), "?since_id=foo");
    /// ```
    pub fn since_id<S: Into<Cow<'a, str>>>(mut self, since_id: S) -> Self {
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
    /// let request = StatusesRequest::new().limit(10);
    /// assert_eq!(&request.to_querystring(), "?limit=10");
    /// ```
    pub fn limit(mut self, limit: usize) -> Self {
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
    /// let request = StatusesRequest::new().limit(10).pinned();
    /// assert_eq!(&request.to_querystring(), "?pinned=1&limit=10");
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
