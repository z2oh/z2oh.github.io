extern crate dialoguer;
extern crate walkdir;

use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

use dialoguer::{Confirmation, Input};

use walkdir::{DirEntry, WalkDir};

fn main() {
    let path = "";
    let key = "";
    let data_section = true;

    let posts_iter = traverse_posts(path);
    for post in posts_iter {
        process_file(&post, &key, data_section);
    }
}

struct Posts {
    iterator: walkdir::IntoIter,
}

impl Iterator for Posts {
    type Item = DirEntry;

    fn next(&mut self) -> Option<DirEntry> {
        let next_entry = self.iterator.next();
        match next_entry {
            Some(entry) => {
                let entry = entry.unwrap();
                if entry.file_type().is_dir() {
                    self.next()
                } else {
                    Some(entry)
                }
            }
            None => None,
        }
    }
}

fn traverse_posts(path_to_posts_folder: &str) -> Posts {
    Posts {
        iterator: WalkDir::new(path_to_posts_folder).into_iter(),
    }
}

fn process_file(file_dir_entry: &DirEntry, key: &str, data_section: bool) {
    println!(
        "Processing file: {}",
        file_dir_entry.file_name().to_str().unwrap()
    );

    // Look for key
    let mut is_key_in_frontmatter = false;
    let mut key_line: String = String::default();
    let mut current_line_number: usize = 0;
    let mut key_line_number: usize = 0;
    let mut data_line_number: usize = 0;
    let mut frontmatter_end_line_number: usize = 0;

    {
        let file = File::open(file_dir_entry.path()).unwrap();
        let reader = BufReader::new(&file);
        for line in reader.lines() {
            let line = line.unwrap();
            if line.trim().starts_with(key) {
                is_key_in_frontmatter = true;
                key_line = line.clone();
                key_line_number = current_line_number;
            }

            if line.starts_with("data:") {
                data_line_number = current_line_number;
            }

            // Break once we have left frontmatter.
            if line == "---" {
                frontmatter_end_line_number = current_line_number;
                break;
            }

            current_line_number += 1;
        }
    }

    if is_key_in_frontmatter {
        println!("Found key on line {}", key_line_number);
        println!("{}", key_line.trim());
        if Confirmation::new("Would you like to update this key?")
            .interact()
            .unwrap()
        {
            let value = Input::new("What value should be set for this key?")
                .interact()
                .unwrap();

            // If we are writing to the data_section, the new line should be indented.
            let new_line = if data_section {
                format!("    {}: {}", key, value)
            } else {
                format!("{}: {}", key, value)
            };

            let mut lines: Vec<String> = Vec::new();
            {
                let file = File::open(file_dir_entry.path()).unwrap();
                let reader = BufReader::new(&file);
                lines = reader.lines().map(|l| l.unwrap()).collect();
                lines[key_line_number] = new_line;
            }
            {
                fs::remove_file(file_dir_entry.path()).unwrap();
                let mut file = File::create(file_dir_entry.path()).unwrap();
                for line in lines {
                    file.write_all(&line.into_bytes());
                    file.write(&[b'\n']);
                }
            }
        }
    } else {
        println!("Key not found.");
        if Confirmation::new("Would you like to add this key?")
            .interact()
            .unwrap()
        {
            let value = Input::new("What value should be set for this key?")
                .interact()
                .unwrap();

            // If we are writing to the data_section, the new line should be indented.
            let new_line = if data_section {
                format!("    {}: {}", key, value)
            } else {
                format!("{}: {}", key, value)
            };

            let mut lines: Vec<String> = Vec::new();
            {
                let file = File::open(file_dir_entry.path()).unwrap();
                let reader = BufReader::new(&file);
                lines = reader.lines().map(|l| l.unwrap()).collect();
                if data_section {
                    lines.insert(data_line_number + 1, new_line);
                } else {
                    lines.insert(frontmatter_end_line_number, new_line);
                }
            }
            {
                fs::remove_file(file_dir_entry.path()).unwrap();
                let mut file = File::create(file_dir_entry.path()).unwrap();
                for line in lines {
                    file.write_all(&line.into_bytes());
                    file.write(&[b'\n']);
                }
            }
        }
    }

    println!();
}
