use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

pub(crate) mod articles;
pub(crate) mod system;

pub(crate) type TJsonResult<T> = TResult<actix_web::web::Json<T>>;

pub(crate) fn as_json<T>(t_result: TResult<T>) -> TJsonResult<T> {
    let result = t_result?;
    Ok(actix_web::web::Json(result))
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct UuidPayload {
    uuid: uuid::Uuid,
}

#[derive(Debug)]
pub(crate) enum AppError {
    Articles(inclusion_articles::ApiError),
}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl actix_web::ResponseError for AppError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        actix_web::dev::HttpResponseBuilder::new(self.status_code())
            .set_header(actix_web::http::header::CONTENT_TYPE, mime::TEXT.as_str())
            .body(format!("{:?}", self))
    }
}

impl From<inclusion_articles::ApiError> for AppError {
    fn from(e: inclusion_articles::ApiError) -> Self {
        AppError::Articles(e)
    }
}

pub(crate) type TResult<T> = Result<T, AppError>;
