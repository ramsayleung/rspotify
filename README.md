[![Continuous Integration](https://github.com/ramsayleung/rspotify/workflows/Continuous%20Integration/badge.svg)](https://github.com/ramsayleung/rspotify/actions)
[![License](https://img.shields.io/github/license/ramsayleung/rspotify)](https://github.com/ramsayleung/rspotify/blob/master/LICENSE)
[![Crates.io](https://img.shields.io/crates/v/rspotify.svg)](https://crates.io/crates/rspotify)
[![Docs](https://docs.rs/rspotify/badge.svg)](https://docs.rs/crate/rspotify/)

# Rspotify

Rspotify is a wrapper for the [Spotify Web API](https://developer.spotify.com/web-api/), inspired by [spotipy](https://github.com/plamere/spotipy). It includes support for all the [authorization flows](https://developer.spotify.com/documentation/general/guides/authorization-guide/), and helper functions for [all endpoints](https://developer.spotify.com/documentation/web-api/reference/).

To learn how to use Rspotify, please refer to the [documentation](https://docs.rs/crate/rspotify/). There are some [examples that may be useful](./examples) as well.

## Changelog

Please see the [changelog](./CHANGELOG.md) for a release history and indications on how to upgrade from one version to another.

## Contributing

If you find any problems or have suggestions about this crate, please submit an issue. Moreover, any pull request, code review and feedback are welcome.

## Building

Rspotify uses [`maybe_async`](https://docs.rs/maybe-async/0.2.0/maybe_async/) crate to switch between async and blocking client, which is triggered inside `Cargo.toml`, so there is something you need to pay attention to when you are trying to build `rspotify`

Build with `client-reqwest` feature, the `async` version, and this would compile Rspotify with [`reqwest`](https://docs.rs/reqwest/)

```sh
cargo build --features client-reqwest
```

Build with `client-ureq` feature, the `blocking` version, and this would compile Rspotify with [`ureq`](https://docs.rs/ureq/):

```sh
cargo build --no-default-features --features client-ureq
```

Noticed that you could not build `rspotify` with all features like this:

```sh
cargo build --all --all-features
```

Because in order to switch between different clients, the different clients have to implement the same methods, so if you build with all features, you'll get `duplicate definitions` error. As every coin has two sides, you could only have one side at a time, not all sides of it.

## License

[MIT](./LICENSE)
