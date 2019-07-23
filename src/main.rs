extern crate google_calendar3;

extern crate chrono;

extern crate hyper;
extern crate hyper_rustls;

extern crate reqwest;

extern crate serde;
extern crate serde_json;
extern crate yup_oauth2;
#[macro_use]
extern crate serde_derive;
extern crate select;

use google_calendar3::{CalendarHub, Channel, Error, Event, EventDateTime, Result};
use yup_oauth2::{
    ApplicationSecret, Authenticator, DefaultAuthenticatorDelegate, DiskTokenStorage, MemoryStorage,
};

use std::fs::File;
use std::io::Read;

use select::document::Document;
use select::predicate::{Attr, Class, Name, Predicate};

use chrono::NaiveDate;

#[derive(Deserialize)]
struct CredentialsFile {
    pub installed: ApplicationSecret,
}

struct MyEvent {
    pub title: String,
    pub date: NaiveDate,
}

type Hub = CalendarHub<
    hyper::Client,
    Authenticator<DefaultAuthenticatorDelegate, DiskTokenStorage, hyper::Client>,
>;


fn main() {
    let mut hub = init_hub();

    for my_event in get_thb_return_date() {
        println!("hey");
        add_calendar_event(&mut hub, &my_event);
    }
}

fn get_thb_return_date() -> Vec<MyEvent> {
    let url = "https://www.th-brandenburg.de/hochschule/termine-veranstaltungen/rahmentermine/";

    let query = "Rückmeldung";

    let body = reqwest::get(url).unwrap().text().unwrap();
    let document = Document::from(body.as_str());

    let mut tds = Vec::new();

    for div in document.find(Name("table")) {
        for tr in div.find(Name("tr")) {
            for td in tr.find(Name("td")) {
                tds.push(td.text());
            }
        }
    }

    let mut events = Vec::new();

    for i in 0..tds.len() {
        if tds[i].contains(query) {
            if let Some(td) = tds.get(i + 2) {
                let split = td.split(", ").collect::<Vec<&str>>();
                if split.len() > 1 {
                    let german_date = split[1];
                    println!("{}", german_date);
                    if let Ok(date) = NaiveDate::parse_from_str(german_date, "%d.%m.%Y") {
                        events.push(MyEvent {
                            title: tds[i].to_string(),
                            date: date,
                        });
                    }
                }

            }
        }
    }
    events
}

fn add_calendar_event(hub: &mut Hub, my_event: &MyEvent) {
    let calendar_id = "s19um28f482r51p1m0ss1tag98@group.calendar.google.com";
    //let result = hub.events().list(calendar_id).max_results(10).doit();
    let mut event: Event = Default::default();

    let mut start: EventDateTime = Default::default();
    let mut end: EventDateTime = Default::default();

    start.time_zone = Some("Europe/Berlin".into());
    end.time_zone = Some("Europe/Berlin".into());

    start.date = Some(my_event.date.to_string());
    end.date = Some(my_event.date.to_string());

    event.start = Some(start);
    event.end = Some(end);
    event.summary = Some(my_event.title.clone());

    let result = hub.events().insert(event, calendar_id).doit();

    match result {
        Ok(res) => {
            println!("{:?}", res.1);
            println!("request successfull");
        }
        Err(err) => {
            println!("{}", err);
        }
    }
}

fn init_hub() -> Hub {
    let mut f = File::open("credentials.json").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();
    let credentials: CredentialsFile = serde_json::from_str(&s).unwrap();

    let auth = Authenticator::new(
        &credentials.installed,
        DefaultAuthenticatorDelegate,
        hyper::Client::with_connector(hyper::net::HttpsConnector::new(
            hyper_rustls::TlsClient::new(),
        )),
        DiskTokenStorage::new(&"token_store.json".to_string()).unwrap(),
        Some(yup_oauth2::FlowType::InstalledInteractive),
    );

    CalendarHub::new(
        hyper::Client::with_connector(hyper::net::HttpsConnector::new(
            hyper_rustls::TlsClient::new(),
        )),
        auth,
    )
}

