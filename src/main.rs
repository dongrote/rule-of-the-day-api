use std::collections::HashMap;
use std::env;
use std::net::Ipv4Addr;
use warp::Filter;
mod handlers;
mod models;
mod types;
mod traits;
use handlers::rotd;
use crate::types::RuleForLife;
use chrono::{NaiveDate, Local, Datelike};

#[tokio::main]
async fn main() {
    const DEFAULT_LISTEN_PORT: u16 = 3000;
    let example1 = warp::get()
        .and(warp::path("api"))
        .and(warp::path("rotd"))
        .and(warp::query::<HashMap<String, String>>())
        .map(|p: HashMap<String, String>| {
            let local_today = Local::today();
            let now = NaiveDate::from_ymd(local_today.year(), local_today.month(), local_today.day());
            let ts = match p.get("date") {
                Some(date) => match date.parse::<NaiveDate>() {
                    Ok(parsed_date) => parsed_date,
                    _ => now,
                },
                None => now,
            };
            match rotd::handler(ts) {
                Ok(rule) => warp::reply::json(&rule),
                Err(status_code) => {
                    println!("Got status code from rotd::handler() {}", status_code);
                    warp::reply::json(& RuleForLife { text: "error".into(), number: 0, quotes: vec![] })
                },
            }
        });
        /*
        .map(|p: HashMap<String, String>| match p.get("name") {
            Some(name) => Response::builder().body(format!("Hello, {}. This HTTP triggered function executed successfully.", name)),
            None => Response::builder().body(String::from("This HTTP triggered function executed successfully. Pass a name in the query string for a personalized response.")),
        });
        */

    let port_key = "FUNCTIONS_CUSTOMHANDLER_PORT";
    let port: u16 = match env::var(port_key) {
        Ok(val) => val.parse().expect("Custom Handler port is not a number!"),
        Err(_) => DEFAULT_LISTEN_PORT,
    };

    warp::serve(example1).run((Ipv4Addr::LOCALHOST, port)).await
}
