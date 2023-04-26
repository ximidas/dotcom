use actix_web::{
    web::Data,
    App, HttpServer,
};
use actix_cors::Cors;
use tokio_postgres::NoTls;

mod config;
mod entities;
mod handlers;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = crate::config::Config::from_file("Config.toml").unwrap();

    let pool = config.pg.create_pool(None, NoTls).unwrap();

    println!("Starting server at http://{}:{}/", config.server.host, config.server.port);
    HttpServer::new(move || {
        let cors = Cors::default()
        .allow_any_origin()
        .allow_any_method()
        .allow_any_header()
        .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(Data::new(pool.clone()))
            .configure(routes::init::initialize)
    })
    .bind((config.server.host, config.server.port))?
    .run()
    .await
}