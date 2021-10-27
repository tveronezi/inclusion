use crate::schema::articles;
use crate::{connection::Connection, ApiResult};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

pub trait Create {
    fn create(self, connection: &Connection) -> crate::ApiResult<Self>
    where
        Self: Sized;
}

pub trait Update {
    fn update(self, connection: &Connection) -> crate::ApiResult<Self>
    where
        Self: Sized;
}

pub trait DeleteByUuid {
    fn delete_by_uuid(connection: &Connection, uuid: &uuid::Uuid) -> crate::ApiResult<()>;
}

pub trait FetchByUuid {
    fn fetch_by_uuid(connection: &Connection, uuid: &uuid::Uuid) -> crate::ApiResult<Option<Self>>
    where
        Self: Sized;
}

pub trait FetchPage {
    fn fetch_page(connection: &Connection, start: i64, end: i64) -> crate::ApiResult<Vec<Self>>
    where
        Self: Sized;
}

pub trait FetchTotal {
    fn fetch_total(connection: &Connection) -> crate::ApiResult<i64>;
}

#[derive(Insertable, Identifiable, Queryable, Debug, PartialEq, QueryableByName, Clone, Serialize, Deserialize)]
#[table_name = "articles"]
#[primary_key(uuid)]
pub struct Article {
    pub uuid: uuid::Uuid,
    pub content: String,
}

impl Article {
    pub fn content<T>(mut self, content: T) -> Self
    where
        T: ToString,
    {
        self.content = content.to_string();
        self
    }

    pub fn uuid(mut self, uuid: &uuid::Uuid) -> Self
    {
        self.uuid = uuid.clone();
        self
    }
}

impl Default for Article {
    fn default() -> Self {
        Self {
            uuid: uuid::Uuid::new_v4(),
            content: "".to_string(),
        }
    }
}

impl DeleteByUuid for Article {
    fn delete_by_uuid(connection: &Connection, item_uuid: &uuid::Uuid) -> crate::ApiResult<()> {
        use crate::schema::articles::dsl::{articles, uuid};
        diesel::delete(articles.filter(uuid.eq(item_uuid))).execute(&connection.connection)?;
        Ok(())
    }
}

impl Create for Article {
    fn create(mut self, connection: &Connection) -> crate::ApiResult<Self> {
        self.content = self.content.trim().to_string();
        let _ = diesel::insert_into(articles::table)
            .values(&self)
            .execute(&connection.connection)?;
        Ok(self)
    }
}

impl Update for Article {
    fn update(mut self, connection: &Connection) -> ApiResult<Self>
    where
        Self: Sized,
    {
        use crate::schema::articles::dsl::{articles, content, uuid};
        self.content = self.content.trim().to_string();
        let _ = diesel::update(articles.filter(uuid.eq(&self.uuid)))
            .set(content.eq(&self.content))
            .execute(&connection.connection)?;
        Ok(self)
    }
}

impl FetchByUuid for Article {
    fn fetch_by_uuid(connection: &Connection, entry_uuid: &uuid::Uuid) -> ApiResult<Option<Self>> {
        use crate::schema::articles::dsl::{articles, uuid};
        match articles
            .filter(uuid.eq(entry_uuid))
            .first(&connection.connection)
        {
            Ok(result) => Ok(Some(result)),
            Err(diesel::result::Error::NotFound) => Ok(None),
            Err(e) => Err(crate::ApiError(format!("{}", e))),
        }
    }
}

impl FetchPage for Article {
    fn fetch_page(connection: &Connection, start: i64, end: i64) -> ApiResult<Vec<Self>>
    where
        Self: Sized,
    {
        use crate::schema::articles::table;
        let query = table.into_boxed().limit(end - start + 1).offset(start);
        Ok(query.load(&connection.connection)?)
    }
}

impl FetchTotal for Article {
    fn fetch_total(connection: &Connection) -> ApiResult<i64> {
        use crate::schema::articles::{dsl::articles, dsl::uuid};
        use diesel::dsl::count;
        let query = articles.into_boxed();
        Ok(query.select(count(uuid)).first(&connection.connection)?)
    }
}
