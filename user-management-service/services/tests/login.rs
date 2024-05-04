#![cfg(test)]
use ::entity::user;
use sea_orm::*;
use services::login::*;
 use sea_orm::prelude::Date;




#[tokio::test] 
async fn login_with_token(){
    let email="seifelwarwary@gmail.com".to_string();
    let password="123".to_string();
    let request=LoginRequest::new(&email, &password);
    let db = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results([
                // First query result
               vec![ user::Model {
                    name: "seif".to_owned(),
                    user_id: 4,
                    tag_name: "seif".to_owned(),
                    tag_number: 12,
                    photo_url: "seif".to_owned(),
                    username: "seif".to_owned(),
                    email:"seifelwarwary@gmail.com".to_owned(),
                    phone_number: "1234567890".to_owned(),
                    password:"RVB3lPDJjBEFDSOkPsFyuvpxktYiWzafpYwqHvue3UM".to_owned(),
                    created_at: Date::from_ymd_opt(2024,4,24).unwrap()
                }]
                ]).into_connection();
    let login_db = login_db(request, &db).await;
    let token="eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzUxMiJ9.eyJleHAiOjE2NTYwOTkyMDAsImlhdCI6MTY1NjA5NTYwMCwidXNlcl9pZCI6NCwidXNlcm5hbWUiOiJzZWlmZWx3YXJ3YXJ5QGdtYWlsLmNvbSJ9.YcykG_GCpc2cK7HsxtogZuG53TcRW-UIIxsR58pwWkDvZ1rtPNckVxiR9yXKVuUVdPThE4Sf5A7PC-lFavthNw".to_string();
    assert_eq!(login_db,token);
}