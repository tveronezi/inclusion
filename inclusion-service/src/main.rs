use inclusion_service::{config, start};

#[actix_web::main]
async fn main() {
    env_logger::init();
    let config = config::get_settings();
    start(
        &config.get::<String>(config::ConfigKey::ArticlesDbUrl),
        &config.get::<String>(config::ConfigKey::Bind),
    )
    .await
    .expect("The server stopped unexpectedly.");
}
