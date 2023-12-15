use std::time::Instant;

use crate::prelude::*;

/// A command for measuring response latency.
#[poise::command(slash_command)]
pub async fn ping(ctx: Context<'_>) -> Result<()> {
    let start = Instant::now();

    ctx.send(|reply| reply.content("Pinging..."))
        .await?
        .into_message()
        .await?
        .to_owned()
        .edit(ctx, |reply| {
            reply.content("").embed(|embed| {
                embed.title("Pong!").color((0, 255, 0)).description(format!(
                    "**Response** latency: `{}ms`",
                    start.elapsed().as_millis()
                ))
            })
        })
        .await?;

    Ok(())
}
