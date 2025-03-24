use actix_web::{
    App,
    dev::Server,
    HttpServer,
    web,
};

mod app_state;


mod json_schemas;

mod models;

mod repository{
    pub mod database;

}

use crate::repository::database;
mod routes;

mod router;
mod services;

use std::sync::Arc;




#[actix_web::main]
async fn main() -> std::io::Result<()> {


    let postgres_database = database::connect_to_postgres()
        .await
        .expect("Postgres connection failed");

    let http_client = database::http_client()
        .await
        .expect("HTTP client failed");

    let mongo_database = database::connect_to_mongodb()
        .await
        .expect("MongoDB connection failed");

    println!("Database connections established");



    services::init_services::init_services(&postgres_database)
        .await
        .expect("Unable to init services");


    let state = Arc::new(
        app_state::state::State::new(
            http_client,
            mongo_database,
            postgres_database
        )
    );



    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .service(router::configure())


    })

    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
