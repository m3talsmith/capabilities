#[macro_use]
extern crate rocket;

use rocket_cors::CorsOptions;
use sqlx::postgres::PgPoolOptions;
use std::env;

mod api;
mod database;
mod models;
mod utils;

#[rocket::main]
async fn main() -> anyhow::Result<()> {
    let database_url = env::var("DATABASE_URL");
    let pool = PgPoolOptions::new()
        .connect(&*database_url.unwrap())
        .await?;
    let cors = CorsOptions::default().to_cors().unwrap();

    rocket::build()
        .manage(pool)
        .attach(cors)
        .mount("/api", routes![api::home::index])
        .mount(
            "/api/auth",
            routes![
                api::authentications::login,
                api::authentications::logout,
                api::authentications::register,
                api::authentications::unregister,
            ],
        )
        .launch()
        .await?;
    Ok(())
}
