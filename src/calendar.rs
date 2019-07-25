use google_calendar3::{CalendarHub, Channel, Error, Event, EventDateTime};
use yup_oauth2::{
    ApplicationSecret, Authenticator, DefaultAuthenticatorDelegate, DiskTokenStorage, MemoryStorage,
};

use std::fs::File;
use std::io::Read;

use chrono::NaiveDate;

#[derive(Deserialize)]
struct CredentialsFile {
    pub installed: ApplicationSecret,
}

pub struct MyEvent {
    pub title: String,
    pub date: NaiveDate,
}

type Hub = CalendarHub<
    hyper::Client,
    Authenticator<DefaultAuthenticatorDelegate, DiskTokenStorage, hyper::Client>,
>;

pub fn add_calendar_event(hub: &mut Hub, my_event: &MyEvent) -> Result<String, String> {
    let calendar_id = "s19um28f482r51p1m0ss1tag98@group.calendar.google.com";
    //let result = hub.events().list(calendar_id).max_results(10).doit();
    let mut new_event: Event = Default::default();

    let mut start: EventDateTime = Default::default();
    let mut end: EventDateTime = Default::default();

    start.time_zone = Some("Europe/Berlin".into());
    end.time_zone = Some("Europe/Berlin".into());
    start.date = Some(my_event.date.to_string());
    end.date = Some(my_event.date.to_string());

    new_event.start = Some(start);
    new_event.end = Some(end);
    new_event.summary = Some(my_event.title.clone());

    if let Ok(event_list) = hub.events().list(calendar_id).doit() {
        if let Some(events) = event_list.1.items {
            if let Some(old_event) = events.iter().find(|e| e.summary == new_event.summary) {
                if let Some(start) = old_event.start.clone() {
                    if Some(start.date) == Some(new_event.clone().start.unwrap().date) {
                        return Err("event already exists".into());
                    }
                }
            }
        }

    }

    let result = hub.events().insert(new_event, calendar_id).doit();
    match result {
        Ok(res) => {
            println!("{:?}", res.1);
            Ok("event added successfully".into())
        }
        Err(err) => {
            println!("{}", err);
            Err("event insert error".into())
        }
    }

}

pub fn init_hub() -> Hub {
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
