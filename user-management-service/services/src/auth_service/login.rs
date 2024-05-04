 
use super::token::{self, create_hash};
use r2d2_redis::{r2d2::PooledConnection, redis::Commands, RedisConnectionManager};
use token::create_token;

use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use entity::*;
use sea_orm::entity::*;
use core::hash;
use std::time::{self, Duration, SystemTime};




#[derive(Deserialize, Debug)]
pub struct LoginRequest {
    email: String,
    password: String,
}
impl LoginRequest {
    pub fn new (email:&String,password:&String) -> LoginRequest{
        LoginRequest {email:email.clone(),password:password.clone()}
    }
}



pub async fn login_db(request:LoginRequest,db:&DatabaseConnection,redis_conn:&mut PooledConnection<RedisConnectionManager>) -> (String,i64) {
    
    let hashed_pass:Result<String,_>=redis_conn.get(&request.password);
    let hash_result:String;
    match hashed_pass {
        Ok(_) => hash_result=hashed_pass.unwrap(),
        Err(_) => hash_result=create_hash(&request.password),
    }
    
    
    let one = User::Entity::find().filter(User::Column::Email.eq(&request.email)).filter(User::Column::Password.eq(&hash_result)).one(db).await;
    match one {
        Ok(_) => (),
        Err(_) => return (String::new(),-1) ,
    }
    let one=one.unwrap();
    match one {
        Some(_) => (),
        None => return   (String::new(),-1) ,
    }
    let userId=one.unwrap().user_id.clone();
    let token = create_token(&request.email,userId);

    
    (token,userId)
}




