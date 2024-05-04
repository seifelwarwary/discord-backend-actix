mod controllers;
use actix_web::{web, App, HttpServer};
use controllers::config;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use sea_orm::{ConnectOptions, Database, DatabaseConnection};

use dotenv::dotenv;
use r2d2_redis::{r2d2, RedisConnectionManager};
use std::{env, fs::OpenOptions};
use tokio::time::Duration;
use tracing::{log, Level};
use tracing_subscriber::Layer;
use tracing_subscriber::{self, filter, fmt, layer::SubscriberExt, Registry};

#[derive(Debug, Clone)]
struct AppState {
    conn: DatabaseConnection,
    redis_conn: r2d2::Pool<RedisConnectionManager>,
}

async fn setup_redis(database_url: String) -> r2d2::Pool<RedisConnectionManager> {
    let manager = RedisConnectionManager::new(database_url).unwrap();
    r2d2::Pool::builder().build(manager).unwrap()
}

pub async fn setup_db(database_url: String) -> DatabaseConnection {
    let mut opt = ConnectOptions::new(&database_url);
    opt.max_connections(10000)
        .min_connections(1)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Info)
        .set_schema_search_path("public");

    let db: DatabaseConnection = Database::connect(opt).await.unwrap();
    db
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // todo!("add authentication middleware");

    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let redis_url = env::var("REDIS_URL").expect("REDIS_URL must be set");

    let log_file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("logs.log")
        .unwrap();

    let subscriber = Registry::default()
        // .with(
        //     // stdout layer, to view everything in the console
        //     fmt::layer().compact().with_ansi(true).with_filter(filter::LevelFilter::from_level(Level::INFO)),
        // )
        .with(
            // log-error file, to log the errors that arise
            fmt::layer()
                .json()
                .with_writer(log_file)
                .with_filter(filter::LevelFilter::from_level(Level::INFO)),
        );
    tracing::subscriber::set_global_default(subscriber).unwrap();

    let db = setup_db(database_url).await;

    let redis_db = setup_redis(redis_url).await;

    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("key.pem", SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file("cert.pem").unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                conn: db.clone(),
                redis_conn: redis_db.clone(),
            }))
            .configure(config)
    })
    .bind_openssl("0.0.0.0:8080", builder)?
    .run()
    .await
}
