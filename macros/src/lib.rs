#![feature(proc_macro_span, proc_macro_internals)]

extern crate proc_macro;
use std::{fs::File, path::PathBuf};

use proc_macro::{TokenStream, Span};

#[proc_macro]
pub fn legal_file(item: TokenStream) -> TokenStream {

    // Turn input into PathBuf
    let path_string = item.to_string();
    let in_path = PathBuf::from(path_string.trim_matches('"'));

    let path = if !in_path.is_absolute() {
        // Get parent directory of calling file
        let span = Span::call_site();
        let src = span.source_file();
        let invoc_path = src.path();

        // Into a PathBuf
        let mut path = PathBuf::from(PathBuf::from(invoc_path).parent().unwrap());

        // Extend
        path.extend(in_path.iter());
        path

    } else { in_path };

    // Get the file
    let file = File::open(path.clone()).expect("Failed to open file");

    // Get file metadata
    let metadata = file.metadata().expect("Failed to get file metadata");

    // Get file created time
    let created_time = metadata.created().expect("Failed to get creation time of file");

    // Turn that into a lovely timestamp
    let time_string = chrono::DateTime::<chrono::Utc>::from(created_time).to_string();

    // Output code!
    format!(r#"concat!("Last Updated {time_string}\n\n", include_str!({}))"#, path_string).parse().unwrap()
}