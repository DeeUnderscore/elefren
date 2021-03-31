#![cfg_attr(not(feature = "toml"), allow(dead_code))]
#![cfg_attr(not(feature = "toml"), allow(unused_imports))]
#[macro_use]
extern crate pretty_env_logger;
extern crate elefren;
mod register;

use elefren::MediaBuilder;
use register::MastodonClient;
use std::error;

#[cfg(feature = "toml")]
fn main() -> Result<(), Box<error::Error>> {
    let mastodon = register::get_mastodon_data()?;
    let input = register::read_line("Enter the path to the photo you'd like to post: ")?;
    let description = register::read_line("Enter the image description: ")?;

    let builder = MediaBuilder {
        description: Some(description),
        ..MediaBuilder::from_file(input.into())
    };

    let attachment = mastodon.media(builder)?;

    println!("{:#?}", attachment);

    Ok(())
}

#[cfg(not(feature = "toml"))]
fn main() {
    println!(
        "examples require the `toml` feature, run this command for this example:\n\ncargo run \
         --example upload_photo --features toml\n"
    );
}
