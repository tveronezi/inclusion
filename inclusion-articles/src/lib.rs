#![doc = include_str!("../README.md")]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

pub mod api;
pub mod connection;
mod schema;

use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApiError(String);

impl Display for ApiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for ApiError {}

type ApiResult<T> = Result<T, ApiError>;

impl From<r2d2::Error> for ApiError {
    fn from(e: r2d2::Error) -> Self {
        ApiError(format!("{}", e))
    }
}

impl From<diesel::result::Error> for ApiError {
    fn from(e: diesel::result::Error) -> Self {
        ApiError(format!("{}", e))
    }
}

impl From<diesel_migrations::RunMigrationsError> for ApiError {
    fn from(e: diesel_migrations::RunMigrationsError) -> Self {
        ApiError(format!("{}", e))
    }
}
