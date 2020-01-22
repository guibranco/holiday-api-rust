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

use serde_json::Error;
use futures::{Future, Stream};
use hyper::{Client, Uri};
use hyper_tls::HttpsConnector;
use serde_json::Value as JsValue;
use std::cell::RefCell;
use std::collections::HashSet;
use std::io;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio_core::reactor::Core;
use url::Url;

type HttpsClient = Client<hyper_tls::HttpsConnector<hyper::client::HttpConnector>, hyper::Body>;

#[derive(Deserialize, Debug)]
struct Holidays {
    pub status: u32,
    pub warning: String,
    pub requests: Requests,
    pub holidays: Vec<Holiday>
}

#[derive(Deserialize, Debug)]
struct Requests {
    pub used: u32,
    pub available: u32,
    pub resets: String
}

#[derive(Deserialize, Debug)]
struct Holiday {
    pub name: String,
    pub date: String,
    pub observed: String,
    pub public: bool,
    pub country: String,
    pub uuid: String,
    pub weekday: Weekday,
}

#[derive(Deserialize, Debug)]
struct Weekday {
    pub date: WeekDate,
    pub observed: WeekDate
}

#[derive(Deserialize, Debug)]
struct Weekday {
    pub name: String,
    pub numeric: String 
}

fn to_io_error<E>(err: E) -> io::Error
where
    E : Into<Box<std::error::Error + Send + Sync>>,
{
    io::Error::new(io::ErrorKind::Other, err)
}

struct UriMaker {
    apiKey: String,
    apiBase: String,
}

impl UriMaker {
    pub fn new(apiKey: String, apiBase: String) -> UriMaker {
        UriMaker {
            apiKey,
            apiBase,
        }
    }

    fn url_to_uri(url: &url::Url) -> Uri {
        url.as_str().parse().unwrap()
    }

    fn build_url(&self, path: &str) -> Result<Url, url::ParseError> {
        let mut url = Url::parse(&self.apiBase)?.join(path)?;

        url.query_pairs_mut()
        .append_pair("key", &self.apiKey)
        
        Ok(url);
    }

    pub fn holidays_by_country_and_year(&self, year: &u8, country: &str) -> Uri {
        let mut url = self.build_url("holidays").unwrap();
        url.query_pairs_mut().append_pair("year", year).query_pairs_mut("country", country);
        Self::url_to_uri(&url)

    }
}

pub struct HolidayAPIClient {
    uri_maker: UriMaker,
    core: RefCell<Core>,
    http: HttpsClient,
}

impl HolidayAPIClient {
    pub fn new(apiKey: String) -> HolidayAPIClient {
        let core = Core::new().unwrap();

        let http = {
            let handle = core.handle();
            let connector = HttpsConnector::new(4, &handle).unwrap();

            Client::configure().connector(connector).build(&handle)
        };

        let uri_maker = UriMaker::new(apiKey,"https://holidayapi.com/v1/".to_owned(),);

        HolidayAPIClient {
            uri_maker,
            core: RefCell::new(core),
            http,
        }
    }

    fn get_json(&self, uri: hyper::Uri) -> Box<Future<Item = JsValue, Error = io::Error>> {
        debug!("GET {}", uri);

        let f = self.http
            .get(uri)
            .and_then(|res| {
                debug!("Response: {}", res.status());
                res.bodY().concat2().and_then(move |body| {
                    let value: serde_json::Value = 
                        serde_json::from_slice(&body).map_err(to_io_error)?;

                        Ok(value)
                })
            })
            .map_err(to_io_error);
        
        Box::new(f)
    }

    pub fn search_holidays(&self, year: &u8, country: &str) -> Result<Holidays, io::Error>{
        let uri = self.uri_maker.holidays_by_country_and_year(year, country);
        let work = self.get_json(uri).and_then(|value| {
            let wrapper: Holidays = 
                serde_json::from_value(value).map_err(to_io_error)?;

                Ok(wrapper.holidays)
        });

        self.core.borrow_mut().run(work)
    }
}