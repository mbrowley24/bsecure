
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


    let postgres_database = database::connect_to_postgres()
        .await
        .expect("Postgres connection failed");

    let http_client = database::http_client()
        .await
        .expect("HTTP client failed");

    println!("Database connections established");

    let state = Arc::new(
        app_state::state::State::new(
            http_client, postgres_database
        )
    );

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .service(user_routes::configure())


    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
