use actix_web::web;

mod models;

pub mod login_controller;
pub mod user_controller;
pub mod server_controller;
pub mod friends_controller;

pub fn config(cfg: &mut web::ServiceConfig){
     cfg.service(
    web::scope("user")
                .service(user_controller::users_by_ids))
        .service(
            web::scope("friends")
                .service(friends_controller::echo))
                .service(
            web::scope("server")
                        .service(server_controller::echo))
            .service(
            web::scope("")
                    .service(login_controller::login)
                    .service(login_controller::register_user)
                    .service(login_controller::refresh_token))
            .service(
        web::scope("server")
                    .service(server_controller::echo))
                    ;
}


