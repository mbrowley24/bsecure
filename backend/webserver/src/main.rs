
mod repository{
    pub mod database;

}

mod json_schemas;
mod app_state;

mod routes{
    pub mod user_routes;
}

use std::sync::Arc;
use actix_web::{web, App, HttpServer};
use crate::repository::database;
use crate::routes::user_routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mongo_database = database::connect_to_mongodb()
        .await
        .expect("MongoDB connection failed");

    let postgres_database = database::connect_to_postgres()
        .await
        .expect("Postgres connection failed");


    println!("Database connections established");

    let state = app_state::state::State::new(mongo_database, postgres_database);

    let db_arc = Arc::new(state);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_arc.clone()))
            .service(user_routes::configure())


    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
