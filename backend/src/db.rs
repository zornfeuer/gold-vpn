use anyhow::Result;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::time::Duration;
use tracing::info;

pub struct Database {
    pub pool: Pool<Postgres>,
}

impl Database {
    pub async fn connect() -> Result<Self> {
        let database_host = std::env::var("POSTGRES_HOST")
            .unwrap_or("localhost".to_string());
        let database_port = std::env::var("POSTGRES_PORT")
            .unwrap_or(5432.to_string());
        let database_user = std::env::var("POSTGRES_USER")
            .unwrap_or("user".to_string());
        let database_password = crate::utils::secrets::load_password("/run/secrets/db_password")
            .await
            .unwrap_or("password".to_string());
        let database_name = std::env::var("POSTGRES_DB")
            .unwrap_or("postgres".to_string());

        let database_url = format!(
            "postgres://{}:{}@{}:{}/{}",
            database_user,
            database_password,
            database_host,
            database_port,
            database_name
        );
        
        info!("🔗 Подключение к базе данных PostgreSQL...");
        
        let pool = PgPoolOptions::new()
            .max_connections(10)
            .acquire_timeout(Duration::from_secs(10))
            .connect(&database_url)
            .await?;
        
        info!("✅ Подключение к базе данных установлено");
        
        sqlx::query("SELECT 1")
            .execute(&pool)
            .await
            .map_err(|e| anyhow::anyhow!("Ошибка проверки подключения: {}", e))?;
        
        Ok(Self { pool })
    }
    
    pub async fn initialize(&self) -> Result<()> {
        info!("🔧 Инициализация базы данных...");
        
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS vps_credentials (
                id SERIAL PRIMARY KEY,
                ip VARCHAR(45) NOT NULL,
                uuid VARCHAR(36) NOT NULL,
                pbk TEXT NOT NULL,
                sid VARCHAR(32) NOT NULL,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                is_active BOOLEAN DEFAULT TRUE
            )
            "#
        )
        .execute(&self.pool)
        .await?;
        
        sqlx::query(
            r#"
            CREATE INDEX IF NOT EXISTS idx_vps_active 
            ON vps_credentials(is_active)
            "#
        )
        .execute(&self.pool)
        .await?;
        
        info!("✅ База данных инициализирована");
        Ok(())
    }
}

pub type DbPool = Pool<Postgres>;
