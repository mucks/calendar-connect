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

mod calendar;
mod sites;


fn main() {
    let mut hub = calendar::init_hub();

    for my_event in sites::thb_general_schedule::get_thb_return_date() {
        println!("hey");
        println!("{:?}", calendar::add_calendar_event(&mut hub, &my_event));
    }
}