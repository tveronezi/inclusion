use crate::api;
use actix_web::{web, Responder, Scope};
use inclusion_articles::{
    api::{Article, Create, DeleteByUuid},
    connection::ConnectionPool,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

async fn delete(
    pool: web::Data<Arc<ConnectionPool>>,
    web::Path(uuid): web::Path<uuid::Uuid>,
) -> impl Responder {
    let connection = pool.connect()?;
    Article::delete_by_uuid(&connection, &uuid)?;
    api::as_json(Ok(()))
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct PostPayload {
    content: String,
}

async fn post(
    pool: web::Data<Arc<ConnectionPool>>,
    payload: web::Json<PostPayload>,
) -> impl Responder {
    let connection = pool.connect()?;
    let article = Article::default()
        .content(&payload.content)
        .create(&connection)?;
    let uuid = article.uuid;
    api::as_json(Ok(api::UuidPayload { uuid }))
}

pub(crate) fn scope() -> Scope {
    actix_web::web::scope("/api/v1/article")
        .service(web::resource("/{uuid}/").route(web::delete().to(delete)))
        .service(web::resource("/").route(web::post().to(post)))
}
