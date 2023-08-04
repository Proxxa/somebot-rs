pub(crate) mod commands;
pub mod prelude;
mod routes;
pub mod util;

use rocket::fs::{FileServer, Options};
use std::path::PathBuf;

use crate::prelude::*;

#[shuttle_runtime::main]
async fn init(
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
    #[shuttle_static_folder::StaticFolder] static_folder: PathBuf,
) -> StdResult<PoiseRocketService, shuttle_runtime::Error> {
    // Get the discord token set in `Secrets.toml`
    let discord_token = secret_store
        .get("DISCORD_TOKEN")
        .context("'DISCORD_TOKEN' was not found")?;

    let poise = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                commands::hello(),
                commands::ping(),
                commands::_8ball().with_fn(|cmd| cmd.name = String::from("8ball")),
                commands::resize(),
                // commands::attach(),
            ],
            ..Default::default()
        })
        .token(discord_token)
        .intents(serenity::GatewayIntents::non_privileged())
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        });

    let rocket: shuttle_rocket::RocketService = rocket::build()
        .mount(
            "/tos",
            FileServer::new(static_folder.join("tos.txt"), Options::IndexFile).rank(1),
        )
        .mount(
            "/privacy",
            FileServer::new(static_folder.join("privacy_policy.txt"), Options::IndexFile).rank(1),
        )
        .mount(
            "/",
            FileServer::new(static_folder.join("public/index.html"), Options::IndexFile).rank(-999),
        )
        .mount("/public", FileServer::from(static_folder.join("public")))
        .into();

    Ok(PoiseRocketService { poise, rocket })
}
