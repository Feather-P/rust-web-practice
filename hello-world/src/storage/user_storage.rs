use crate::connection::*;
use crate::database::DatabaseError;
pub async fn init_user_table() -> Result<(), DatabaseError> {
    if let Err(e) = connect_database(None).await {

    }
}