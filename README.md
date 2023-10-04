# HolidayAPI Rust client

📆⚙️ [HolidayAPI](https://holidayapi.com/docs) client wrapper for Rust projects.

![GitHub last commit (branch)](https://img.shields.io/github/last-commit/guibranco/holiday-api-rust/main)
![Crates.io](https://img.shields.io/crates/d/holiday-api-rust)
[![wakatime](https://wakatime.com/badge/github/guibranco/holiday-api-rust.svg)](https://wakatime.com/badge/github/guibranco/holiday-api-rust)

[![Maintainability](https://api.codeclimate.com/v1/badges/392b044637f43eb881ac/maintainability)](https://codeclimate.com/github/guibranco/holiday-api-rust/maintainability)
[![Test Coverage](https://api.codeclimate.com/v1/badges/392b044637f43eb881ac/test_coverage)](https://codeclimate.com/github/guibranco/holiday-api-rust/test_coverage)
[![CodeFactor](https://www.codefactor.io/repository/github/guibranco/holiday-api-rust/badge)](https://www.codefactor.io/repository/github/guibranco/holiday-api-rust)

| Service      | Status |
| -------      | :----: |
| AppVeyor CI   | [![Build status](https://ci.appveyor.com/api/projects/status/4ksqycqm761c06jb/branch/main?svg=true)](https://ci.appveyor.com/project/guibranco/holiday-api-rust/branch/main) |
| crates.io    | [![Crates.io](https://img.shields.io/crates/v/holiday-api-rust.svg)](https://crates.io/crates/holiday-api-rust) |

Pure Rust bindings to the [Holiday API](https://holidayapi.com).

## Dependencies and support

`holiday-api-rust` is intended to work on all tier 1 supported Rust systems:

- MacOSX
- Linux
- Windows

## Minimum Compiler Version

Due to the use of certain features `holiday-api-rust` requires `rustc` version 1.18 or
higher.

## Getting Started

Add the following to your `Cargo.toml`

```toml
[dependencies]
holiday_api_rust = "0.3.1"
serde_json = "1.0"
```

Then in your `lib.rs` or `main.rs` file add:

```rust
extern crate holiday_api_rust;

let client = HolidayAPIClient::new("HolidayAPI key here");
match client.search_holidays("2019", "BR") {
    Err(e) => eprintln!("{:?}", e),
    Ok(holidays) => {
        for holiday in holidays {
            println!("Holiday: {} | Date: {} | Country: {}", holiday.name, holiday.date, holiday.country);
        }
    }
}
```

## License

Licensed under

- MIT license ([LICENSE](https://github.com/guibranco/holiday-api-rust/blob/main/LICENSE) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))
