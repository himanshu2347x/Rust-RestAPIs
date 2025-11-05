use axum::Router;
use mongodb::Database;

use crate::api::todo_routes;


pub fn create_router(db:Database)->Router{
   return  Router::new().nest("/api", todo_routes()).with_state(db);
}