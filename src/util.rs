use poise::FrameworkBuilder;

#[path = "./alltime_util.rs"]
mod alltime_util;
pub use alltime_util::*;

use crate::prelude::*;
use std::{
    env::temp_dir,
    fmt::Display,
    path::PathBuf,
    process::{Command, Output},
    sync::atomic::{AtomicUsize, Ordering},
};



pub async fn tmpfile_from_attachment(attach: Attachment) -> Result<TempFile> {
    let bytes: Vec<u8> = reqwest::get(attach.url)
        .await?
        .bytes()
        .await?
        .iter()
        .map(|borrowed| *borrowed)
        .collect();

    println!("Collected bytes: {}", bytes.len());

    let pat = temp_dir().join(format!(
        "{}{}",
        attach.id.0.to_string(),
        attach.filename.clone()
    ));

    // Result casting
    Ok(TempFile::with_data(&pat, bytes)?)
}

impl TryFrom<&[u8]> for TempFile {
    type Error = std::io::Error;

    fn try_from(data: &[u8]) -> IoResult<Self> {
        lazy_static::lazy_static! {
            static ref ID: AtomicUsize = AtomicUsize::new(0);
        }

        let path = std::env::temp_dir().join(format!("unknownfile{}", ID.fetch_add(1, Ordering::Relaxed)));

        Self::with_data(&path, Vec::from(data))
    }
}

pub struct PoiseRocketService {
    pub poise:
        FrameworkBuilder<Data, Box<(dyn std::error::Error + std::marker::Send + Sync + 'static)>>,
    pub rocket: shuttle_rocket::RocketService,
}

#[shuttle_runtime::async_trait]
impl shuttle_runtime::Service for PoiseRocketService {
    async fn bind(
        mut self,
        addr: std::net::SocketAddr,
    ) -> std::result::Result<(), shuttle_runtime::Error> {
        let binder = self.rocket.bind(addr);

        tokio::select! {
            _ = self.poise.run() => {},
            _ = binder => {},
        }

        Ok(())
    }
}

#[derive(Debug)]
pub(crate) struct StringError(String);

#[allow(dead_code)]
impl StringError {
    pub fn new(string: impl AsRef<str>) -> Self {
        Self(string.as_ref().to_string())
    }
}

impl Display for StringError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl std::error::Error for StringError {}

pub trait With {
    fn with_fn(mut self, lambda: impl FnOnce(&mut Self)) -> Self
    where
        Self: Sized,
    {
        lambda(&mut self);
        self
    }
}

impl<T> With for T {}

pub fn mogrify_file(path: &PathBuf, args: &[&str]) -> Result<Output> {
    let magick = Command::new("magick")
        .arg("mogrify")
        .args(args)
        .arg(path.to_str().unwrap())
        .output()?;

    if !magick.status.success() {
        return Err(Box::new(StringError::new(
            "An error occurred while modifying the image.",
        )));
    }

    Ok(magick)
}
