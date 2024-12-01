use actix_web::{web, HttpResponse};
use tera::{Context, Tera};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(home_page))
        .route("/about", web::get().to(about_page))
        .route("/contact", web::get().to(contact_page));
}

async fn home_page(tmpl: web::Data<Tera>) -> HttpResponse {
    let mut context = Context::new();
    context.insert("page_title", "Home Page");

    match tmpl.render("pages/home.html", &context) {
        Ok(rendered) => HttpResponse::Ok().content_type("text/html").body(rendered),
        Err(err) => {
            eprintln!("Template rendering error: {}", err);
            HttpResponse::InternalServerError().body("Internal Server Error")
        }
    }
}

async fn about_page(tmpl: web::Data<Tera>) -> HttpResponse {
    let mut context = Context::new();
    context.insert("page_title", "About Us");

    match tmpl.render("pages/about.html", &context) {
        Ok(rendered) => HttpResponse::Ok().content_type("text/html").body(rendered),
        Err(err) => {
            eprintln!("Template rendering error: {}", err);
            HttpResponse::InternalServerError().body("Internal Server Error")
        }
    }
}

async fn contact_page(tmpl: web::Data<Tera>) -> HttpResponse {
    let mut context = Context::new();
    context.insert("page_title", "Contact Us");

    match tmpl.render("pages/contact.html", &context) {
        Ok(rendered) => HttpResponse::Ok().content_type("text/html").body(rendered),
        Err(err) => {
            eprintln!("Template rendering error: {}", err);
            HttpResponse::InternalServerError().body("Internal Server Error")
        }
    }
}
