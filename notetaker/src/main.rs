use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{stdin, BufReader, Result, Write},
};

#[allow(unused)]
#[derive(Serialize, Deserialize)]
struct Note {
    title: String,
    content: String,
}

fn main() {
    create_note(String::from("notes/"));
}

fn create_note(path: String) {
    let title: String = read_line();
    let content: String = read_line();

    let note = Note {
        title: title.clone(),
        content: content.clone(),
    };

    let path: String = format!("{}{}.json", path, sanitize_file_name(&title));
    let mut file: File = File::create(path).expect("File creation failed");
    let serialized_note = serde_json::to_string(&note).expect("Seriaization failed");
    file.write_all(serialized_note.as_bytes());
}

// Ensures that the filename is valid, replaces non-alphanumeric characters with an underscore.
fn sanitize_file_name(name: &str) -> String {
    name.chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
        .collect()
}

// Reads a line of input from the user.
fn read_line() -> String {
    let mut input: String = String::new();
    match stdin().read_line(&mut input) {
        Ok(_) => {
            input = input.trim().to_string();
        }
        Err(error) => {
            eprintln!("Error: {}", error);
        }
    }

    input
}
