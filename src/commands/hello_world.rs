use crate::prelude::*;

/// A simple "Hello World" command.
///
/// I wonder if another line will show up.
#[poise::command(slash_command)]
pub async fn hello(ctx: Context<'_>) -> Result<(), Error> {
    ctx.send(|reply| reply.ephemeral(true).content("Hello, World!"))
        .await?;

    Ok(())
}
