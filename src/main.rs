use actix_cors::Cors;
use actix_web::{
    dev::{ServiceRequest, ServiceResponse},
    error::ErrorUnauthorized,
    middleware::Logger,
    App, Error, HttpServer, web::Data,
};
use actix_web_lab::middleware::{from_fn, Next};
use configs::Configs;
use database::Database;
use dotenv::dotenv;
use regex::Regex;
use routes::init_routes;

mod configs;
mod routes;
mod database;

lazy_static::lazy_static! {
    pub static ref CONFIGS: Configs = Configs::new();
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    pretty_env_logger::init();

    let database = Database::new(&CONFIGS.database_url).await;
    database.migrate().await;
    let database_data = Data::new(database);

    HttpServer::new(move || {
        App::new()
            .configure(init_routes)
            .wrap(from_fn(auth))
            .wrap(Logger::default())
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header(),
            )
        .app_data(database_data.clone())
    })
    .bind(&CONFIGS.bind_url)?
    .run()
    .await
}

async fn auth<B>(req: ServiceRequest, next: Next<B>) -> Result<ServiceResponse<B>, Error> {
    let re = Regex::new(r"^\/(?:books\/([^\/]+)\/images\/([^\/]+)|collections\/([^\/]+)\/thumbnail)$").unwrap();
    if re.is_match(req.request().path()) {
        let res = next.call(req).await?;
        return Ok(res);
    }

    if let Some(auth) = req.headers().get("Authorization") {
        if auth.to_str().unwrap() == CONFIGS.sst {
            let res = next.call(req).await?;
            Ok(res)
        } else {
            Err(ErrorUnauthorized("Unauthorized"))
        }
    } else {
        Err(ErrorUnauthorized("Unauthorized"))
    }
}
