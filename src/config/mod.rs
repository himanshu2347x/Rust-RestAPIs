use std::env::var;



pub fn get_mongo_uri()->String{
    return var("MONGO_URI").expect("MONGO URI must be set");
}

pub fn get_db_name()->String{
    return var("DATABASE_NAME").unwrap_or("todo_db".to_string());
}