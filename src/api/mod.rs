use axum::{Router, routing::{delete, get, patch, post}};
use mongodb::Database;

use crate::api::handler::{create_todo,get_all_todos,get_todo, update_todo,delete_todo,delete_all_todos};



pub mod routes;
pub mod handler;

pub fn todo_routes() -> Router<Database> {
    Router::new()
        .route("/todos", post(create_todo))
        .route("/todos", get(get_all_todos))
        .route("/todos/{id}", get(get_todo))
     .route("/todos/{id}",patch(update_todo))
         .route("/todos/{id}", delete(delete_todo))
         .route("/todos", delete(delete_all_todos))
}