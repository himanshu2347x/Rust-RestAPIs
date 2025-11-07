use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize)]
pub struct Todo{
      #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id:Option<ObjectId>,
    pub title:String,
    #[serde(default)]
    pub completed:bool
}
#[derive(Serialize,Deserialize)]
pub struct CreateTodoRequest{
    pub title:String,
    pub completed:Option<bool>
}
#[derive(Serialize,Deserialize)]
pub struct  UpdateTodoRequest{
    pub title:Option<String>,
    pub completed:Option<bool>
}

#[derive(Serialize,Deserialize)]
pub struct TodoQuery{
    pub completed:Option<bool>,
    pub title:Option<String>
}
#[derive(Serialize,Deserialize)]
pub struct User{
     #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id:Option<ObjectId>,
    pub name :String,
    pub email:String,
    pub password:String
}

#[derive(Serialize,Deserialize)]
pub struct RegisterUser{
    pub name :String,
    pub email:String,
    pub password:String

}

#[derive(Serialize,Deserialize)]
pub struct UserResponse{
    pub id:Option<ObjectId>,
    pub name:String,
    pub email:String
}


#[derive(Serialize,Deserialize)]
pub struct LoginUser{
    pub email:String,
    pub password:String
}
