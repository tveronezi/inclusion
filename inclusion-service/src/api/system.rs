async fn ping() -> impl actix_web::Responder {
    "pong"
}

pub(crate) fn scope(path: &str) -> actix_web::Scope {
    actix_web::web::scope(path)
        .service(actix_web::web::resource("/ping/").route(actix_web::web::get().to(ping)))
}
