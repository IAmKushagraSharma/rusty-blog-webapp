use actix_files as fs;
use actix_web::{middleware::Logger, web, App, HttpServer};
use tera::Tera;
use tracing::info;

mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter("actix_web=info")
        .init();

    info!("Starting the Actix Web server...");

    let tera = Tera::new("templates/**/*").unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(tera.clone()))
            .wrap(Logger::default())
            .configure(routes::init)
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}
