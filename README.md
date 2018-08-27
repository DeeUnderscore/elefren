# Elefren

## A Wrapper for the Mastodon API.

[![Build Status](https://travis-ci.org/pwoolcoc/elefren.svg?branch=master)](https://travis-ci.org/pwoolcoc/elefren)
[![Build Status](https://ci.appveyor.com/api/projects/status/qeigk3nmmps52wxv?svg=true)](https://ci.appveyor.com/project/pwoolcoc/elefren)
[![Coverage Status](https://coveralls.io/repos/github/pwoolcoc/elefren/badge.svg?branch=master&service=github)](https://coveralls.io/github/pwoolcoc/elefren?branch=master)
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

```rust,ignore
extern crate elefren;
```

## Example

```rust,no_run
extern crate elefren;

use std::io;
use std::error::Error;

use elefren::prelude::*;
use elefren::helpers::toml; // requires `features = ["toml"]`

fn main() -> Result<(), Box<Error>> {
    let mastodon = if let Ok(data) = toml::from_file("mastodon-data.toml") {
        Mastodon::from(data)
    } else {
        register()?
    };

    let you = mastodon.verify_credentials()?;

    println!("{:#?}", you);

    Ok(())
}

fn register() -> Result<Mastodon, Box<Error>> {
    let registration = Registration::new("https://mastodon.social")
                                    .client_name("elefren-examples")
                                    .build()?;
    let url = registration.authorize_url()?;

    println!("Click this link to authorize on Mastodon: {}", url);
    println!("Paste the returned authorization code: ");

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let code = input.trim().to_string();
    let mastodon = registration.complete(&code)?;

    // Save app data for using on the next run.
    toml::to_file(&*mastodon, "mastodon-data.toml")?;

    Ok(mastodon)
}
