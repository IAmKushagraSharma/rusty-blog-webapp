use actix_web::{web, HttpResponse, Responder};
use tera::{Context, Tera};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(index))
        .route("/example", web::get().to(example_handler));
}

async fn index(tmpl: web::Data<Tera>) -> HttpResponse {
    let mut context = Context::new();
    context.insert("page_title", "API Endpoints");

    match tmpl.render("pages/api.html", &context) {
        Ok(rendered) => HttpResponse::Ok().content_type("text/html").body(rendered),
        Err(err) => {
            eprintln!("Template rendering error: {}", err);
            HttpResponse::InternalServerError().body("Internal Server Error")
        }
    }
}

async fn example_handler() -> impl Responder {
    HttpResponse::Ok().body("This is an API example route")
}
