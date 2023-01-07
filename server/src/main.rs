mod api;
mod models;
mod db;

use actix_web::{get, App, HttpResponse, HttpServer, Responder, web::Data};
use db::mongodb_repo::MongoRepo;
use std::env;
use dotenv::dotenv;
use anyhow;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().json("Hello! And welcome to the simple sec santa service.")
}

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let db_uri = env::var("MONGOURI")?;
    let db = MongoRepo::init(&db_uri).await?;
    let db_data = Data::new(db);
    HttpServer::new(move || {
        App::new().app_data(db_data.clone())
            .service(hello)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;

    Ok(())
}
