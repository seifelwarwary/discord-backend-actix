use jsonwebtoken::*;
use serde::{Deserialize, Serialize};
use argon2::{password_hash::SaltString, PasswordHasher};
use sea_orm::entity::*;
use r2d2_redis::{r2d2::PooledConnection, redis::Commands, RedisConnectionManager};
use sea_orm::{ DatabaseConnection};
use entity::*;
use chrono::{prelude::*, Days, Duration};
use uuid::Uuid;

static  SECRET_KEY_TOKEN:&str = "your_secret_key";
static  S_STR:&str="saltstrintstring";
static  AUDIENCE:&str="webApp";

pub fn create_token(email:&String,user_id:i64) ->  String {
    

let claims = serde_json::json!(
    Claims {
        user_id: user_id.to_string(),
        iss: email.clone(),
        aud: AUDIENCE.to_string(),
        exp: (Utc::now() + Duration::days(2)).timestamp(),
        iat: Utc::now().timestamp(),
    }
);


let secret_key = SECRET_KEY_TOKEN.as_bytes();


let header = Header::new(Algorithm::HS256);
let enc=EncodingKey::from_secret(secret_key);

let token = encode(&header, &claims, &enc).unwrap();
token
}

pub fn create_hash(password:&String ) -> String{
    // OsRng OS's random number generator 
    
    
    let salt=SaltString::from_b64(S_STR).expect("Invalid salt");
    let argon2=argon2::Argon2::default();
    let password_hash=argon2.hash_password(password.as_bytes(), &salt).unwrap().hash.unwrap().to_string();
     
    password_hash
}


#[derive(Debug, Serialize, Deserialize,Default)]
pub struct RefreshTokenRequest{
    pub token:String,
    pub user_id:i64,
}


#[derive(Debug, Serialize, Deserialize,Default)]
pub struct TokenResponse{
    pub token:String,
    pub user_id:i64,
    pub refresh_token:String,
}

#[derive(Debug, Serialize, Deserialize,Default)]
pub struct Claims {
   user_id: String,
   iss: String,
   aud: String,
    exp: i64,
    iat: i64,
}

pub fn is_valid_token(token:&str) -> bool{

let claims =  decode_token(token);
let now: i64 = Utc::now().timestamp() ;
let exp:i64=claims.exp;
let iat = claims.iat;
if ( exp> now) && (iat < now) {
    return true;
}
else {
    return false;
}

}



pub fn decode_token(token:&str) -> Claims {
    let key = DecodingKey::from_secret(SECRET_KEY_TOKEN.as_bytes());
    let mut validation = Validation::new(Algorithm::HS256);
    validation.set_audience( &[AUDIENCE]);

    
    let token = decode::<Claims>(&token, &key, &validation);
    match token {
        Ok(token) => token.claims,
        Err(_) => Claims::default(),
        
    }

}
pub async fn renew_token(token:RefreshTokenRequest,redis_conn:&mut PooledConnection<RedisConnectionManager>,db:&DatabaseConnection) -> (String,i64) {
     let result: Result<String,_> = redis_conn.get(token.token);
     match result {
         Ok(_) => (),
         Err(_) => return (String::new(),-1),
         
     }
    let result = result.unwrap();
     if token.user_id==result.parse::<i64>().unwrap() {
        let email = user::Entity::find_by_id(token.user_id).one(db).await.unwrap().unwrap().email;
        (create_token(&email,token.user_id),token.user_id)
    }
    else {
         (String::new(),-1) 
    }
}

pub fn create_refresh_token(user_id:i64,redis_conn:&mut PooledConnection<RedisConnectionManager>) -> String {
    let token = Uuid::new_v4().to_string()+Uuid::new_v4().to_string().as_str();
    let _:()=redis_conn.set(token.clone(),user_id).expect("failed to write to Redis");
    let _expire:() = redis_conn.expire(token.clone(),3600*24*30).expect("failed to set expiry time");
    token
}

pub fn delete_refresh_token(token:String,redis_conn:&mut PooledConnection<RedisConnectionManager>) -> bool {
    let result: Result<String,_> = redis_conn.get(token.clone());
    match result {
        Ok(_) => (),
        Err(_) => return false,
        
    }
    let _:() = redis_conn.del(token).expect("failed to delete token");
    true
}