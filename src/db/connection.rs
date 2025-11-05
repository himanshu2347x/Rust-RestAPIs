use mongodb::{Client, Database, error::Error};

use crate::config::{get_db_name, get_mongo_uri};



pub async fn connect_db()->Result<Database,Error>{
    let client = Client::with_uri_str(get_mongo_uri()).await?;
    let db = client.database(&get_db_name());
    return Ok(db);
}