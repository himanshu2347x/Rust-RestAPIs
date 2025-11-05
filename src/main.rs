use tokio::net::TcpListener;
use std::{net::SocketAddr, process::exit};


use crate::{api::routes::create_router, db::connection::connect_db};

 mod db;
 mod config;
 mod models;
 mod utils;
 mod api;

 #[tokio::main]
async fn main(){
dotenvy::dotenv().ok();

let db = match connect_db().await{
    Ok(database)=>{
        println!("Mongo db connected successfully");
      database
},
    Err(err)=>{
        println!("Failed to connect to db : {}",err);
    exit(1)
    }
};
let app = create_router(db);

let address = SocketAddr::from(([127,0,0,1],3000));

let listener = TcpListener::bind(address).await.unwrap();

 println!("🚀 Server running on http://{}", address);
    println!("📚 API endpoints:");
    println!("   GET    /api/todos");
    println!("   POST   /api/todos");
    println!("   GET    /api/todos/{{id}}");
    println!("   PUT    /api/todos/{{id}}");
    println!("   DELETE /api/todos/{{id}}");
    println!("   DELETE /api/todos");

axum::serve(listener, app).await.unwrap();
}