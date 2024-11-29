use actix_web::{web, HttpResponse, Responder};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.route("/login", web::post().to(login_handler));
}

async fn login_handler() -> impl Responder {
    HttpResponse::Ok().body("This is the login endpoint")
}
