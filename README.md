[![License](https://img.shields.io/crates/l/tower-request-id.svg)](https://choosealicense.com/licenses/mit/)
[![Crates.io](https://img.shields.io/crates/v/tower-request-id.svg)](https://crates.io/crates/tower-request-id)
[![Docs.rs](https://docs.rs/tower-request-id/badge.svg)](https://docs.rs/tower-request-id)

# tower-request-id

<!-- cargo-sync-readme start -->

A tiny [tower] ([hyper], [axum], [warp] etc) service to generate a random id for each
incoming request.

## Usage

After adding the [`RequestIdLayer`] into the [axum] middlewares the request id is available in
the [`http::Request::extensions()`]. For the [tracing] integration, please refer to the
[logging example].

[tower]: https://crates.io/crates/tower
[hyper]: https://crates.io/crates/hyper
[axum]: https://crates.io/crates/axum
[warp]: https://crates.io/crates/warp
[tracing]: https://crates.io/crates/tracing
[`Request.extensions()`]: https://docs.rs/http/0.2.5/http/request/struct.Request.html#method.extensions
[logging example]: https://github.com/imbolc/tower-request-id/blob/main/examples/logging.rs

<!-- cargo-sync-readme end -->

## Contributing

We appreciate all kinds of contributions, thank you!


### Note on README

Most of the readme is automatically copied from the crate documentation by [cargo-sync-readme][].
This way the readme is always in sync with the docs and examples are tested.

So if you find a part of the readme you'd like to change between `<!-- cargo-sync-readme start -->`
and `<!-- cargo-sync-readme end -->` markers, don't edit `README.md` directly, but rather change
the documentation on top of `src/lib.rs` and then synchronize the readme with:
```bash
cargo sync-readme
```
(make sure the cargo command is installed):
```bash
cargo install cargo-sync-readme
```

If you have [rusty-hook] installed the changes will apply automatically on commit.


## License

This project is licensed under the [MIT license](LICENSE).

[cargo-sync-readme]: https://github.com/phaazon/cargo-sync-readme
[rusty-hook]: https://github.com/swellaby/rusty-hook
