extern crate google_calendar3;

extern crate hyper;
extern crate hyper_rustls;
extern crate yup_oauth2;

use yup_oauth2::{ApplicationSecret, Authenticator, DefaultAuthenticatorDelegate, MemoryStorage};
use google_calendar3::{Channel, Result, Error, CalendarHub};

fn main() {
    let secret: ApplicationSecret = Default::default();

    let auth = Authenticator::new(
        &secret,
        DefaultAuthenticatorDelegate,
        hyper::Client::with_connector(hyper::net::HttpsConnector::new(
            hyper_rustls::TlsClient::new(),
        )),
        <MemoryStorage as Default>::default(),
        None,
    );

    let mut hub = CalendarHub::new(
        hyper::Client::with_connector(hyper::net::HttpsConnector::new(
            hyper_rustls::TlsClient::new(),
        )),
        auth,
    );

    let mut req = Channel::default();

    let result = hub.events().watch(req, "calendarID").doit();

    if let Ok(res) = result {
        println!("request successfull");
    }
}
