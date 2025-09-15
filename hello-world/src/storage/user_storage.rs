use crate::connection::*;
use crate::database::DatabaseError;
use crate::models::users::User;

pub async fn init_user_table() -> Result<(), DatabaseError> {
    connect_database(None).await?;

    {
        let db_guard = get_database().await?;
        let conn = db_guard.get()?;
        conn.call(|conn| {
            conn.execute(
                "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT NOT NULL,
            email TEXT NOT NULL,
            created_at TEXT NOT NULL)",
                [],
            )?;
            Ok(())
        }).await?;
    }

    close_database().await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::storage::user_storage::init_user_table;

    #[tokio::test]
    async fn try_init_user_table() {
        let result = init_user_table().await;
        assert!(result.is_ok(), "Failed to initialize user table: {:?}", result.err());
    }
}
