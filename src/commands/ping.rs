use std::time::Instant;

use crate::prelude::*;

///
#[poise::command(slash_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    let start = Instant::now();

    let rh = ctx.send(|reply| reply.content("Pinging...")).await?;

    rh.message().await?;

    let elapsed = Instant::now().duration_since(start);

    rh.edit(ctx, |reply| {
        reply.content("").embed(|embed| {
            embed
                .title("Pong!")
                .color((0, 255, 0))
                .description(format!("**REST**: `{}ms`", elapsed.as_millis()))
        })
    })
    .await?;

    Ok(())
}
