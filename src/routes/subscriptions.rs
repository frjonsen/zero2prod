use actix_web::{web, HttpResponse, Responder};
#[derive(serde::Deserialize)]
pub(crate) struct FormData {
    email: String,
    name: String,
}
pub(crate) async fn subscribe(_form: web::Form<FormData>) -> impl Responder {
    HttpResponse::Ok()
}
