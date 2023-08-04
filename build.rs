use std::{process::Command, fs::{OpenOptions, File}, io::{ErrorKind, Write}, env::temp_dir};

const PATH: &'static str = concat!(env!("CARGO_MANIFEST_DIR"), "/magick_already_installed");
const SCRIPT_LINK: &'static str =  "https://gist.githubusercontent.com/timoteostewart/16624088e656d336a2a862778788378a/raw/f529fba2c7d851932f1aa4b928c7c3fdf76021b5/i-imagemagick-latest.sh";

fn main() {
    let _ = std::fs::remove_file(PATH);
    
    match Command::new("magick").spawn() {
        Ok(_) => {
            let _ = OpenOptions::new().create_new(true).open(PATH);
        },
        Err(e) => {
            if e.kind() == ErrorKind::NotFound {
                install_magick();
            } else {
                panic!("Could not determine if magick is installed.");
            }
        }
    }
}

#[path = "./src/alltime_util.rs"]
mod util;
use util::*;


use tokio::runtime::Runtime;
macro_rules! block_future {
    ($ex:expr) => {
        Runtime::new().unwrap().block_on($ex)
    };
}

fn install_magick() {
    let script = block_future!(block_future!(reqwest::get(SCRIPT_LINK)).unwrap().text()).unwrap();

    let script_path = temp_dir().join("install_magick.sh");
    let buf: Vec<u8> = script.bytes().collect();

    let script = TempFile::with_data(&script_path, buf).expect("Failed to create script file");

    Command::new("sudo")
        .arg("bash")
        .arg(script_path)
        .output()
        .expect("Failed to run installation command");

    // Manually drop here to keep file until now.
    drop(script);

}