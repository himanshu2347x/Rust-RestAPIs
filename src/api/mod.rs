use axum::{Router, routing::{delete, get, patch, post}};
use mongodb::Database;

use crate::api::handler::{create_todo, delete_all_todos, delete_todo, get_todo, get_todos_by_filter, update_todo};

pub mod routes;
pub mod handler;

pub fn todo_routes() -> Router<Database> {
    Router::new()
        .route("/todos", post(create_todo))
        .route("/todos", get(get_todos_by_filter))
        .route("/todos/{id}", get(get_todo))
     .route("/todos/{id}",patch(update_todo))
         .route("/todos/{id}", delete(delete_todo))
         .route("/todos", delete(delete_all_todos))
}