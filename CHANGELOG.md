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
