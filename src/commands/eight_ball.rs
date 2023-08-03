use crate::prelude::*;

const AFFIRMATIVE_ANSWERS: &[&str] = &[
    "It is certain.",
    "It is decidedly so.",
    "Without a doubt.",
    "Yes definitely.",
    "You may rely on it.",
    "As I see it, yes.",
    "Most likely.",
    "Outlook good.",
    "Yes.",
    "Signs point to yes.",
];

const AFFIRMATIVE_COLOR: (u8, u8, u8) = (0, 255, 0);

const NEGATIVE_ANSWERS: &[&str] = &[
    "Don't count on it.",
    "My reply is no.",
    "My sources say no.",
    "Outlook not so good.",
    "Very doubtful.",
];

const NEGATIVE_COLOR: (u8, u8, u8) = (255, 0, 0);

const NON_COMMITTAL_ANSWERS: &[&str] = &[
    "Reply hazy, try again.",
    "Ask again later.",
    "Better not tell you now.",
    "Cannot predict now.",
    "Concentrate and ask again.",
];

const NON_COMMITTAL_COLOR: (u8, u8, u8) = (255, 255, 0);

/// A simple 8ball command.
///
/// I wonder if another line will show up.
#[poise::command(slash_command)]
#[export_name = "8ball"]
pub async fn _8ball(
    ctx: Context<'_>,
    #[description = "The question supplied to the bot. (Does nothing)"] question: String,
) -> Result<(), Error> {
    let mrand1: u128 = rand::random();
    let mrand2: u128 = rand::random();
    let (message_pool, embed_color) = match mrand1 % 3 {
        0 => (AFFIRMATIVE_ANSWERS, AFFIRMATIVE_COLOR),
        1 => (NEGATIVE_ANSWERS, NEGATIVE_COLOR),
        2 => (NON_COMMITTAL_ANSWERS, NON_COMMITTAL_COLOR),
        _ => (NON_COMMITTAL_ANSWERS, NON_COMMITTAL_COLOR), // This shouldn't really need to be here
    };

    let pool_size = message_pool.len();

    let message = message_pool[(mrand2 % (pool_size as u128)) as usize];

    ctx.send(|reply| {
        reply.embed(|embed| {
            embed
                .title(question)
                .description(message)
                .color(embed_color)
        })
    })
    .await?;

    Ok(())
}
