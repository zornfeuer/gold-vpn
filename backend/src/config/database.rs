use anyhow::Result;
use sqlx::{FromRow, Row, QueryBuilder, Postgres};
use crate::db::DbPool;

#[derive(Debug, FromRow, Clone)]
pub struct VpsCredential {
    pub id: i32,
    pub ip: String,
    pub uuid: String,
    pub pbk: String,
    pub sid: String,
    pub is_active: bool,
}

pub async fn fetch_active_vps(pool: &DbPool) -> Result<Vec<VpsCredential>> {
    let vpses = sqlx::query_as::<_, VpsCredential>(
        r#"
        SELECT id, ip, uuid, pbk, sid, is_active
        FROM vps_credentials
        WHERE is_active = TRUE
        ORDER BY id ASC
        "#
    )
    .fetch_all(pool)
    .await?;
    
    tracing::info!("Получено {} активных VPS из базы данных", vpses.len());
    
    Ok(vpses)
}

pub async fn insert_vps(
    pool: &DbPool,
    ip: &str,
    uuid: &str,
    pbk: &str,
    sid: &str,
) -> Result<i32> {
    let record = sqlx::query(
        r#"
        INSERT INTO vps_credentials (ip, uuid, pbk, sid)
        VALUES ($1, $2, $3, $4)
        RETURNING id
        "#
    )
    .bind(ip)
    .bind(uuid)
    .bind(pbk)
    .bind(sid)
    .fetch_one(pool)
    .await?;

    let id: i32 = record.get(0);
    
    tracing::info!("Добавлен новый VPS с ID: {}", id);
    
    Ok(id)
}

pub async fn update_vps(
    pool: &DbPool,
    id: i32,
    ip: Option<String>,
    uuid: Option<String>,
    pbk: Option<String>,
    sid: Option<String>,
    is_active: Option<bool>,
) -> Result<()> {
    let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(
        "UPDATE vps_credentials SET updated_at = CURRENT_TIMESTAMP"
    );
    
    let mut separated = query_builder.separated(", ");
    
    if let Some(ip) = ip {
        separated.push("ip = ");
        separated.push_bind(ip);
    }
    
    if let Some(uuid) = uuid {
        separated.push("uuid = ");
        separated.push_bind(uuid);
    }
    
    if let Some(pbk) = pbk {
        separated.push("pbk = ");
        separated.push_bind(pbk);
    }
    
    if let Some(sid) = sid {
        separated.push("sid = ");
        separated.push_bind(sid);
    }
    
    if let Some(is_active) = is_active {
        separated.push("is_active = ");
        separated.push_bind(is_active);
    }
    
    query_builder.push(" WHERE id = ");
    query_builder.push_bind(id);
    
    let query = query_builder.build();
    query.execute(pool).await?;
    
    tracing::info!("Обновлен VPS с ID: {}", id);
    
    Ok(())
}

pub async fn deactivate_vps(pool: &DbPool, id: i32) -> Result<()> {
    sqlx::query(
        r#"
        UPDATE vps_credentials
        SET is_active = FALSE, updated_at = CURRENT_TIMESTAMP
        WHERE id = $1
        "#,
    )
    .bind(id)
    .execute(pool)
    .await?;
    
    tracing::info!("Деактивирован VPS с ID: {}", id);
    
    Ok(())
}

pub async fn fetch_all_vps(pool: &DbPool) -> Result<Vec<VpsCredential>> {
    let vpses = sqlx::query_as::<_, VpsCredential>(
        r#"
        SELECT id, ip, uuid, pbk, sid, is_active
        FROM vps_credentials
        ORDER BY id ASC
        "#
    )
    .fetch_all(pool)
    .await?;
    
    Ok(vpses)
}
