use actix_web::{web, HttpResponse, Responder};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.route("/example", web::get().to(example_handler));
}

async fn example_handler() -> impl Responder {
    HttpResponse::Ok().body("This is an API example route")
}
