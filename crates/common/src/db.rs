use anyhow::Result;
use deadpool_postgres::{Config, CreatePoolError, Pool, Runtime};
use tokio_postgres::NoTls;

pub async fn create_pool(env: String) -> Result<Pool, CreatePoolError> {
    if env == "dev" {
        let mut cfg = Config::new();
        cfg.user = Some(String::from(""));
        cfg.password = Some(String::from(""));
        cfg.host = Some(String::from(""));
        cfg.port = Some(0);
        cfg.dbname = Some(String::from(""));
        cfg.create_pool(Some(Runtime::Tokio1), NoTls)
    } else if env == "prod" {
        let mut cfg = Config::new();
        cfg.user = Some(String::from(""));
        cfg.password = Some(String::from(""));
        cfg.host = Some(String::from(""));
        cfg.port = Some(1);
        cfg.dbname = Some(String::from(""));
        cfg.create_pool(Some(Runtime::Tokio1), NoTls)
    } else {
        panic!("env {} not supported", env);
    }
}
