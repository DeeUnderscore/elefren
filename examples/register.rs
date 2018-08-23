extern crate elefren;
extern crate toml;

pub use self::elefren::{Data, MastodonClient};

use std::{error::Error, fs, io};

use self::elefren::{
    apps::{App, Scopes},
    Mastodon,
    Registration,
};

#[allow(dead_code)]
fn main() -> Result<(), Box<Error>> {
    register()?;
    Ok(())
}

#[allow(dead_code)]
pub fn get_mastodon_data() -> Result<Mastodon, Box<Error>> {
    if let Ok(config) = fs::read_to_string("mastodon-data.toml") {
        let data: Data = toml::from_str(&config)?;
        Ok(Mastodon::from(data))
    } else {
        register()
    }
}

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
    let input = read_line("Paste the returned authorization code: ")?;

    let code = input.trim();
    let mastodon = registered.complete(code.to_string())?;

    // Save app data for using on the next run.
    let toml = toml::to_string(&*mastodon)?;
    fs::write("mastodon-data.toml", toml.as_bytes())?;

    Ok(mastodon)
}

pub fn read_line(message: &str) -> Result<String, Box<Error>> {
    println!("{}", message);

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    Ok(input)
}
