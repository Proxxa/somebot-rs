pub use anyhow::Context as _;
pub use anyhow::Error;
pub use poise::serenity_prelude as serenity;
pub use serenity::model::prelude::Attachment;
pub use shuttle_poise::ShuttlePoise;
pub use shuttle_secrets::SecretStore;

/// User data, which is stored and accessible in all command invocations
pub struct Data {}
pub type Context<'a> = poise::Context<'a, Data, Error>;
pub type StdResult<T, E> = std::result::Result<T, E>;
pub type Result<T> = StdResult<T, Error>;

pub use crate::util::{mogrify_file, tmpfile_from_attachment, PoiseRocketService, With};
