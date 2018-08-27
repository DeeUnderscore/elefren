use std::borrow::Cow;

use try_from::TryInto;
use reqwest::{Client, RequestBuilder, Response};

use apps::{App, AppBuilder, Scopes};
use http_send::{HttpSend, HttpSender};
use Data;
use Mastodon;
use MastodonBuilder;
use Result;
use Error;

const DEFAULT_REDIRECT_URI: &'static str = "urn:ietf:wg:oauth:2.0:oob";

/// Handles registering your mastodon app to your instance. It is recommended
/// you cache your data struct to avoid registering on every run.
pub struct Registration<'a, H: HttpSend = HttpSender> {
    base: String,
    client: Client,
    app_builder: AppBuilder<'a>,
    http_sender: H,
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

impl<'a> Registration<'a, HttpSender> {
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
            http_sender: HttpSender,
        }
    }
}

impl<'a, H: HttpSend> Registration<'a, H> {
    #[allow(dead_code)]
    pub(crate) fn with_sender<I: Into<String>>(base: I, http_sender: H) -> Self {
        Registration {
            base: base.into(),
            client: Client::new(),
            app_builder: AppBuilder::new(),
            http_sender,
        }
    }

    pub fn client_name<I: Into<Cow<'a, str>>>(&mut self, name: I) -> &mut Self {
        self.app_builder.client_name(name.into());
        self
    }

    pub fn redirect_uris<I: Into<Cow<'a, str>>>(&mut self, uris: I) -> &mut Self {
        self.app_builder.redirect_uris(uris);
        self
    }

    pub fn scopes(&mut self, scopes: Scopes) -> &mut Self {
        self.app_builder.scopes(scopes);
        self
    }

    pub fn website<I: Into<Cow<'a, str>>>(&mut self, website: I) -> &mut Self {
        self.app_builder.website(website);
        self
    }

    fn send(&self, req: &mut RequestBuilder) -> Result<Response> {
        Ok(self.http_sender.send(&self.client, req)?)
    }

    /// Register the given application
    ///
    /// ```no_run
    /// # extern crate elefren;
    /// # fn main () -> elefren::Result<()> {
    /// use elefren::prelude::*;
    /// use elefren::apps::App;
    ///
    /// let mut app = App::builder();
    /// app.client_name("elefren_test");
    ///
    /// let registration = Registration::new("https://mastodon.social")
    ///     .register(app)?;
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
    pub fn register<I: TryInto<App>>(&mut self, app: I) -> Result<Registered<H>>
        where Error: From<<I as TryInto<App>>::Err>
    {
        let app = app.try_into()?;
        let oauth = self.send_app(&app)?;

        Ok(Registered {
            base: self.base.clone(),
            client: self.client.clone(),
            client_id: oauth.client_id,
            client_secret: oauth.client_secret,
            redirect: oauth.redirect_uri,
            scopes: app.scopes(),
            http_sender: self.http_sender.clone(),
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
    pub fn build(&mut self) -> Result<Registered<H>> {
        let app: App = self.app_builder.clone().build()?;
        let oauth = self.send_app(&app)?;

        Ok(Registered {
            base: self.base.clone(),
            client: self.client.clone(),
            client_id: oauth.client_id,
            client_secret: oauth.client_secret,
            redirect: oauth.redirect_uri,
            scopes: app.scopes(),
            http_sender: self.http_sender.clone(),
        })
    }

    fn send_app(&self, app: &App) -> Result<OAuth> {
        let url = format!("{}/api/v1/apps", self.base);
        Ok(self.send(self.client.post(&url).form(&app))?.json()?)
    }
}

impl<H: HttpSend> Registered<H> {
    fn send(&self, req: &mut RequestBuilder) -> Result<Response> {
        Ok(self.http_sender.send(&self.client, req)?)
    }

    /// Returns the full url needed for authorisation. This needs to be opened
    /// in a browser.
    pub fn authorize_url(&self) -> Result<String> {
        let url = format!(
            "{}/oauth/authorize?client_id={}&redirect_uri={}&scope={}&response_type=code",
            self.base, self.client_id, self.redirect, self.scopes,
        );

        Ok(url)
    }

    /// Create an access token from the client id, client secret, and code
    /// provided by the authorisation url.
    pub fn complete(self, code: &str) -> Result<Mastodon<H>> {
        let url = format!(
            "{}/oauth/token?client_id={}&client_secret={}&code={}&grant_type=authorization_code&redirect_uri={}",
            self.base,
            self.client_id,
            self.client_secret,
            code,
            self.redirect
        );

        let token: AccessToken = self.send(&mut self.client.post(&url))?.json()?;

        let data = Data {
            base: self.base.into(),
            client_id: self.client_id.into(),
            client_secret: self.client_secret.into(),
            redirect: self.redirect.into(),
            token: token.access_token.into(),
        };

        let mut builder = MastodonBuilder::new(self.http_sender);
        builder.client(self.client).data(data);
        Ok(builder.build()?)
    }
}

pub struct Registered<H: HttpSend> {
    base: String,
    client: Client,
    client_id: String,
    client_secret: String,
    redirect: String,
    scopes: Scopes,
    http_sender: H,
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
        r.scopes(Scopes::All);

        assert_eq!(r.base, "https://example.com".to_string());
        assert_eq!(&mut r.app_builder, AppBuilder::new().scopes(Scopes::All));
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
