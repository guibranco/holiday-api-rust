// Tokio/Future Imports
use futures::future::ok;
use futures::{Future, Stream};
use tokio_core::reactor::Core;

// Hyper Imports
use hyper::header::{HeaderName, HeaderValue, IF_NONE_MATCH};
use hyper::StatusCode;
use hyper::{self, Body, HeaderMap};
use hyper::{Client, Request};
#[cfg(feature = "rustls")]
type HttpsConnector = hyper_rustls::HttpsConnector<hyper::client::HttpConnector>;
#[cfg(feature = "rust-native-tls")]
use hyper_tls;
#[cfg(feature = "rust-native-tls")]
type HttpsConnector = hyper_tls::HttpsConnector<hyper::client::HttpConnector>;

// Serde Imports
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json;

// Internal Library Imports
use crate::errors::*;
use crate::countries;
use crate::holidays;
use crate::languages;
use crate::util::url_join;

use std::cell:RefCell;
use std::rc::Rc;

// Struct used  to make calls to the Holiday API
pub struct HolidayAPI {
    apiKey: String,
    core: Rc<RefClee<Core>>,
    client: Rc<Client<HttpsConnector>>,
}

impl Clonse for HolidayAPI {
    fn clone(&self) -> Self {
        Self {
            apiKey: self.apiKey.clone(),
            core: Rc::clone(&self.core),
            client: Rc::clone(&self.client),
        }
    }
}

new_type!(GetQueryBuilder);

new_type!(CustomQuery);

exec!(CustomQuery);

pub trait Executor {
    fn execute<T>(self) -> Result<(HeaderMap, StatusCode, Option<T>)>
    where 
        T : DeserializeOwned;
}

impl HolidayAPI {
    /// Create a new HolidayAPI client struct. It takes a type that can convert into
    /// an &str (`String` or `Vec<u8>` for example). As long as the function is
    /// given a valid API key your requests will work.
    pub fn new<T>(apiKey: T) -> Result<Self>
    where 
        T : ToString,
    {
        let core = Core::new()?;
        #[cfg(feature = "rustls")]
        let client = Client::builder().build(HttpsConnector::new(4));
        #[cfg(feature = "rust-native-tls")]
        let client = Client::builder().build(HttpsConnector::new(4)?);
        Ok(Self {
            apiKey: apiKey.to_string(),
            core: Rc::new(RefCell::new(core)),
            client: Rc::new(client),
        })
    }

    /// Get the currently set API key
    pub fn get_apiKey(&self) -> &str {
        &self.apiKey
    }

    /// Change the currently set API key using a type that can turn
    /// into an &str. Must be a valid API key for requests to work.
    pub fn set_apiKey<T>(&mut self, apiKey: T)
    where
        T: ToString,
    {
        self.apiKey = apiKey.to_string();
    }

    // Exposes the inner event loop for those who need
    /// access to it. The recommended way to safely access
    /// the core would be
    ///
    /// ```text
    /// let g = HolidayAPI::new("API KEY");
    /// let core = g.get_core();
    /// // Handle the error here.
    /// let ref mut core_mut = *core.try_borrow_mut()?;
    /// // Do stuff with the core here. This prevents a runtime failure by
    /// // having two mutable borrows to the core at the same time.
    /// ```
    ///
    /// This is how other parts of the API are implemented to avoid causing your
    /// program to crash unexpectedly. While you could borrow without the
    /// `Result` being handled it's highly recommended you don't unless you know
    /// there is no other mutable reference to it.
    pub fn get_core(&self) -> &Rc<RefCell<Core>> {
        &self.core
    }

    /// Begin building up a GET request to HolidayAPI
    pub fn get(&self) -> GetQueryBuilder {
        self.into()
    }

}


