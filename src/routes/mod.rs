pub mod api;
pub mod auth;
pub mod web;

use actix_files as fs;
use actix_web::{error::ErrorNotFound, web as actix_web_web, HttpResponse, Result};
use lazy_static::lazy_static;
use tera::{Context, Tera};

lazy_static! {
    static ref TERA: Tera =
        Tera::new("templates/**/*.html").expect("Failed to initialize Tera templates");
}

pub fn init(cfg: &mut actix_web_web::ServiceConfig) {
    cfg.service(fs::Files::new("/static", "./static").show_files_listing())
        .service(actix_web_web::scope("/api").configure(api::init))
        .service(actix_web_web::scope("/auth").configure(auth::init))
        .service(actix_web_web::scope("").configure(web::init))
        .default_service(actix_web_web::route().to(not_found));
}

async fn not_found() -> Result<HttpResponse> {
    let context = Context::new();
    let rendered = TERA
        .render("pages/404.html", &context)
        .map_err(|_| ErrorNotFound("Error rendering template"))?;

    Ok(HttpResponse::NotFound().body(rendered))
}

