# HolidayAPI Rust client

The [HolidayAPI](https://holidayapi.com/docs) client wrapper written in Rust. ![Crates.io](https://img.shields.io/crates/d/holiday-api-rust)

| Service      | Status |
| -------      | :----: |
| AppveyorCI   | [![Build status](https://ci.appveyor.com/api/projects/status/4ksqycqm761c06jb?svg=true)](https://ci.appveyor.com/project/guibranco/holiday-api-rust/branch/master) |
| crates.io    | [![crates.io](https://img.shields.io/crates/v/holiday-api-rust.svg)](https://crates.io/crates/holiday-api-rust) |

Pure Rust bindings to the [Holiday API](https://holidayapi.com).

## Dependencies and support

holiday-api-rust is intended to work on all tier 1 supported Rust systems:

- MacOSX
- Linux
- Windows

## Minimum Compiler Version

Due to the use of certain features holiday-api-rust requires `rustc` version 1.18 or
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

- MIT license ([LICENSE](https://github.com/guibranco/holiday-api-rust/blob/master/LICENSE) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))
