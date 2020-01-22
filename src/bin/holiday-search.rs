extern crate dotenv;
extern crate holiday_api_rust;
#[macro_use]
extern crate prettytable;

use dotenv::dotenv;
use holiday_api_rust::HolidayAPIClient;
use prettytable::Table;
use prettytable::format;
use std::env;

fn main(){
    dotenv().ok();

    let api_key = env::var("HOLIDAYAPI_APIKEY").unwrap();
    
    let client = HolidayAPIClient::new(api_key);

    let year = env::args().nth(1).expect("year");
    let country = env::args().nth(2).expect("country");

    match client.search_holidays(&year, &country) {
        Err(e) => eprintln!("{:?}", e),
        Ok(holidays) =>{
            let mut table = Table::new();
            table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
            table.set_titles(row!["Name", "Date", "Country"]);

            for holiday in holidays {
                table.add_row(row![holiday.name, holiday.date, holiday.country]);
            }
            table.printstd();
        }
    }
}