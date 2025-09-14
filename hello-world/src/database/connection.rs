use std::sync::{Mutex};
use rusqlite::{Connection};
use once_cell::sync::Lazy;

use super::error::DatabaseError;

static DB_INSTANCE: Lazy<Mutex<Option<Connection>>>
= Lazy::new(| | Mutex::new(None));

pub async fn connect_database(path: Option<&str>) -> Result<(), DatabaseError> {
    let db_path = path.unwrap_or("database.db");
    let conn = Connection::open(db_path)?;
    let mut db_static = match DB_INSTANCE.lock() {
        Ok(guard) => guard,
        Err(_) => return Err(DatabaseError::LockError("Failed to acquire database pointer lock.".to_string()))
    };

    *db_static = Some(conn);

    Ok(())
}

pub async fn get_database() -> Result<DatabaseGuard, DatabaseError> {
    let db_static = match DB_INSTANCE.lock() {
        Ok(guard) => guard,
        Err(_) => return Err(DatabaseError::LockError("Failed to acquire database pointer lock".to_string()))
    };

    match &*db_static {
        Some(_) => {
            Ok(DatabaseGuard::new(db_static))
        },
        None => {
            return Err(DatabaseError::NotInitializedError)
        }
    }
}

pub struct DatabaseGuard {
    guard: std::sync::MutexGuard<'static, Option<Connection>>,
}

impl DatabaseGuard {
    fn new(guard: std::sync::MutexGuard<'static, Option<Connection>>) -> Self {
        Self { guard }
    }
    
    /// 获取数据库连接的引用
    pub fn get(&self) -> Result<&Connection, DatabaseError> {
        match &*self.guard {
            Some(conn) => Ok(conn),
            None => Err(DatabaseError::NotInitializedError)
        }
    }
}

impl std::ops::Deref for DatabaseGuard {
    type Target = Connection;
    fn deref(&self) -> &Self::Target {
        match self.get() {
            Ok(conn) => conn,
            Err(e) => {
                panic!("Database guard error: {:?}", e)
            }
        }
    }
}

pub async fn close_database() -> Result<(), DatabaseError> {
    let mut db_static = match DB_INSTANCE.lock() {
        Ok(guard) => guard,
        Err(_) => return Err(DatabaseError::LockError("Failed to acquire database pointer lock".to_string()))
    };
    
    // 直接把静态内部option设置为None视为关闭数据库连接
    *db_static = None;
    
    Ok(())
}