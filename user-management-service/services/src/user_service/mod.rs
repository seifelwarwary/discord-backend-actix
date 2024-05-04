use repository::user_repository::*;

use sea_orm::DatabaseConnection;
use entity::*;
use serde::{Deserialize, Serialize};

pub async fn get_users(ids:Vec<i64>,db:&DatabaseConnection) -> Vec<UserDto> {
    let users = get_users_by_id(ids,db).await.unwrap();
    let users = users.into_iter().map(|user| {
        user_to_user_dto(user)
    }).collect();
    users
}


fn user_to_user_dto(user:user::Model) -> UserDto {
    UserDto {
        id:user.user_id,
        username:user.username,
        email:user.email,
        name:user.name,
        created_at:user.created_at.to_string()
    }
}


#[derive(Debug, Serialize, Deserialize,Default)]
pub struct UserDto {
    pub id:i64,
    pub username:String,
    pub email:String,
    pub name:String,
    pub created_at:String
}