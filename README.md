# cf-turnstile

A Rust client for [Cloudflare Turnstile].


# Example
```rust,no_run
use cf_turnstile::{SiteVerifyRequest, TurnstileClient};

let client = TurnstileClient::new("my-secret".to_string().into());

let validated = client.siteverify(SiteVerifyRequest {
   response: "myresponse".to_string(),
  ..Default::default()
}).await?;

assert!(validated.success);
```

## Features

### Idempotency

To utilize Turnstile's [`indempotency_key`](https://developers.cloudflare.com/turnstile/get-started/server-side-validation/#:~:text=A%20response%20may,for%20that%20response.) feature, you can enable the `idempotency` feature flag.  

This will enable the `idempotency_key` field on the [`SiteVerifyRequest`](struct.SiteVerifyRequest.html) struct and the [`generate_indepotency_key`](fn.generate_indepotency_key.html) function.

### TLS

**Note**: not enabling any TLS feature is supported for use behind a proxy;
Turnstile's API is HTTPS only.

**Note**: this TLS code was taken from [twilight-http](https://github.com/twilight-rs/twilight/tree/main/twilight-http) in accodance with its license.

`cf-turnstile` has features to enable HTTPS connectivity with [`hyper`]. These
features are mutually exclusive. `rustls-native-roots` is enabled by default.

#### `native`

The `native` feature uses a HTTPS connector provided by [`hyper-tls`].

To enable `native`, do something like this in your `Cargo.toml`:

```toml
[dependencies]
cf-turnstile = { default-features = false, features = ["native"], version = "0.1" }
```

#### `rustls-native-roots`

The `rustls-native-roots` feature uses a HTTPS connector provided by [`hyper-rustls`], which uses
[`rustls`] as the TLS backend, and enables its `native-tokio` feature, which uses [`rustls-native-certs`]
for root certificates.

This is enabled by default.

#### `rustls-webpki-roots`

The `rustls-webpki-roots` feature uses a HTTPS connector provided by [`hyper-rustls`], which uses
[`rustls`] as the TLS backend, and enables its `webpki-tokio` feature, which uses [`webpki-roots`]
for root certificates.

This should be preferred over `rustls-native-roots` in Docker containers based on `scratch`.

### Trust-DNS

The `trust-dns` enables [`hyper-trust-dns`], which replaces the default
`GaiResolver` in [`hyper`]. [`hyper-trust-dns`] instead provides a fully
async DNS resolver on the application level.

[Cloudflare Turnstile]: https://developers.cloudflare.com/turnstile/
[`hyper`]: https://crates.io/crates/hyper
[`hyper-rustls`]: https://crates.io/crates/hyper-rustls
[`hyper-tls`]: https://crates.io/crates/hyper-tls
[`rustls`]: https://crates.io/crates/rustls
[`rustls-native-certs`]: https://crates.io/crates/rustls-native-certs
[`hyper-trust-dns`]: https://crates.io/crates/hyper-trust-dns
[`webpki-roots`]: https://crates.io/crates/webpki-roots
