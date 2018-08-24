use reqwest::{Client, RequestBuilder, Response};
use try_from::TryInto;

use apps::{App, Scopes};
use Data;
use Error;
use Mastodon;
use MastodonBuilder;
use Result;
use http_send::{HttpSend, HttpSender};

/// Handles registering your mastodon app to your instance. It is recommended
/// you cache your data struct to avoid registering on every run.
pub struct Registration<H: HttpSend> {
    base: String,
    client: Client,
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
    "urn:ietf:wg:oauth:2.0:oob".to_string()
}

#[derive(Deserialize)]
struct AccessToken {
    access_token: String,
}

impl Registration<HttpSender> {
    /// Construct a new registration process to the instance of the `base` url.
    /// ```
    /// use elefren::apps::prelude::*;
    ///
    /// let registration = Registration::new("https://mastodon.social");
    /// ```
    pub fn new<I: Into<String>>(base: I) -> Self {
        Registration {
            base: base.into(),
            client: Client::new(),
            http_sender: HttpSender,
        }
    }
}

impl<H: HttpSend> Registration<H> {
    #[allow(dead_code)]
    pub(crate) fn with_sender<I: Into<String>>(base: I, http_sender: H) -> Self {
        Registration {
            base: base.into(),
            client: Client::new(),
            http_sender: http_sender,
        }
    }

    fn send(&self, req: &mut RequestBuilder) -> Result<Response> {
        Ok(self.http_sender.send(
                &self.client,
                req
        )?)
    }

    /// Register the application with the server from the `base` url.
    ///
    /// ```no_run
    /// # extern crate elefren;
    /// # fn main () -> elefren::Result<()> {
    /// use elefren::{apps::prelude::*, prelude::*};
    ///
    /// let mut builder = App::builder();
    /// builder.client_name("elefren_test");
    /// let app = builder.build()?;
    ///
    /// let registration = Registration::new("https://mastodon.social");
    /// let registered = registration.register(app)?;
    /// let url = registered.authorize_url()?;
    /// // Here you now need to open the url in the browser
    /// // And handle a the redirect url coming back with the code.
    /// let code = String::from("RETURNED_FROM_BROWSER");
    /// let mastodon = registered.complete(code)?;
    ///
    /// println!("{:?}", mastodon.get_home_timeline()?.initial_items);
    /// # Ok(())
    /// # }
    /// ```
    pub fn register<I: TryInto<App>>(self, app: I) -> Result<Registered<H>>
    where
        Error: From<<I as TryInto<App>>::Err>,
    {
        let app = app.try_into()?;
        let url = format!("{}/api/v1/apps", self.base);
        let oauth: OAuth = self.send(
                self.client.post(&url).form(&app)
        )?.json()?;

        Ok(Registered {
            base: self.base,
            client: self.client,
            client_id: oauth.client_id,
            client_secret: oauth.client_secret,
            redirect: oauth.redirect_uri,
            scopes: app.scopes(),
            http_sender: self.http_sender,
        })
    }
}

impl<H: HttpSend> Registered<H> {
    fn send(&self, req: &mut RequestBuilder) -> Result<Response> {
        Ok(self.http_sender.send(
                &self.client,
                req
        )?)
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
    pub fn complete(self, code: String) -> Result<Mastodon<H>> {
        let url = format!(
            "{}/oauth/token?client_id={}&client_secret={}&code={}&grant_type=authorization_code&redirect_uri={}",
            self.base,
            self.client_id,
            self.client_secret,
            code,
            self.redirect
        );

        let token: AccessToken = self.send(
                &mut self.client.post(&url)
        )?.json()?;

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
