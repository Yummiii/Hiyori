use std::env;

use actix_cors::Cors;
use actix_web::{middleware::Logger, web::Data, App, HttpServer};
use dotenv::dotenv;

use database::Database;
use routes::init_routes;

mod database;
mod routes;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    pretty_env_logger::init();

    let database = Database::new(&env::var("DATABASE_URL").expect("Kd a db arrombido?")).await;
    database.migrate().await;
    let database_data = Data::new(database);

    HttpServer::new(move || {
        App::new()
            .configure(init_routes)
            .wrap(Logger::default())
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header(),
            )
            .app_data(database_data.clone())
    })
    .bind(env::var("BIND_URL").unwrap_or("0.0.0.0:3000".to_string()))?
    .run()
    .await
}
