# 0.13 (2018/08/27)

## Features

- `Registration` now duplicates the `AppBuilder` API, so you can
  replace:

  ```
  let app = App::builder();
  app.client_name("test-client");

  let registration = Registration::new("http://example.com")
                                  .register(app)?;
  ```

  with this:

  ```
  let registration = Registration::new("http://example.com")
                                  .client_name("test-client")
                                  .build()?;
  ```

  You can still call use the `Registration` & `AppBuilder` APIs like
  before, but any App passed to `.register` will supercede anything app
  config set on the `Registration` object itself.

  In future releases, this will become a hard error.

- `elefren::status_builder::StatusBuilder::new()` now takes anything
  that implements `Display` instead of specifically an owned `String`

## Breaking Changes

- The `elefren::data::toml` module has been moved to
  `elefren::helpers::toml`
- Because of the changes to `Registration`, the `elefren::apps::prelude`
  module has been removed. The types that are still necessary from that
  prelude have been moved to `elefren::prelude`, but
  `elefren::apps::App` will have to be imported separately
- `elefren::entities::account::CredientialsBuilder` has been moved to
  `elefren::entities::account::CredentialsBuilder` (note the spelling
  difference)
- `Registered::complete` now takes a `&str` instead of a `String`

## Compatibility

- `elefren::entities::instance::Instance` now has the `max_toot_chars`
  property, for use with the Pleroma and Glitch-soc APIs

# 0.12 (2018/08/23)

## Features

- `Page::items_iter` added, abstracts over "next_page" logic
- `elefen::prelude` and `elefen::apps::prelude` modules added to more
  easily import all necessary types
- Helpers added to allow serialization & deseriasization of `Data` types
  to `toml`

## Breaking Changes

- Combined all parameters to `mastodon.statuses` (except `id`) to their
  own type, `StatusesRequest`
- All API methods on `Mastodon` type moved into a trait,
  `MastodonClient`, to enable better mocking during tests
- `Mastodon::from_data(Data)` was changed to `Mastodon::from(Data)`
- `AppBuilder` was changed, use `App::builder()` instead
- `Registration` was broken up to enable better registration flow

## Documentation

- All API methods now have doc comments
- All docs were updated for the new breaking changes

## Compatibility

- Login to pleroma instances was fixed

# 0.11
- Added more examples to `examples` directory.
- Fixed `follow` and `unfollow` routes.
- Updated `moved` field to be `Box<Account>`.

# 0.10

- Added the ability to handle paged entities like favourites and such.(Only favourites in prerelease)
- Added optional `source` and `moved` fields to `Account`.
- Added `Source` struct to match with the `Account.source` field.
- Added `CredientialsBuilder` struct for updating profile using
  `verify_credientials`.
- Attachment now handles being sent an empty object, which is converted
  to `None`.
- Added ombed data fields to `Card`.
- Added `version` and `urls` fields to `Instance`.
- Added `id`, `muting_notifications`, and `domain_blocking` to `Relationship`.
- Added `emojis`, `language`, and `pinned` fields to `Status`
- Added `Emoji` struct.
- Added `List` and `Mention` structs(matching routes not added yet).
- Added example that prints your profile.
- Updated dependencies
