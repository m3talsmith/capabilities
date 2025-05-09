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
        .mount(
            "/api/my/skills",
            routes![
                api::my::user_skills::get_user_skills,
                api::my::user_skills::create_user_skill,
                api::my::user_skills::update_user_skill,
                api::my::user_skills::delete_user_skill,
            ],
        )
        .mount(
            "/api/my/user",
            routes![
                api::my::user::get_user,
                api::my::user::update_user,
                api::my::user::change_password,
            ],
        )
        .mount(
            "/api/my/backup-codes",
            routes![
                api::my::backup_codes::get_backup_codes,
                api::my::backup_codes::regenerate_backup_codes,
            ],
        )
        .mount(
            "/api/my/teams",
            routes![
                api::my::teams::get_teams,
                api::my::teams::get_team,
                api::my::teams::create_team,
                api::my::teams::update_team,
                api::my::teams::delete_team,
                api::my::teams::create_invitation,
                api::my::teams::get_team_activities,
                api::my::teams::get_team_activity,
                api::my::teams::create_team_activity,
                api::my::teams::update_team_activity,
                api::my::teams::delete_team_activity,
                api::my::teams::assign_team_activity,
                api::my::teams::pause_team_activity,
                api::my::teams::resume_team_activity,
                api::my::teams::unassign_team_activity,
                api::my::teams::reopen_team_activity,
            ],
        )
        .mount(
            "/api/my/invitations",
            routes![
                api::my::invitations::get_invitations,
                api::my::invitations::get_invitation,
                api::my::invitations::accept_invitation,
                api::my::invitations::reject_invitation,
            ],
        )
        .mount(
            "/api/teams",
            routes![
                api::teams::teams::get_teams,
                api::teams::teams::get_team,
                api::teams::invitations::get_invitations,
                api::teams::users::get_users,
            ],
        )
        .mount(
            "/api/invitations",
            routes![
                api::invitations::get_invitations,
                api::invitations::get_invitation,
            ],
        )
        .mount("/api/users", routes![api::users::get_users])
        .mount(
            "/api/my/activities",
            routes![
                api::my::activities::get_activities,
                api::my::activities::get_activity,
                api::my::activities::complete_activity,
                api::my::activities::pause_activity,
                api::my::activities::resume_activity,
                api::my::activities::reopen_activity,
            ],
        )
        .launch()
        .await?;
    Ok(())
}
