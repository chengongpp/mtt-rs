use sqlx::{Database, Pool, SqlitePool};
use crate::conf::DatabaseConfig;

pub enum DatabaseDriver {
    Sqlite,
    Postgres,
    Mysql,
}

pub struct MttDatabase {
    pub driver: DatabaseDriver,
    pub host: String,
    pub port: u16,
}

pub enum DatabaseError {
    InitError(String),
    SqliteError(String),
    PostgresError(String),
    MysqlError(String),
}

impl MttDatabase {
    pub fn new(conf: &DatabaseConfig) -> Result<MttDatabase, DatabaseError> {
        let driver = match conf.driver.to_lowercase().as_str() {
            "sqlite" | "sqlite3" => { Ok(DatabaseDriver::Sqlite) }
            "mysql" | "mariadb" => { Ok(DatabaseDriver::Mysql) }
            "postgres" | "postgresql" => { Ok(DatabaseDriver::Mysql) }
            _ => Err(DatabaseError::InitError(format!("Unknown database driver: {}", conf.driver)))
        };
        let driver = driver?;
        let database: dyn Database = match driver {
            DatabaseDriver::Sqlite => {
                let db_url = format!("sqlite://{}", conf.path);
                let pool = SqlitePool::builder().max_connections(conf.pool_size).build(&db_url).map_err(|e| DatabaseError::SqliteError(e.to_string()))?;
                sqlx::sqlite::Sqlite::new(pool)
            }
            DatabaseDriver::Mysql => {
                let db_url = format!("mysql://{}:{}@{}:{}/{}", conf.user, conf.password, conf.host, conf.port, conf.database);
                let pool = Pool::builder().max_connections(conf.pool_size).build(&db_url).map_err(|e| DatabaseError::MysqlError(e.to_string()))?;
                sqlx::mysql::MySql::new(pool)
            }
            DatabaseDriver::Postgres => {
                let db_url = format!("postgres://{}:{}@{}:{}/{}", conf.user, conf.password, conf.host, conf.port, conf.database);
                let pool = Pool::builder().max_connections(conf.pool_size).build(&db_url).map_err(|e| DatabaseError::PostgresError(e.to_string()))?;
                sqlx::postgres::PgPool::new(pool)
            }
        };
        let db = MttDatabase {
            driver,
            host: conf.host.clone(),
            port: conf.port,
        };
        Ok(db)
    }
}