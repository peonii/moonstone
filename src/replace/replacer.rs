use std::{fs, path::PathBuf, str::FromStr};

use crate::{home, Error};

pub struct Replacer {
    contents: String,
}

impl FromStr for Replacer {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        Ok(Self {
            contents: s.to_string(),
        })
    }
}

impl Replacer {
    pub fn from_path(path: &PathBuf) -> Result<Self, Error> {
        if !path.exists() {
            return Err(format!("Can't compile file {} - doesn't exist", path.display()).into());
        }

        let contents = fs::read_to_string(path)?;

        Self::from_str(&contents)
    }

    pub fn replace(&self) -> Result<String, Error> {
        let mut new_text = String::new();

        for line in self.contents.lines() {
            let mut indents = line;
            let line = line.trim_start();

            if let Some(chars_till_first_char) = indents.find(line) {
                indents = if let Some(c) = indents.get(0..chars_till_first_char) {
                    c
                } else {
                    new_text.push('\n');
                    continue;
                };
            }

            if !line.starts_with("%lib") {
                new_text.push_str(indents);
                new_text.push_str(line);
                new_text.push('\n');
                continue;
            }

            let library_name = match line.split_whitespace().nth(1) {
                Some(n) => n,
                None => return Err("Couldn't get library name!".into()),
            };

            let mut library_path = home!();

            library_path.push(".mst");
            library_path.push("libs");
            library_path.push(library_name);
            library_path.set_extension("mh");

            println!("Linking library {}.mh...", library_name);

            if !library_path.exists() {
                return Err(format!(
                    "Error adding library {library_name} - library does not exist!"
                )
                .into());
            }

            let library = fs::read_to_string(&library_path)?;

            new_text.push('\n');
            new_text.push_str(format!("/// Library {library_name}.mh ///").as_str());
            new_text.push('\n');
            for library_line in library.lines() {
                new_text.push_str(indents);
                new_text.push_str(library_line);
                new_text.push('\n');
            }
            new_text.push_str("/// Library end ///");
            new_text.push('\n');
        }

        new_text.push_str("\n\n// Compiled with moonstone");
        new_text.push_str("\n// Tool made by peony#6666");
        new_text.push_str("\n// https://github.com/peonii/moonstone");

        Ok(new_text)
    }

    pub fn replace_to_file(&self, path: &PathBuf) -> Result<(), Error> {
        let replaced = self.replace()?;

        fs::write(path, replaced)?;

        Ok(())
    }
}
