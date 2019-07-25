use crate::calendar::MyEvent;

use chrono::NaiveDate;
use select::document::Document;
use select::predicate::{Attr, Class, Name, Predicate};

pub fn get_thb_return_date() -> Vec<MyEvent> {
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
