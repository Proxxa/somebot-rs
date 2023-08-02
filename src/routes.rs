#[rocket::get("/")]
pub fn index() -> &'static str {
    "Hello!"
}

#[rocket::get("/tos")]
pub fn tos() -> &'static str {
    util_macros::legal_file!("./static_data/tos.txt")
}

#[rocket::get("/privacy")]
pub fn privacy_policy() -> &'static str {
    util_macros::legal_file!("./static_data/privacy_policy.txt")
}
