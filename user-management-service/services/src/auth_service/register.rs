

use std::time::SystemTime;

use chrono::Utc;
use sea_orm::{ActiveModelBehavior, ActiveModelTrait, DatabaseConnection, Value};
use entity::{ *};
use serde::{Deserialize, Serialize};
use r2d2_redis::{r2d2::PooledConnection, redis::Commands, RedisConnectionManager};

use super::token;

#[derive(Deserialize, Debug,Serialize,Clone)]
pub struct RegisterRequest {
    
   pub username: String,
   pub password: String,
   pub email: String,
   pub tag_name: String,
   pub name: String,
   pub phone_number: String,
   pub photo_url: String,
    
}
pub async fn register_db(data: RegisterRequest, db: &DatabaseConnection,redis_conn:&mut PooledConnection<RedisConnectionManager>) -> i64 {
    
    let pass=data.password.clone();
    let mut user = User::ActiveModel::new();
    let t1=SystemTime::now();
    let hash_result = token::create_hash(&pass);
    
    let t2=SystemTime
    ::now();
    println!("{:?}",t2.duration_since(t1).unwrap());
    user.set(entity::User::Column::Username, Value::from(data.username));
    user.set(entity::User::Column::Email, Value::from(data.email));
    user.set(entity::User::Column::TagName, Value::from(data.tag_name));
    user.set(entity::User::Column::Name, Value::from(data.name));
    user.set(entity::User::Column::PhoneNumber, Value::from(data.phone_number));
    user.set(entity::User::Column::PhotoUrl, Value::from(data.photo_url));
    user.set(entity::User::Column::TagNumber, Value::from(0));
    user.set(entity::User::Column::Password, hash_result.clone().into());
    let naive_date_time = Utc::now().date_naive();
    
    user.set(entity::User::Column::CreatedAt, Value::from(naive_date_time));
    let user = user.insert(db).await;
        
    match user {
        Ok(user1) => {
            let i :i64=user1.user_id;
            let _:Result<(), r2d2_redis::redis::RedisError> = redis_conn.set(&pass,&hash_result);
            i
        },
        Err(_) => -1,
    }
}