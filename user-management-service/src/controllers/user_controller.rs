use actix_web::{get, post, web::{self, Json}, HttpRequest, HttpResponse, Responder};
use macros_lib::authenticate;
use services::auth_service::token::is_valid_token;

use crate::{controllers::models::{ResponseModel, ResponseStatus}, AppState};





#[authenticate]
#[post("ids")]
pub async fn users_by_ids(req : HttpRequest,ids:Json<IdsRequest>,state:web::Data<AppState>) -> impl Responder {

    let db=&state.conn;
    let users = services::user_service::get_users(ids.0.ids, db).await;

    let count_users = users.len();
  

    HttpResponse::Ok().json(ResponseModel{status:ResponseStatus::Ok,message:&format!("count of users fetched : {count_users}"),data:users})

}

#[derive(Debug,Clone,serde::Deserialize,serde::Serialize)]
pub struct IdsRequest{
    pub ids:Vec<i64>
}