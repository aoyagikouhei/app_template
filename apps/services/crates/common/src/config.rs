use serde::{Deserialize, Serialize};
use sqlx::{postgres::{PgConnectOptions, PgPoolOptions}, PgPool};
use std::{sync::OnceLock, time::Duration};

// PG Parameters
// https://docs.rs/sqlx/0.8.5/sqlx/postgres/struct.PgConnectOptions.html

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Config {
    pub max_connections: u32,
    pub acquire_timeout: u64,
    pub idle_timeout: u64,
    pub pg_password: String,
}

static INSTANCE: OnceLock<Config> = OnceLock::new();

impl Config {
    pub fn new() -> Self {
        dotenv::dotenv().ok();
        let max_connections = std::env::var("MAX_CONNECTIONS")
            .unwrap_or_else(|_| "10".to_string())
            .parse()
            .unwrap_or(10);
        let acquire_timeout = std::env::var("ACQUIRE_TIMEOUT")
            .unwrap_or_else(|_| "300".to_string())
            .parse()
            .unwrap_or(300);
        let idle_timeout = std::env::var("IDLE_TIMEOUT")
            .unwrap_or_else(|_| "600".to_string())
            .parse()
            .unwrap_or(600);
        // TODO SSM
        let pg_password = std::env::var("PGPASSWORD")
            .unwrap_or_else(|_| "pass".to_string());
        Self {
            max_connections,
            acquire_timeout,
            idle_timeout,
            pg_password,
        }    
    }

    pub async fn pg_pool(&self) -> Result<PgPool, sqlx::Error> {
        let connect_options = PgConnectOptions::new_without_pgpass()
            .password(&self.pg_password)
        ;
        PgPoolOptions::new()
            .max_connections(self.max_connections)
            .acquire_timeout(Duration::from_secs(self.acquire_timeout))
            .idle_timeout(Duration::from_secs(self.idle_timeout))
            .connect_with(connect_options)
            .await
    }

    pub fn global() -> &'static Config {
        INSTANCE.get().expect("config is not initialized")
    }

    pub fn set_global(value: &Config) {
        INSTANCE.set(value.clone()).expect("config don't update");
    }

    pub fn setup() -> &'static Config {
        let config = Config::new();
        Self::set_global(&config);
        Self::global()
    }
}
