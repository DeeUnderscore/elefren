use std::borrow::Cow;

use reqwest::{Client, RequestBuilder, Response};
use serde::Deserialize;
use std::convert::TryInto;

use crate::{
    apps::{App, AppBuilder},
    scopes::Scopes,
    Data,
    Error,
    Mastodon,
    MastodonBuilder,
    Result,
};

const DEFAULT_REDIRECT_URI: &str = "urn:ietf:wg:oauth:2.0:oob";

/// Handles registering your mastodon app to your instance. It is recommended
/// you cache your data struct to avoid registering on every run.
#[derive(Debug, Clone)]
pub struct Registration<'a> {
    base: String,
    client: Client,
    app_builder: AppBuilder<'a>,
    force_login: bool,
}

#[derive(Deserialize)]
struct OAuth {
    client_id: String,
    client_secret: String,
    #[serde(default = "default_redirect_uri")]
    redirect_uri: String,
}

fn default_redirect_uri() -> String {
    DEFAULT_REDIRECT_URI.to_string()
}

#[derive(Deserialize)]
struct AccessToken {
    access_token: String,
}

impl<'a> Registration<'a> {
    /// Construct a new registration process to the instance of the `base` url.
    /// ```
    /// use elefren::prelude::*;
    ///
    /// let registration = Registration::new("https://mastodon.social");
    /// ```
    pub fn new<I: Into<String>>(base: I) -> Self {
        Registration {
            base: base.into(),
            client: Client::new(),
            app_builder: AppBuilder::new(),
            force_login: false,
        }
    }
}

impl<'a> Registration<'a> {
    /// Sets the name of this app
    ///
    /// This is required, and if this isn't set then the AppBuilder::build
    /// method will fail
    pub fn client_name<I: Into<Cow<'a, str>>>(&mut self, name: I) -> &mut Self {
        self.app_builder.client_name(name.into());
        self
    }

    /// Sets the redirect uris that this app uses
    pub fn redirect_uris<I: Into<Cow<'a, str>>>(&mut self, uris: I) -> &mut Self {
        self.app_builder.redirect_uris(uris);
        self
    }

    /// Sets the scopes that this app requires
    ///
    /// The default for an app is Scopes::Read
    pub fn scopes(&mut self, scopes: Scopes) -> &mut Self {
        self.app_builder.scopes(scopes);
        self
    }

    /// Sets the optional "website" to register the app with
    pub fn website<I: Into<Cow<'a, str>>>(&mut self, website: I) -> &mut Self {
        self.app_builder.website(website);
        self
    }

    /// Forces the user to re-login (useful if you need to re-auth as a
    /// different user on the same instance
    pub fn force_login(&mut self, force_login: bool) -> &mut Self {
        self.force_login = force_login;
        self
    }

    fn send(&self, req: RequestBuilder) -> Result<Response> {
        let req = req.build()?;
        let handle = tokio::runtime::Handle::current();
        Ok(handle.block_on(self.client.execute(req))?)
    }

    /// Register the given application
    ///
    /// ```no_run
    /// # extern crate elefren;
    /// # fn main () -> elefren::Result<()> {
    /// use elefren::{apps::App, prelude::*};
    ///
    /// let mut app = App::builder();
    /// app.client_name("elefren_test");
    ///
    /// let registration = Registration::new("https://mastodon.social").register(app)?;
    /// let url = registration.authorize_url()?;
    /// // Here you now need to open the url in the browser
    /// // And handle a the redirect url coming back with the code.
    /// let code = String::from("RETURNED_FROM_BROWSER");
    /// let mastodon = registration.complete(&code)?;
    ///
    /// println!("{:?}", mastodon.get_home_timeline()?.initial_items);
    /// # Ok(())
    /// # }
    /// ```
    pub fn register<I: TryInto<App>>(&mut self, app: I) -> Result<Registered>
    where
        Error: From<<I as TryInto<App>>::Error>,
    {
        let app = app.try_into()?;
        let oauth = self.send_app(&app)?;

        Ok(Registered {
            base: self.base.clone(),
            client: self.client.clone(),
            client_id: oauth.client_id,
            client_secret: oauth.client_secret,
            redirect: oauth.redirect_uri,
            scopes: app.scopes().clone(),
            force_login: self.force_login,
        })
    }

    /// Register the application with the server from the `base` url.
    ///
    /// ```no_run
    /// # extern crate elefren;
    /// # fn main () -> elefren::Result<()> {
    /// use elefren::prelude::*;
    ///
    /// let registration = Registration::new("https://mastodon.social")
    ///     .client_name("elefren_test")
    ///     .build()?;
    /// let url = registration.authorize_url()?;
    /// // Here you now need to open the url in the browser
    /// // And handle a the redirect url coming back with the code.
    /// let code = String::from("RETURNED_FROM_BROWSER");
    /// let mastodon = registration.complete(&code)?;
    ///
    /// println!("{:?}", mastodon.get_home_timeline()?.initial_items);
    /// # Ok(())
    /// # }
    /// ```
    pub fn build(&mut self) -> Result<Registered> {
        let app: App = self.app_builder.clone().build()?;
        let oauth = self.send_app(&app)?;

        Ok(Registered {
            base: self.base.clone(),
            client: self.client.clone(),
            client_id: oauth.client_id,
            client_secret: oauth.client_secret,
            redirect: oauth.redirect_uri,
            scopes: app.scopes().clone(),
            force_login: self.force_login,
        })
    }

    fn send_app(&self, app: &App) -> Result<OAuth> {
        let url = format!("{}/api/v1/apps", self.base);
        let handle = tokio::runtime::Handle::current();
        Ok(handle.block_on(self.send(self.client.post(&url).json(&app))?.json())?)
    }
}

impl Registered {
    /// Skip having to retrieve the client id and secret from the server by
    /// creating a `Registered` struct directly
    ///
    /// # Example
    ///
    /// ```no_run
    /// # extern crate elefren;
    /// # fn main() -> elefren::Result<()> {
    /// use elefren::{prelude::*, registration::Registered};
    ///
    /// let registration = Registered::from_parts(
    ///     "https://example.com",
    ///     "the-client-id",
    ///     "the-client-secret",
    ///     "https://example.com/redirect",
    ///     Scopes::read_all(),
    ///     false,
    /// );
    /// let url = registration.authorize_url()?;
    /// // Here you now need to open the url in the browser
    /// // And handle a the redirect url coming back with the code.
    /// let code = String::from("RETURNED_FROM_BROWSER");
    /// let mastodon = registration.complete(&code)?;
    ///
    /// println!("{:?}", mastodon.get_home_timeline()?.initial_items);
    /// #   Ok(())
    /// # }
    /// ```
    pub fn from_parts(
        base: &str,
        client_id: &str,
        client_secret: &str,
        redirect: &str,
        scopes: Scopes,
        force_login: bool,
    ) -> Registered {
        Registered {
            base: base.to_string(),
            client: Client::new(),
            client_id: client_id.to_string(),
            client_secret: client_secret.to_string(),
            redirect: redirect.to_string(),
            scopes,
            force_login,
        }
    }
}

impl Registered {
    fn send(&self, req: RequestBuilder) -> Result<Response> {
        let req = req.build()?;
        let handle = tokio::runtime::Handle::current();
        Ok(handle.block_on(self.client.execute(req))?)
    }

    /// Returns the parts of the `Registered` struct that can be used to
    /// recreate another `Registered` struct
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate elefren;
    /// use elefren::{prelude::*, registration::Registered};
    /// # fn main() -> Result<(), Box<std::error::Error>> {
    ///
    /// let origbase = "https://example.social";
    /// let origclient_id = "some-client_id";
    /// let origclient_secret = "some-client-secret";
    /// let origredirect = "https://example.social/redirect";
    /// let origscopes = Scopes::all();
    /// let origforce_login = false;
    ///
    /// let registered = Registered::from_parts(
    ///     origbase,
    ///     origclient_id,
    ///     origclient_secret,
    ///     origredirect,
    ///     origscopes.clone(),
    ///     origforce_login,
    /// );
    ///
    /// let (base, client_id, client_secret, redirect, scopes, force_login) = registered.into_parts();
    ///
    /// assert_eq!(origbase, &base);
    /// assert_eq!(origclient_id, &client_id);
    /// assert_eq!(origclient_secret, &client_secret);
    /// assert_eq!(origredirect, &redirect);
    /// assert_eq!(origscopes, scopes);
    /// assert_eq!(origforce_login, force_login);
    /// #   Ok(())
    /// # }
    /// ```
    pub fn into_parts(self) -> (String, String, String, String, Scopes, bool) {
        (
            self.base,
            self.client_id,
            self.client_secret,
            self.redirect,
            self.scopes,
            self.force_login,
        )
    }

    /// Returns the full url needed for authorisation. This needs to be opened
    /// in a browser.
    pub fn authorize_url(&self) -> Result<String> {
        let mut url = url::Url::parse(&self.base)?.join("/oauth/authorize")?;

        url.query_pairs_mut()
            .append_pair("client_id", &self.client_id)
            .append_pair("redirect_uri", &self.redirect)
            .append_pair("scope", &self.scopes.to_string())
            .append_pair("response_type", "code")
            .append_pair("force_login", &self.force_login.to_string());

        Ok(url.into_string())
    }

    /// Create an access token from the client id, client secret, and code
    /// provided by the authorisation url.
    pub fn complete(&self, code: &str) -> Result<Mastodon> {
        let url = format!(
            "{}/oauth/token?client_id={}&client_secret={}&code={}&grant_type=authorization_code&\
             redirect_uri={}",
            self.base, self.client_id, self.client_secret, code, self.redirect
        );

        let handle = tokio::runtime::Handle::current();
        let token: AccessToken = handle.block_on(self.send(self.client.post(&url))?.json())?;

        let data = Data {
            base: self.base.clone().into(),
            client_id: self.client_id.clone().into(),
            client_secret: self.client_secret.clone().into(),
            redirect: self.redirect.clone().into(),
            token: token.access_token.into(),
        };

        let mut builder = MastodonBuilder::new();
        builder.client(self.client.clone()).data(data);
        Ok(builder.build()?)
    }
}

/// Represents the state of the auth flow when the app has been registered but
/// the user is not authenticated
#[derive(Debug, Clone)]
pub struct Registered {
    base: String,
    client: Client,
    client_id: String,
    client_secret: String,
    redirect: String,
    scopes: Scopes,
    force_login: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registration_new() {
        let r = Registration::new("https://example.com");
        assert_eq!(r.base, "https://example.com".to_string());
        assert_eq!(r.app_builder, AppBuilder::new());
    }

    #[test]
    fn test_registration_with_sender() {
        let r = Registration::new("https://example.com");
        assert_eq!(r.base, "https://example.com".to_string());
        assert_eq!(r.app_builder, AppBuilder::new());
    }

    #[test]
    fn test_set_client_name() {
        let mut r = Registration::new("https://example.com");
        r.client_name("foo-test");

        assert_eq!(r.base, "https://example.com".to_string());
        assert_eq!(
            &mut r.app_builder,
            AppBuilder::new().client_name("foo-test")
        );
    }

    #[test]
    fn test_set_redirect_uris() {
        let mut r = Registration::new("https://example.com");
        r.redirect_uris("https://foo.com");

        assert_eq!(r.base, "https://example.com".to_string());
        assert_eq!(
            &mut r.app_builder,
            AppBuilder::new().redirect_uris("https://foo.com")
        );
    }

    #[test]
    fn test_set_scopes() {
        let mut r = Registration::new("https://example.com");
        r.scopes(Scopes::all());

        assert_eq!(r.base, "https://example.com".to_string());
        assert_eq!(&mut r.app_builder, AppBuilder::new().scopes(Scopes::all()));
    }

    #[test]
    fn test_set_website() {
        let mut r = Registration::new("https://example.com");
        r.website("https://website.example.com");

        assert_eq!(r.base, "https://example.com".to_string());
        assert_eq!(
            &mut r.app_builder,
            AppBuilder::new().website("https://website.example.com")
        );
    }

    #[test]
    fn test_default_redirect_uri() {
        assert_eq!(&default_redirect_uri()[..], DEFAULT_REDIRECT_URI);
    }
}
