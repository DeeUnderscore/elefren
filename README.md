# Elefren

## A Wrapper for the Mastodon API.

[![Build Status](https://travis-ci.org/pwoolcoc/elefren.svg?branch=master)](https://travis-ci.org/pwoolcoc/elefren)
[![Coverage Status](https://coveralls.io/repos/github/pwoolcoc/elefren/badge.svg?branch=master)](https://coveralls.io/github/pwoolcoc/elefren?branch=master)
[![crates.io](https://img.shields.io/crates/v/elefren.svg)](https://crates.io/crates/elefren)
[![Docs](https://docs.rs/elefren/badge.svg)](https://docs.rs/elefren)
[![MIT/APACHE-2.0](https://img.shields.io/crates/l/elefren.svg)](https://crates.io/crates/elefren)

[Documentation](https://docs.rs/elefren/)

A wrapper around the [API](https://github.com/tootsuite/documentation/blob/master/docs/Using-the-API/API.md#tag) for [Mastodon](https://mastodon.social/)

## Installation

To add `elefren` to your project, add the following to the
`[dependencies]` section of your `Cargo.toml`

```toml
elefren = "0.12"
```

## Usage

To use this crate in your project, add this to your crate root (lib.rs, main.rs, etc):

```rust
extern crate elefren;
```

## Example

```rust
extern crate elefren;

use std::io;
use std::fs::File;
use std::io::prelude::*;

use elefren::{Data, Mastodon, Registration};
use elefren::apps::{AppBuilder, Scopes};
use elefren::data::toml; // requires `features = ["toml"]`

fn main() {
    let mastodon = match toml::from_file("mastodon-data.toml") {
        Ok(data) => {
            Mastodon::from(data)
        },
        Err(_) => register(),
    };

    let you = mastodon.verify_credentials().unwrap();

    println!("{:#?}", you);
}

fn register() -> Mastodon {
    let mut app = App::builder();
    app.client_name("elefren-examples");

    let registration = Registration::new("https://mastodon.social");
                                    .register(app).unwrap();
    let url = registration.authorize_url().unwrap();

    println!("Click this link to authorize on Mastodon: {}", url);
    println!("Paste the returned authorization code: ");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let code = input.trim().to_string();
    let mastodon = registration.complete(code).unwrap();

    // Save app data for using on the next run.
    toml::to_file(&*mastodon, "mastodon-data.toml").unwrap();

    mastodon
}
```
