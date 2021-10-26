#[actix_web::main]
async fn main() {
    env_logger::init();
    inclusion_service::start()
        .await
        .expect("Unable to start the server.");
}
