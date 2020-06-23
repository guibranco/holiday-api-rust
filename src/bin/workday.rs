extern crate dotenv;
extern crate holiday_api_rust;

use dotenv::dotenv;
use holiday_api_rust::HolidayAPIClient;
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
        Ok(workday) => println!("Workday: {}", workday.date)
    }
}