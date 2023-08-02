pub(crate) mod commands;
pub mod prelude;
mod routes;

use rocket::routes;

use crate::prelude::*;

#[shuttle_runtime::main]
async fn init(#[shuttle_secrets::Secrets] secret_store: SecretStore) -> Result<PoiseRocketService, shuttle_runtime::Error> {
    // Get the discord token set in `Secrets.toml`
    let discord_token = secret_store
        .get("DISCORD_TOKEN")
        .context("'DISCORD_TOKEN' was not found")?;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![commands::hello(), commands::ping(), {
                let mut com = commands::_8ball();
                com.name = String::from("8ball");
                com
            }],
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
        .mount("/", routes![routes::index, routes::tos, routes::privacy_policy])
        .into();

    Ok(PoiseRocketService {
        poise: framework,
        rocket,
    })
}
