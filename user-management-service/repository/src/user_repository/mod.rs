use sea_orm::{ActiveModelBehavior, ActiveModelTrait, DatabaseConnection, EntityTrait, QueryFilter, Value,ColumnTrait};
use entity::{ *};


pub async fn get_users_by_id(ids:Vec<i64>,db:&DatabaseConnection) -> Result<Vec<user::Model>,sea_orm::error::DbErr> {
    let users = User::Entity::find().filter(User::Column::UserId.is_in(ids)).all(db).await;
    users
}

pub async fn get_users_by_username(username:String,db:&DatabaseConnection) -> Result<user::Model,sea_orm::error::DbErr> {
    
    
    todo!()
    
    
}

