use crate::api;
use actix_web::{web, Responder, Scope};
use inclusion_articles::{
    api::{Article, Create, DeleteByUuid, FetchByUuid, FetchPage, FetchTotal, Update},
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

#[derive(Deserialize)]
pub struct GetPageQueryString {
    start: i64,
    end: i64,
}

async fn get_list(
    pool: web::Data<Arc<ConnectionPool>>,
    query: web::Query<GetPageQueryString>,
) -> crate::api::TResult<actix_web::HttpResponse> {
    let connection = pool.connect()?;
    let result = Article::fetch_page(&connection, query.start, query.end)?;
    let result_total = Article::fetch_total(&connection)?;
    let content_range = format!(
        "article {start}-{end}/{total}", // used for pagination
        start = query.start,
        end = query.end,
        total = result_total
    );
    Ok(actix_web::HttpResponse::Ok()
        .header("Content-Range", content_range)
        .json(result))
}

async fn get(
    pool: web::Data<Arc<ConnectionPool>>,
    web::Path(uuid): web::Path<uuid::Uuid>,
) -> impl Responder {
    let connection = pool.connect()?;
    api::as_json(Ok(Article::fetch_by_uuid(&connection, &uuid)?))
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct PutPayload {
    uuid: uuid::Uuid,
    name: String,
}

async fn put(
    pool: web::Data<Arc<ConnectionPool>>,
    payload: web::Json<PutPayload>,
) -> crate::api::TResult<actix_web::HttpResponse> {
    let connection = pool.connect()?;
    let _article = Article::default()
        .uuid(&payload.uuid)
        .content(&payload.name)
        .update(&connection)?;
    Ok(actix_web::web::HttpResponse::Ok().body(""))
}

pub(crate) fn scope(path: &str) -> Scope {
    actix_web::web::scope(path)
        .service(
            actix_web::web::resource("/{uuid}/")
                .route(actix_web::web::delete().to(delete))
                .route(actix_web::web::get().to(get)),
        )
        .service(
            actix_web::web::resource("/")
                .route(actix_web::web::get().to(get_list))
                .route(actix_web::web::post().to(post))
                .route(actix_web::web::put().to(put)),
        )
}
