use std::fs::File;

use once_cell::sync::Lazy;
use tracing::info;

#[rocket::get("/")]
pub fn index() -> &'static str {
    "Hello!"
}

const TOS_TIMESTAMP: Lazy<String> = Lazy::new(|| {
    info!("TOS file path: {}", concat!(env!("CARGO_MANIFEST_DIR"),"/src/","./static_data/tos.txt"));
    let time = File::open(concat!(env!("CARGO_MANIFEST_DIR"),"/src/","./static_data/tos.txt"))
        .expect("Failed to open TOS text file")
        .metadata()
        .expect("Failed to get TOS file metadata")
        .created()
        .expect("Failed to get TOS file created timestamp");

    let dt = chrono::DateTime::<chrono::Utc>::from(time);

    format!("{}", dt.format("%A, %B %d %Y %H:%M:%S"))
});

#[rocket::get("/tos")]
pub fn tos() -> String {
    format!("Last Updated {}\n\n{}", TOS_TIMESTAMP.clone(), include_str!("./static_data/tos.txt"))
}

const PRIVACY_TIMESTAMP: Lazy<String> = Lazy::new(|| {
    let time = File::open(concat!(env!("CARGO_MANIFEST_DIR"),"/src/","./static_data/privacy_policy.txt"))
        .expect("Failed to open privacy policy text file")
        .metadata()
        .expect("Failed to get privacy policy file metadata")
        .created()
        .expect("Failed to get privacy policy file created timestamp");

        let dt = chrono::DateTime::<chrono::Utc>::from(time);
    
        format!("{}", dt.format("%A, %B %d %Y %H:%M:%S"))
});

#[rocket::get("/privacy")]
pub fn privacy_policy() -> String {
    format!("Last Updated {}\n\n{}", PRIVACY_TIMESTAMP.clone(), include_str!("./static_data/privacy_policy.txt"))
}
