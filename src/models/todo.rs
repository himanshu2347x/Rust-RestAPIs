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
