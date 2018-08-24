#![cfg_attr(not(feature = "toml"), allow(dead_code))]
#![cfg_attr(not(feature = "toml"), allow(unused_imports))]
extern crate elefren;

pub use self::elefren::{Data, MastodonClient};

use std::{error::Error, io};

use self::elefren::{
    apps::{App, Scopes},
    Mastodon,
    Registration,
};

#[cfg(feature = "toml")]
use self::elefren::data::toml;

#[allow(dead_code)]
#[cfg(feature = "toml")]
fn main() -> Result<(), Box<Error>> {
    register()?;
    Ok(())
}

#[allow(dead_code)]
#[cfg(feature = "toml")]
pub fn get_mastodon_data() -> Result<Mastodon, Box<Error>> {
    if let Ok(data) = toml::from_file("mastodon-data.toml") {
        Ok(Mastodon::from(data))
    } else {
        register()
    }
}

#[cfg(feature = "toml")]
pub fn register() -> Result<Mastodon, Box<Error>> {
    let mut app = App::builder();
    app.client_name("elefren-examples")
        .scopes(Scopes::All)
        .website("https://github.com/pwoolcoc/elefren");

    let website = read_line("Please enter your mastodon instance url:")?;
    let registration = Registration::new(website.trim());
    let registered = registration.register(app)?;
    let url = registered.authorize_url()?;

    println!("Click this link to authorize on Mastodon: {}", url);
    let code = read_line("Paste the returned authorization code: ")?;

    let mastodon = registered.complete(code)?;

    // Save app data for using on the next run.
    toml::to_file(&*mastodon, "mastodon-data.toml")?;

    Ok(mastodon)
}

#[cfg(feature = "toml")]
pub fn read_line(message: &str) -> Result<String, Box<Error>> {
    println!("{}", message);

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    Ok(input.trim().to_string())
}

#[cfg(not(feature = "toml"))]
fn main() { }
