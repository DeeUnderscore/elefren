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
elefren = "0.18"
```

## Usage

To use this crate in your project, add this to your crate root (lib.rs, main.rs, etc):

```rust,ignore
extern crate elefren;
```

## Example

```rust,no_run
extern crate elefren;

use std::error::Error;

use elefren::prelude::*;
use elefren::helpers::toml; // requires `features = ["toml"]`
use elefren::helpers::cli;

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
    let mastodon = cli::authenticate(registration)?;

    // Save app data for using on the next run.
    toml::to_file(&*mastodon, "mastodon-data.toml")?;

    Ok(mastodon)
}
```

## Relationship to [Mammut](https://github.com/Aaronepower/mammut)

This library was forked from Mammut around elefren commit
6c37ecb1e1ec3aa888711c1fad1a76f3bdf826b3.  I started this fork as a
place to experiment with some of the changes I wanted to make to Mammut,
and ended up diverging enough that I didn't really want to spend the time
trying to get everything merged back.

Some of the major differences between this and mammut are:

* Compatibility with newer mastodon versions. Elefren supports new API
  calls introduced in Mastodon 2.4 and 2.5, and 2.6 support is being
  worked on.
* Compatibility with other implementations of the Mastodon API. One
  design goal of elefren is to be compatible with as many
  implementations of the Mastodon API as possible. Currently, this means
  the GlitchSoc fork of Mastodon, and Pleroma. I do not guarantee that
  support is 100% there, but if you find any inconsistencies, it is a
  bug and I hope you can take the time to open an issue here
* Multiple bug fixes

I have a couple big features on the horizon, the first being a
comprehensive test suite that I'm currently working on, which should
help us test for inconsistencies and issues between the different
versions of the Mastodon API that elefren aims to support. After that, I
will be trying to get support for the Mastodon Streaming API merged in,
though that might need to wait until rust's async story is a little more
stable.

For a complete list of changes, see the
[Changelog](https://github.com/pwoolcoc/elefren/blob/master/CHANGELOG.md).
