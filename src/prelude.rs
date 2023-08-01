pub use anyhow::Context as _;
pub use poise::serenity_prelude as serenity;
pub use shuttle_poise::ShuttlePoise;
pub use shuttle_secrets::SecretStore;

pub struct Data {} // User data, which is stored and accessible in all command invocations
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;
