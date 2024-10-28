mod schema;
mod model;
mod pagination;

use std::collections::HashMap;
use std::sync::RwLock;
use chrono::NaiveTime;

use diesel::associations::HasTable;
use diesel::{prelude::*, query_builder};
use diesel::connection::{set_default_instrumentation, Instrumentation, InstrumentationEvent};
use crate::pagination::Debugable;
use crate::model::Post;

pub fn establish_connection() -> PgConnection {
    let database_url = std::env::var("DATABASE_URL")
        .expect("database_url must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}




fn main() {
    use crate::schema::posts::dsl::*;


    let query_debugger = RwLock::new(HashMap::<String, NaiveTime>::new());

    let connection = &mut establish_connection();

    connection.set_instrumentation(move |event: InstrumentationEvent<'_>| {
        println!("Instrumentation event: {:?}", event);
        let mut query_debugger = query_debugger.write().unwrap();
        match event {
            InstrumentationEvent::StartQuery { query, .. } => {
                let str = query.to_string();
                let re = regex::Regex::new(r"QUERY_ID=(.{32})").unwrap();
                let query_id = re.captures(&str).unwrap().get(1).unwrap().as_str().to_string();

                println!("query_id: {}", query_id);
                println!("query: {}", query.to_string());

                query_debugger.insert(query_id, chrono::Utc::now().time());
            }
            InstrumentationEvent::FinishQuery { query , .. } => {
                let str = query.to_string();
                let re = regex::Regex::new(r"QUERY_ID=(.{32})").unwrap();
                let query_id = re.captures(&str).unwrap().get(1).unwrap().as_str();

                println!("query_id: {}", query_id);
                println!("queries: {:?}", query_debugger);
                println!("query: {}", query.to_string());

                let start_time = query_debugger.remove(query_id).unwrap();
                let end_time = chrono::Utc::now().time();
                println!("Query: {} took {:?}", query, end_time - start_time);
            }
            _ => {}
        }
    });

    let results  = posts::table()
        .filter(published.eq(true))
        .limit(5)
        .select(Post::as_select())
        .debug()
        .exec(connection)
        .expect("Error loading posts");

    println!("{:?}", results);
}
