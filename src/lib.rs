extern crate futures;
extern crate hyper;
extern crate hyper_tls;
#[macro_use]
extern crate log;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate tokio_core;
extern crate url;

use futures::{Future, Stream};
use hyper::{Client, Uri};
use hyper_tls::HttpsConnector;
use serde_json::Value as JsValue;
use std::cell::RefCell;
use std::io;
use tokio_core::reactor::Core;
use url::Url;

type HttpsClient = Client<hyper_tls::HttpsConnector<hyper::client::HttpConnector>, hyper::Body>;

#[derive(Deserialize, Debug)]
pub struct Holidays {
    pub status: u32,
    pub warning: String,
    pub requests: Requests,
    pub holidays: Vec<Holiday>
}

#[derive(Deserialize, Debug)]
pub struct Requests {
    pub used: u32,
    pub available: u32,
    pub resets: String
}

#[derive(Deserialize, Debug)]
pub struct Holiday {
    pub name: String,
    pub date: String,
    pub observed: String,
    pub public: bool,
    pub country: String,
    pub uuid: String,
    pub weekday: Weekday,
}

#[derive(Deserialize, Debug)]
pub struct Weekday {
    pub date: WeekDate,
    pub observed: WeekDate
}

#[derive(Deserialize, Debug)]
pub struct WeekDate {
    pub name: String,
    pub numeric: String 
}

fn to_io_error<E>(err: E) -> io::Error
where
    E : Into<Box<dyn std::error::Error + Send + Sync>>,
{
    io::Error::new(io::ErrorKind::Other, err)
}

struct UriMaker {
    api_key: String,
    api_base: String,
}

impl UriMaker {
    pub fn new(api_key: String, api_base: String) -> UriMaker {
        UriMaker {
            api_key,
            api_base,
        }
    }

    fn url_to_uri(url: &url::Url) -> Uri {
        url.as_str().parse().unwrap()
    }

    fn build_url(&self, path: &str) -> Result<Url, url::ParseError> {
        let mut url = Url::parse(&self.api_base)?.join(path)?;

        url.query_pairs_mut()
        .append_pair("key", &self.api_key);
        
        Ok(url)
    }

    pub fn holidays_by_country_and_year(&self, year: &str, country: &str) -> Uri {
        let mut url = self.build_url("holidays").unwrap();
        url.query_pairs_mut().append_pair("year", year).append_pair("country", country);
        Self::url_to_uri(&url)

    }
}

pub struct HolidayAPIClient {
    uri_maker: UriMaker,
    core: RefCell<Core>,
    http: HttpsClient,
}

impl HolidayAPIClient {
    pub fn new(api_key: String) -> HolidayAPIClient {
        let core = Core::new().unwrap();

        let http = {
            let handle = core.handle();
            let connector = HttpsConnector::new(4, &handle).unwrap();

            Client::configure().connector(connector).build(&handle)
        };

        let uri_maker = UriMaker::new(api_key,"https://holidayapi.com/v1/".to_owned(),);

        HolidayAPIClient {
            uri_maker,
            core: RefCell::new(core),
            http,
        }
    }

    fn get_json(&self, uri: hyper::Uri) -> Box<dyn Future<Item = JsValue, Error = io::Error>> {
        debug!("GET {}", uri);

        let f = self.http
            .get(uri)
            .and_then(|res| {
                debug!("Response: {}", res.status());
                res.body().concat2().and_then(move |body| {
                    let value: serde_json::Value = 
                        serde_json::from_slice(&body).map_err(to_io_error)?;

                        Ok(value)
                })
            })
            .map_err(to_io_error);
        
        Box::new(f)
    }

    pub fn search_holidays(&self, year: &str, country: &str) -> Result<Vec<Holiday>, io::Error>{
        let uri = self.uri_maker.holidays_by_country_and_year(year, country);
        let work = self.get_json(uri).and_then(|value| {
            let wrapper: Holidays = 
                serde_json::from_value(value).map_err(to_io_error)?;

                Ok(wrapper.holidays)
        });

        self.core.borrow_mut().run(work)
    }
}