#[derive(Debug)]
pub enum DatabaseError {
    ConnectionError(rusqlite::Error),
    LockError(String),
    NotInitializedError
}

impl std::fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DatabaseError::ConnectionError(e) => write!(f, "Database connection error: {}", e),
            DatabaseError::LockError(msg) => write!(f, "Lock error: {}",msg),
            DatabaseError::NotInitializedError => write!(f, "Database not initialized error")
        }
    }
}

impl std::error::Error for DatabaseError {
}

impl From<rusqlite::Error> for DatabaseError {
    fn from(err: rusqlite::Error) -> Self {
        DatabaseError::ConnectionError(err)
    }
}