use axum::{Json, extract::{Path, State, path}, http::StatusCode};
use futures::{TryStreamExt};
use mongodb::{Database, bson::{doc, oid::ObjectId}};
use serde_json::{Value, json};

use crate::{models::todo::{CreateTodoRequest, Todo, UpdateTodoRequest}, utils::error::AppError};



pub async fn create_todo(State(db):State<Database>,
Json(payload):Json<CreateTodoRequest>
)->Result<(StatusCode,Json<Value>),AppError>{
    if payload.title.trim().is_empty() {
        return Err(AppError::ValidationError("Title cannot be empty".to_string()));
    }
    let collection = db.collection::<Todo>("todos");

    let new_todo =Todo{
        id:None,
        title:payload.title,
        completed:false
    };
        print!("api called successfuulllyyyy");

   let res = collection.insert_one(&new_todo).await?;

  let created_todo=Todo{
    id:Some(res.inserted_id.as_object_id().unwrap()),
    title:new_todo.title,
    completed:new_todo.completed
  };

return Ok((
    StatusCode::CREATED,
    Json(json!({
         "message": "Todo created successfully",
            "todo": created_todo
    })))
);

}

pub async fn get_all_todos(
    State(db):State<Database>
)->Result<(StatusCode,Json<Value>),AppError>{
   
   let collection=db.collection::<Todo>("todos");

let res = collection.find(doc! {}).await?;

let todos :Vec<Todo>=res.try_collect().await?;


    return Ok((StatusCode::OK,
    Json(json!({
        "message":"Todo fetched successsfuly",
         "total_todos":todos.len(),
         "todos":todos
    }))));
}
pub async fn get_todo(State(db):State<Database>,
Path(id):Path<String>
)->Result<(StatusCode,Json<Value>),AppError>{
   let object_id=ObjectId::parse_str(&id).map_err(|_| AppError::InvalidObjectId(id.clone()))?;

  let collection = db.collection::<Todo>("todos");
  let todo = collection.find_one(doc!{"_id":object_id}).await?;

 if let Some(todo)=todo{
    Ok((StatusCode::OK,
    Json(json!({
          "message": "Todo found successfully",
                "todo": todo
    }))
    ))
 }
 else{
    return Err(AppError::TodoNotFound(id));
 }
}
 pub async fn update_todo(
    Path(id): Path<String>,
    State(db): State<Database>,
    Json(payload): Json<UpdateTodoRequest>,
)-> Result<(StatusCode,Json<Value>),AppError> {

    let object_id = ObjectId::parse_str(&id).map_err(|_| AppError::InvalidObjectId(id.clone()))?;

    let collection = db.collection::<Todo>("todos");

    let mut updated_doc = doc! {};
    if let Some(title)=payload.title{

        if title.trim().is_empty(){
            return Err(AppError::ValidationError("Title cannot be empty".to_string()));
        }
        updated_doc.insert("title", title);
    }

    if let Some(completed)=payload.completed{
        updated_doc.insert("completed", completed);
    }

    if updated_doc.is_empty(){
        return Err(AppError::ValidationError("No fields to update".to_string()));
    }

    let result = collection.update_one(doc!{"_id":object_id}, doc!{
        "$set":updated_doc
    }).await?;

if result.matched_count == 0 {
    return Err(AppError::ValidationError(format!(
        "Todo with id {} not found",
        object_id
    )));
}
  
  let updated_todo = collection.find_one(
    doc!{"_id":object_id}
  ).await?;

return Ok((StatusCode::OK,
Json(json!({
    "message":"Todo updated successfully",
    "todo":updated_todo
}))));
}

pub async fn delete_todo(
    Path(id): Path<String>,
    State(db): State<Database>
)->Result<(StatusCode,Json<Value>),AppError>{
  let object_id = ObjectId::parse_str(&id).map_err(|_| AppError::InvalidObjectId(id.clone()))?;

  let collection = db.collection::<Todo>("todos");

  collection.delete_one(doc!{
    "_id":object_id
  }).await?;

    return Ok(( StatusCode::OK,
    Json(json!({
        "message":"todo Deleted Succesfuuly",
    }))));
}
pub async fn delete_all_todos(
    State(db): State<Database>
)->Result<(StatusCode,Json<Value>),AppError>{


  let collection = db.collection::<Todo>("todos");

let result =collection.delete_many(doc!{}).await?;

if result.deleted_count==0{
     return Err(AppError::TodoNotFound("No todos found to delete".to_string()));
}
    return Ok((
        StatusCode::OK,
        Json(json!({
            "message": "Todos deleted successfully",
            "deleted_count": result.deleted_count
        }))
    ));

}
