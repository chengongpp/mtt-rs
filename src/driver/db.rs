use sqlx::{Any, AnyPool, Pool};
use log::{info, error};
use crate::conf::DatabaseConfig;

pub enum DatabaseDriver {
    Sqlite,
    Postgres,
    Mysql,
}

#[derive(Debug)]
pub enum DatabaseError {
    InitError(String),
}

pub async fn new_pool(conf: &DatabaseConfig) -> Result<Pool<Any>, DatabaseError> {
    let driver = match conf.driver.to_lowercase().as_str() {
        "sqlite" | "sqlite3" => { Ok(DatabaseDriver::Sqlite) }
        "mysql" | "mariadb" => { Ok(DatabaseDriver::Mysql) }
        "postgres" | "postgresql" => { Ok(DatabaseDriver::Postgres) }
        _ => Err(DatabaseError::InitError(format!("Unknown database driver: {}", conf.driver)))
    };
    let driver = driver?;
    // unimplemented!();
    // TODO: Support multiple drivers
    let db_url = match driver {
        DatabaseDriver::Sqlite => {
            format!("sqlite://{}", conf.database)
        }
        DatabaseDriver::Mysql => {
            format!("mysql://{}:{}@{}:{}/{}", conf.user, conf.password, conf.host, conf.port, conf.database)
        }
        DatabaseDriver::Postgres => {
            format!("postgres://{}:{}@{}:{}/{}", conf.user, conf.password, conf.host, conf.port, conf.database)
        }
    };
    let pool = AnyPool::connect(&db_url).await;
    let pool = match pool {
        Ok(pool) => {
            info!("Connected to database {}", db_url);
            pool
        }
        Err(e) => {
            error!("{:?}", e);
            return Err(DatabaseError::InitError(format!("Failed to connect to database")));
        }
    };
    Ok(pool)
}