use actix_web::{HttpResponse, Responder, web};

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(form: web::Form<FormData>) -> impl Responder {
    // Needed so cargo check passes.
    println!("email: {}", form.email);
    println!("name: {}", form.name);

    HttpResponse::Ok()
}
