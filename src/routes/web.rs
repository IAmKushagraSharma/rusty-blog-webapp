use actix_web::{error::ErrorNotFound, web, HttpResponse, Result};
use tera::{Context, Tera};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(home_page))
        .route("/about", web::get().to(about_page))
        .route("/contact", web::get().to(contact_page))
        .default_service(web::route().to(not_found));
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

async fn not_found() -> Result<HttpResponse> {
    let tera =
        Tera::new("templates/**/*.html").map_err(|_| ErrorNotFound("Tera template error"))?;
    let context = Context::new();
    let rendered = tera
        .render("pages/404.html", &context)
        .map_err(|_| ErrorNotFound("Error rendering template"))?;

    Ok(HttpResponse::NotFound().body(rendered))
}


