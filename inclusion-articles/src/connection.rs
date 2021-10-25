use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::PgConnection;
use std::str::FromStr;

embed_migrations!();

pub struct Connection {
    pub(crate) connection: PooledConnection<ConnectionManager<PgConnection>>,
}

pub struct ConnectionPool {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl ConnectionPool {
    pub fn connect(&self) -> crate::ApiResult<Connection> {
        let connection = self.pool.get()?;
        Ok(Connection { connection })
    }
}

impl FromStr for ConnectionPool {
    type Err = crate::ApiError;

    fn from_str(database_url: &str) -> Result<Self, Self::Err> {
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = Pool::builder().build(manager)?;
        let migration_connection = pool.get()?;
        embedded_migrations::run(&migration_connection)?;
        Ok(Self { pool })
    }
}
