use axum::{Json, extract::{Path, Query, State, Form}, http::StatusCode};
use bcrypt::{DEFAULT_COST, hash, verify};
use futures::{TryStreamExt};
use mongodb::{Database, bson::{doc, oid::ObjectId}};
use serde_json::{Value, json};

use crate::{models::todo::{CreateTodoRequest, LoginUser, RegisterUser, Todo, TodoQuery, UpdateTodoRequest, User, UserResponse}, utils::error::AppError};



pub async fn create_todo(State(db):State<Database>,
Form(payload):Form<CreateTodoRequest>
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

pub async fn get_todos_by_filter(
    State(db):State<Database>,
    Query(params):Query<TodoQuery>
)->Result<(StatusCode,Json<Value>),AppError>{

    let mut filter_doc = doc!{};
   
  if let Some(completed)=params.completed{
    filter_doc.insert("completed",completed);
  }

  if let Some(title)=params.title{

    if title.trim().is_empty(){
        return Err(AppError::ValidationError("Title cannot be empty".to_string()));
    }
    filter_doc.insert("title", title);
  }
  let collection = db.collection::<Todo>("todos");

  let result = collection.find(filter_doc).await?;

  let todos:Vec<Todo>=result.try_collect().await?;

  if todos.is_empty(){
    return Err(AppError::TodoNotFound("No todos found".to_string()));
  }

return Ok((StatusCode::OK,
Json(json!({
    "message":"Todos fetched successfully",
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

pub async fn register_user(
    State(db):State<Database>,
    Form(payload):Form<RegisterUser>
)->Result<(StatusCode,Json<Value>),AppError>{
        let collection = db.collection::<User>("users");
    // Registration logic goes here
 if payload.name.trim().is_empty(){
    return Err(AppError::ValidationError("Name cannot be empty".to_string()));
 }
    if payload.email.trim().is_empty(){
        return Err(AppError::ValidationError("Email cannot be empty".to_string()));
    }
    if let Some(_) = collection.find_one(doc!{"email":&payload.email}).await?{
    return Err(AppError::ValidationError("User Already exists".to_string()))}

    if payload.password.trim().is_empty(){        
        return Err(AppError::ValidationError("Password cannot be empty".to_string()));
    }

    let hashed_password = hash(&payload.password, DEFAULT_COST)
        .map_err(|_| AppError::ValidationError("Failed to hash password".to_string()))?;

    let new_user =User{
        id:None,
        name:payload.name,
        email:payload.email,
        password:hashed_password
    };
    let res = collection.insert_one(&new_user).await?;

    let user_response = UserResponse{
        id:Some(res.inserted_id.as_object_id().unwrap()),
        name:new_user.name,
        email:new_user.email
    };

    Ok((StatusCode::CREATED,
    Json(json!({
        "message":"User registered successfully",
        "user":user_response
    }))))
}

pub async fn login_user(State(db):State<Database>,
Form(payload):Form<LoginUser>)->Result<(StatusCode,Json<Value>),AppError>{

    let collection = db.collection::<User>("users");

    if payload.email.trim().is_empty()
{
    return Err(AppError::ValidationError("Email cannot be empty".to_string()));
}
let user = collection.find_one(doc!{"email":&payload.email}).await?;

let user = match user {
    Some(user) => user,
    None => return Err(AppError::ValidationError("User does not exist".to_string())),
};

if payload.password.trim().is_empty(){
    return Err(AppError::ValidationError("Password cannot be empty".to_string()));
}

let password_match = verify(&payload.password, &user.password)
    .map_err(|_| AppError::ValidationError("Password verification failed".to_string()))?;

if !password_match{
    return Err(AppError::ValidationError("Invalid password".to_string()));  
}

let user_response = UserResponse{
    id:user.id,
    name:user.name,
    email:user.email
};

Ok((StatusCode::OK,
Json(json!({
    "message":"Login successful",
    "user":user_response
}))))
}

pub async fn get_all_users(State(db):State<Database>)->Result<(StatusCode,Json<Value>),AppError>{

    let collection = db.collection::<User>("users");

let result = collection.find(doc!{}).await?;

let user :Vec<User>=result.try_collect().await?;
if user.is_empty(){
    return Err(AppError::TodoNotFound("No users found".to_string()));
}

Ok((StatusCode::OK,
Json(json!({
    "message":"Users fetched successfully",
    "user":user
}))))
}
