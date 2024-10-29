mod schema;
mod model;
mod pagination;

use std::collections::HashMap;
use std::sync::RwLock;
use chrono::{NaiveTime, TimeDelta};

use diesel::associations::HasTable;
use diesel::{prelude::*, query_builder};
use diesel::connection::{set_default_instrumentation, Instrumentation, InstrumentationEvent};
use serde::Serialize;
use time::{Duration, OffsetDateTime, Time};
use crate::pagination::Debugable;
use crate::model::Post;

pub fn establish_connection() -> PgConnection {
    let database_url = std::env::var("DATABASE_URL")
        .expect("database_url must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[derive(Debug, Serialize)]
struct QueryTiming {
    query: String,
    duration: String
}


fn main() {
    use crate::schema::posts::dsl::*;


    let query_debugger = RwLock::new(HashMap::<String, Time>::new());

    let connection = &mut establish_connection();

    connection.set_instrumentation(move |event: InstrumentationEvent<'_>| {
        let mut query_debugger = query_debugger.write().unwrap();
        match event {
            InstrumentationEvent::StartQuery { query, .. } => {
                let str = query.to_string();
                let re = regex::Regex::new(r"-- QUERY_ID=(.{36})").unwrap();
                let query_id = re.captures(&str).unwrap().get(1).unwrap().as_str().to_string();
                let time = OffsetDateTime::now_utc().time();

                query_debugger.insert(query_id, time);
            }
            InstrumentationEvent::FinishQuery { query , .. } => {
                let str = query.to_string();
                let re = regex::Regex::new(r"-- QUERY_ID=(.{36})").unwrap();
                let query_id = re.captures(&str).unwrap().get(1).unwrap().as_str();

                let filtered = re.replace(&str, "").to_string();
                let filtered = filtered.replace("\"", "");

                let start_time = query_debugger.remove(query_id).unwrap();
                let end_time = OffsetDateTime::now_utc().time();
                let duration = end_time - start_time;

                let query_timing = QueryTiming {
                    query: filtered,
                    duration: format!("{} µs", duration.whole_microseconds())
                };

                let json = serde_json::to_string_pretty(&query_timing).unwrap();
                println!("{}", json);
            }
            _ => {}
        }
    });

    let results  = posts::table()
        .filter(published.eq(true))
        .limit(5)
        .select(Post::as_select())
        .debug()
        .load(connection)
        .expect("Error loading posts");

    println!("{:?}", results);
}
