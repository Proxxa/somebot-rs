use std::{borrow::Cow, io::Read};

use crate::prelude::*;

/// Image Resize command
///
/// This is never actually called because it's a slash command/
#[poise::command(
    slash_command,
    subcommands("percent", "scalar", "width", "height", "dimensions")
)]
pub async fn resize(_ctx: Context<'_>) -> Result<()> {
    Ok(())
}

/// Scale the image by a percentage
#[poise::command(slash_command)]
pub async fn percent(
    ctx: Context<'_>,
    #[description = "The image"] image: Attachment,
    #[description = "The percentage"] percent: f32,
) -> Result<()> {
    internal_resize(ctx, image, format!("{percent}%")).await
}

/// Scale the image by a scalar
#[poise::command(slash_command)]
pub async fn scalar(
    ctx: Context<'_>,
    #[description = "The image"] image: Attachment,
    #[description = "The percentage"] scalar: f32,
) -> Result<()> {
    internal_resize(ctx, image, format!("{}%", scalar * 100.)).await
}

/// Scale the image to have the given width
#[poise::command(slash_command)]
pub async fn width(
    ctx: Context<'_>,
    #[description = "The image"] image: Attachment,
    #[description = "The percentage"] width: usize,
) -> Result<()> {
    internal_resize(ctx, image, format!("{width}")).await
}

/// Scale the image to have the given height
#[poise::command(slash_command)]
pub async fn height(
    ctx: Context<'_>,
    #[description = "The image"] image: Attachment,
    #[description = "The percentage"] height: usize,
) -> Result<()> {
    internal_resize(ctx, image, format!("x{height}")).await
}

/// Scale the image to have the given dimensions
#[poise::command(slash_command)]
pub async fn dimensions(
    ctx: Context<'_>,
    #[description = "The image"] image: Attachment,
    #[description = "The width"] width: usize,
    #[description = "The height"] height: usize,
) -> Result<()> {
    internal_resize(ctx, image, format!("{width}x{height}!")).await
}

async fn internal_resize(
    ctx: Context<'_>,
    image: Attachment,
    scale_text: impl AsRef<str>,
) -> Result<()> {
    ctx.defer().await?;

    let mut file = tmpfile_from_attachment(image).await?;

    mogrify_file(file.path(), &["-resize", scale_text.as_ref()])?;

    let mut buf: Vec<u8> = Vec::new();
    file.reopen()?.read_to_end(&mut buf)?;

    ctx.send(|response| {
        response.attachment(serenity::AttachmentType::Bytes {
            data: Cow::Owned(buf),
            filename: file.file_name().to_string_lossy().into_owned(),
        })
    })
    .await?;

    drop(file);

    Ok(())
}
