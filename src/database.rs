use sqlx::sqlite::SqlitePool;
use std::env;
use tokio::runtime::Runtime;

pub struct DatabaseConnection {
    pool: SqlitePool,
}

impl DatabaseConnection {
    /// Creates a connection pool based on the variable set in the '.env' file.
    pub fn new() -> anyhow::Result<Self> {
        let mut runtime = Runtime::new().unwrap();
        let pool = runtime.block_on(Self::create_pool())?;
        Ok(DatabaseConnection { pool })
    }

    async fn create_pool() -> anyhow::Result<SqlitePool> {
        Ok(SqlitePool::builder()
            .max_size(5)
            .build(&env::var("DATABASE_URL")?)
            .await?)
    }

    /// Retrieves the currently initialized pool for the database.
    pub fn pool(&self) -> &SqlitePool {
        &self.pool
    }
}
