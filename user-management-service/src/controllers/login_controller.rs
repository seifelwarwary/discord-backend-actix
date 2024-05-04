#![allow(dead_code,unused_variables,unreachable_code)]

use actix_web::{post, web::{self, Json}, HttpRequest, HttpResponse, Responder};

use crate::{controllers::models::{ResponseModel, ResponseStatus}, AppState};
use services::auth_service::{login::LoginRequest, login::*, register::{register_db, RegisterRequest}, token::{create_refresh_token, delete_refresh_token, renew_token, RefreshTokenRequest, TokenResponse}};



#[post("login")]
pub async fn login(req_body:HttpRequest,conn:web::Data<AppState>,login_data: Json<LoginRequest>) -> impl Responder {
    
    let db=&conn.conn;
    let redis_conn=&mut conn.redis_conn.get().unwrap();
    let (token,user_id) = login_db(login_data.0, db,redis_conn).await;
    if token.is_empty() {
    return HttpResponse::Ok().json(ResponseModel{status:ResponseStatus::BadRequest,message:"invalid email or password",data:()}) ;
    }
    let _refresh_token = create_refresh_token(user_id, redis_conn);

    let token_response = TokenResponse{token,refresh_token:_refresh_token,user_id};
    HttpResponse::Ok().json(ResponseModel{status:ResponseStatus::Ok,message:"login successful",data:token_response})
}


#[post("register")]
pub async fn register_user(req_body: HttpRequest,conn:web::Data<AppState>,user_data:Json<RegisterRequest>) -> impl Responder {
    let db=&conn.conn;
    let redis_conn=&mut conn.redis_conn.get().unwrap();
    let data = user_data.0;
    let user_id = register_db(data, db,redis_conn).await;
    if user_id>0 {
        
    return HttpResponse::Ok().json(ResponseModel{status:ResponseStatus::Ok,message:"user registered",data:user_id})
    }
   
    HttpResponse::BadRequest().json(ResponseModel{status:ResponseStatus::BadRequest,message:"Duplicate user",data:()})
    
}


#[post("token/refresh")]
pub async fn refresh_token(req_body: HttpRequest,conn:web::Data<AppState>,user_data:Json<RefreshTokenRequest>) -> impl Responder {
    let redis_conn=&mut conn.redis_conn.get().unwrap();
    let db=&conn.conn;
    let refresh_token=user_data.0.token.clone();
    let (token,user_id) = renew_token(user_data.0, redis_conn, db).await;
    let deleted_token= delete_refresh_token(refresh_token, redis_conn);
    if token.is_empty() || deleted_token==false {
    return HttpResponse::BadRequest().json(ResponseModel{status:ResponseStatus::BadRequest,message:"invalid token",data:()}) ;
    }
    

    let refresh_token = create_refresh_token(user_id, redis_conn);
    let token = TokenResponse{token,refresh_token,user_id};
    HttpResponse::Ok().json(ResponseModel{status:ResponseStatus::Ok,message:"token renewed",data:token})
}
