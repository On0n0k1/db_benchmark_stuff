pub mod paciente;

use crate::error::Error;
use crate::globals;
use sqlx::{mssql::MssqlConnectOptions, ConnectOptions, Mssql, MssqlPool, Pool};

pub struct Dao {
    pool: Pool<Mssql>,
}

impl Dao {
    pub fn new() -> Result<Self, Error> {
        let mut options: MssqlConnectOptions = MssqlConnectOptions::new()
            .host(globals::db::HOST)
            .port(globals::db::PORT)
            .username(globals::db::USERNAME)
            .password(globals::db::PASSWORD)
            .database(globals::db::DATABASE);
        // Disable logging for better performance
        options.disable_statement_logging();
        // Lazy connection so that it gives enough time for the database to start up
        let pool: Pool<Mssql> = MssqlPool::connect_lazy_with(options);
        Ok(Self { pool })
    }

    pub fn pool(&self) -> &Pool<Mssql> {
        &self.pool
    }
}
