mod schema;
mod model;
mod pagination;

use diesel::associations::HasTable;
use diesel::prelude::*;
use diesel::connection::{set_default_instrumentation, Instrumentation, InstrumentationEvent};
use pagination::{DebugQuery, Paginate};
use crate::model::Post;

pub fn establish_connection() -> PgConnection {
    let database_url = std::env::var("DATABASE_URL")
        .expect("database_url must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}




fn main() {
    use crate::schema::posts::dsl::*;

    let connection = &mut establish_connection();

    connection.set_instrumentation(move |event: InstrumentationEvent<'_>| {
        println!("{event:?}")
    });

    let results  = posts::table()
        .filter(published.eq(true))
        .limit(5)
        .select(Post::as_select())
        .paginate(1)
        .load_and_count_pages(connection)
        .expect("Error loading posts");

    println!("{:?}", results);
}
