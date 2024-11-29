pub mod api;
pub mod auth;
pub mod web;

use actix_web::web as actix_web_web;

pub fn init(cfg: &mut actix_web_web::ServiceConfig) {
    cfg.service(actix_web_web::scope("/api").configure(api::init))
        .service(actix_web_web::scope("/auth").configure(auth::init));
}



