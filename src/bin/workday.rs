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

    let country = env::args().nth(1).expect("country");
    let start = env::args().nth(2).expect("start");
    let days = env::args().nth(3).expect("days");

    match client.workday(&country, &start, &days) {
        Err(e) => eprintln!("{:?}", e),
        Ok(workday) =>{
            match wd {
                None => println!("No workday!"),
                Some(h) => println!(h),
            }          
        }
    }
}