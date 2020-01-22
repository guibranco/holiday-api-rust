# holiday-api-rust
The [HolidayAPI](https://holidayapi.com/docs) client wrapper written in Rust

| Service      | Status |
| -------      | :----: |
| AppveyorCI   | [![Build status](https://ci.appveyor.com/api/projects/status/4ksqycqm761c06jb?svg=true)](https://ci.appveyor.com/project/guibranco/holiday-api-rust/branch/master)       |
| CodeCov   | [![codecov](https://codecov.io/gh/guibranco/holiday-api-rust/branch/master/graph/badge.svg)](https://codecov.io/gh/guibranco/holiday-api-rust)      |
| crates.io | [![crates.io](https://img.shields.io/crates/v/holiday-api-rust.svg)](https://crates.io/crates/holiday-api-rust) |

Pure Rust bindings to the [Holiday API](https://holidayapi.com).

## Dependencies and support

holiday-api-rust is intended to work on all tier 1 supported Rust systems:

- MacOSX
- Linux
- Windows

holiday-api-rust supports [rustls] and [rust-native-tls] for TLS connectivity.
`rustls` is used by default, but one can toggle support with Cargo features:

[rustls]: https://github.com/ctz/rustls
[rust-native-tls]: https://github.com/sfackler/rust-native-tls
[ring]: https://github.com/briansmith/ring

## Minimum Compiler Version

Due to the use of certain features github-rs requires `rustc` version 1.18 or
higher.

## Getting Started

Add the following to your `Cargo.toml`

```toml
[dependencies]
holiday-api-rust = "1.0"
serde_json = "1.0"
```

Then in your `lib.rs` or `main.rs` file add:

```rust
use holiday_api::client::{Executor, Github};
use serde_json::Value;
```

Now you can start making queries. Here's a small example to get supported languages by the Holiday API:

```rust
use gholiday_api::client::{Executor, HolidayApi};
use serde_json::Value;

fn main() {
    let client = HolidayApi::new("API TOKEN").unwrap();
    let holidays = client.get()
                   .holidays("br", 2019)
                   .execute::<Value>();
    match holidays {
        Ok((headers, status, json)) => {
            println!("{:#?}", headers);
            println!("{}", status);
            if let Some(json) = json{
                println!("{}", json);
            }
        },
        Err(e) => println!("{}", e)
    }
}
```

## License

Licensed under

- MIT license ([LICENSE](LICENSE) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))

This library is based on the [github-rs](https://github.com/github-rs/github-rs) project.
