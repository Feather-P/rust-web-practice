use once_cell::sync::Lazy;
use tokio::sync::Mutex;
use tokio_rusqlite::Connection;

use super::error::DatabaseError;

static DB_INSTANCE: Lazy<Mutex<Option<Connection>>> = Lazy::new(|| Mutex::new(None));

pub async fn connect_database(path: Option<&str>) -> Result<(), DatabaseError> {
    let db_path = path.unwrap_or("database.db");
    let conn = Connection::open(db_path).await?;
    let mut db_static = DB_INSTANCE.lock().await;

    *db_static = Some(conn);

    Ok(())
}

async fn get_database() -> Result<DatabaseGuard, DatabaseError> {
    let db_static = DB_INSTANCE.lock().await;

    match &*db_static {
        Some(_) => Ok(DatabaseGuard::new(db_static)),
        None => return Err(DatabaseError::NotInitializedError),
    }
}

struct DatabaseGuard {
    guard: tokio::sync::MutexGuard<'static, Option<Connection>>,
}

impl DatabaseGuard {
    fn new(guard: tokio::sync::MutexGuard<'static, Option<Connection>>) -> Self {
        Self { guard }
    }

    /// 获取数据库连接的引用
    fn get(&self) -> Result<&Connection, DatabaseError> {
        match &*self.guard {
            Some(conn) => Ok(conn),
            None => Err(DatabaseError::NotInitializedError),
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

impl Drop for DatabaseGuard {
    fn drop(&mut self) {
        // MutexGuard会自动释放锁，这里不用干事
    }
}

pub async fn close_database() -> Result<(), DatabaseError> {
    let mut db_static = DB_INSTANCE.lock().await;

    // 关闭数据库连接
    if let Some(conn) = db_static.take() {
        match conn.close().await {
            Ok(()) => return Ok(()),
            Err(e) => return Err(DatabaseError::ConnectionError(e)),
        }
    }

    // 直接把静态内部option设置为None视为关闭数据库连接
    *db_static = None;

    Ok(())
}

pub async fn execute_with_lock<F, R>(operation: F) -> Result<R, DatabaseError>
where
    F: FnOnce(&Connection) -> Result<R, DatabaseError>,
{
    let db_guard = get_database().await?;
    let conn = db_guard.get()?;
    
    let result = operation(conn)?;
    Ok(result)
}