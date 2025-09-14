use crate::connection::*;
use crate::database::DatabaseError;
use crate::models::users::User;

pub fn init_user_table() -> Result<(), DatabaseError> {
    connect_database(None)?;

    {
        let db_guard = get_database()?;
        let conn = db_guard.get()?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS users (
            id TEXT PRIMARY KEY,
            username TEXT NOT NULL,
            email TEXT NOT NULL,
            created_at TEXT NOT NULL
        )",
            [],
        )?;
    }

    close_database()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::storage::user_storage::init_user_table;

    #[test]
    fn try_init_user_table() {
        init_user_table();
    }
}
