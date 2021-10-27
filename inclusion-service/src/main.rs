#[actix_web::main]
async fn main() {
    env_logger::init();
    let config = inclusion_service::config::get_settings();
    inclusion_service::start(config)
        .await
        .expect("Unable to start the server.");
}
