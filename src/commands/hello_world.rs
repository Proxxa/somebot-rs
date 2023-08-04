use crate::prelude::*;

/// A simple "Hello World" command.
#[poise::command(slash_command)]
pub async fn hello(ctx: Context<'_>) -> Result<()> {
    ctx.send(|reply| reply.ephemeral(true).content("Hello, World!"))
        .await?;

    Ok(())
}
