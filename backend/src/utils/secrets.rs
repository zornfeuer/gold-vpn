use tokio::fs;
use anyhow::Result;


pub async fn load_password(path: &str) -> Result<String> {
    let raw = fs::read_to_string(path).await?;
    
    let trimmed = raw.trim_end_matches(&['\n', '\r', '\t', ' '][..]).to_string();
    
    tracing::debug!(
        "Пароль загружен из {}: длина={} байт, содержит \\n={}",
        path,
        trimmed.len(),
        raw.contains('\n')
    );
    
    if trimmed.is_empty() {
        return Err(anyhow::anyhow!("Файл пароля {} пуст или содержит только управляющие символы", path));
    }
    
    Ok(trimmed)
}
