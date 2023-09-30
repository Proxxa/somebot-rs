pub(crate) mod commands;
pub mod prelude;
mod routes;
pub mod util;

use rocket::fs::{FileServer, Options};
use std::path::PathBuf;

use crate::prelude::*;

lazy_static::lazy_static! {
    static ref STATIC_FOLDER: PathBuf = PathBuf::from("./static");
}

#[shuttle_runtime::main]
async fn init(
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
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
            FileServer::new(STATIC_FOLDER.join("tos.txt"), Options::IndexFile).rank(1),
        )
        .mount(
            "/privacy",
            FileServer::new(STATIC_FOLDER.join("privacy_policy.txt"), Options::IndexFile).rank(1),
        )
        .mount(
            "/",
            FileServer::new(STATIC_FOLDER.join("public/index.html"), Options::IndexFile).rank(-999),
        )
        .mount("/public", FileServer::from(STATIC_FOLDER.join("public")))
        .into();

    Ok(PoiseRocketService { poise, rocket })
}
