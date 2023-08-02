#![feature(proc_macro_span, proc_macro_internals)]

extern crate proc_macro;
use std::{fs::File, path::PathBuf};

use proc_macro::{TokenStream, Span};

#[proc_macro]
pub fn legal_file(item: TokenStream) -> TokenStream {
    let span = Span::call_site();
    let src = span.source_file();
    let invoc_path = src.path();
    let mut path = PathBuf::from(PathBuf::from(invoc_path).parent().unwrap());
    
    let path_string = item.to_string();
    path.extend(PathBuf::from(path_string.trim_matches('"')).iter());
    println!("{}", path.display());

    let file = File::open(path.clone()).expect("Failed to open file");

    let metadata = file.metadata().expect("Failed to get file metadata");
    let created_time = metadata.created().expect("Failed to get creation time of file");
    let time_string = chrono::DateTime::<chrono::Utc>::from(created_time).to_string();
    format!(r#"concat!("Last Updated {time_string}\n\n", include_str!({}))"#, path_string).parse().unwrap()
}