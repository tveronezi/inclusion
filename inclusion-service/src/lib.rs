#![doc = include_str ! ("../README.md")]

mod api;
pub mod config;

use actix_web::{middleware, App, HttpServer};
use inclusion_articles::connection::ConnectionPool as ArticlesPool;
use inclusion_articles::ApiError as ArticlesError;
use std::fmt::{Display, Formatter};
use std::sync::Arc;

#[derive(Debug)]
pub enum ServerError {
    Articles(String),
    Other(String),
}

impl Display for ServerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl std::error::Error for ServerError {}

impl From<ArticlesError> for ServerError {
    fn from(e: ArticlesError) -> Self {
        Self::Articles(format!("{}", e))
    }
}

impl From<std::io::Error> for ServerError {
    fn from(e: std::io::Error) -> Self {
        Self::Other(format!("{}", e))
    }
}

pub async fn start(articlesdb_url: &str, bind: &str) -> Result<(), ServerError> {
    let articles_pool = Arc::new(articlesdb_url.parse::<ArticlesPool>()?);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(middleware::NormalizePath::new(
                middleware::normalize::TrailingSlash::Always,
            ))
            .data(articles_pool.clone())
            .service(api::articles::scope("/api/v1/article"))
            .service(api::system::scope("/api/v1/system"))
    })
    .bind(bind)?
    .run();
    log::info!("Service ready.");
    server.await?;
    Ok(())
}
