#![allow(dead_code)]
use std::{path::PathBuf, ffi::OsStr, fs::{OpenOptions, File}, ops::{DerefMut, Deref}, io::{Write, Seek}};
pub(super) type IoResult<T> = std::io::Result<T>;

pub struct TempFile(pub File, pub PathBuf);

impl Deref for TempFile {
    type Target = File;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for TempFile {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl TempFile {
    pub fn new(path: &PathBuf) -> IoResult<Self> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create_new(true)
            .open(path)?;

        Ok(Self(file, path.clone()))
    }

    pub fn with_data(path: &PathBuf, buf: Vec<u8>) -> IoResult<Self> {
        let mut this = Self::new(path)?;

        this.write_all(&buf)?;
        this.seek(std::io::SeekFrom::Start(0))?;

        Ok(this)
    }

    pub fn path(&self) -> &PathBuf {
        &self.1
    }

    pub fn file_name(&self) -> &OsStr {
        self.1.file_name().unwrap()
    }

    pub fn reopen(&mut self) -> IoResult<&mut Self> {
        self.0 = OpenOptions::new()
            .read(true)
            .write(true)
            .create(false)
            .open(&self.1)?;

        Ok(self)
    }
}

impl Drop for TempFile {
    fn drop(&mut self) {
        std::fs::remove_file(self.1.clone()).unwrap();
    }
}