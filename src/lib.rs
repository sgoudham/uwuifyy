use linkify::{LinkFinder, LinkKind};
use std::error::Error;
use std::path::PathBuf;

use constants::ACTIONS;
use constants::ACTIONS_SIZE;
use constants::FACES;
use constants::FACES_SIZE;
use io::{UwUInFile, UwUOutFile};
use progress_bar::UwUProgressBar;
use seeder::UwUSeeder;

mod constants;
mod io;
mod progress_bar;
mod seeder;

#[derive(Debug)]
struct Modifiers {
    supplied_at_runtime: bool,
    words: f32,
    faces: f32,
    actions: f32,
    stutters: f32,
}

#[derive(Debug)]
pub struct UwUify {
    text: String,
    input: PathBuf,
    output: String,
    modifiers: Modifiers,
    random: bool,
    linkify: LinkFinder,
}

impl UwUify {
    pub fn new(
        text: Option<String>,
        infile: Option<PathBuf>,
        outfile: Option<String>,
        supplied_at_runtime: bool,
        words: f32,
        faces: f32,
        actions: f32,
        stutters: f32,
        random: bool,
    ) -> UwUify {
        // I dislike this

        let mut linkify = LinkFinder::new();
        linkify.kinds(&[LinkKind::Email, LinkKind::Url]);
        linkify.url_must_have_scheme(false);

        UwUify {
            text: text.unwrap_or_default(),
            input: infile.unwrap_or_default(),
            output: outfile.unwrap_or_default(),
            modifiers: Modifiers {
                supplied_at_runtime,
                words,
                faces,
                actions,
                stutters,
            },
            random,
            linkify,
        }
    }

    pub fn uwuify(&self) -> Result<(), Box<dyn Error>> {
        // Handle Text
        if !self.text.is_empty() {
            let uwu_text = self.uwuify_sentence(&self.text);

            // Handle Text Output
            if !self.output.is_empty() {
                if UwUOutFile::exists(&self.output) {
                    return Err(format!(
                        "File '{}' already exists, aborting operation",
                        &self.output
                    )
                    .into());
                }

                let mut uwu_out_file = match UwUOutFile::new(&self.output) {
                    Ok(uwu_out_file) => uwu_out_file,
                    Err(err) => return Err(err.into()),
                };

                let mut uwu_progress_bar = UwUProgressBar::new(uwu_text.len() as u64);

                match uwu_out_file.write_string(&uwu_text) {
                    Ok(_) => (),
                    Err(err) => return Err(err.into()),
                };

                uwu_progress_bar.update_progess(uwu_text.len());
                uwu_progress_bar.finish("UwU'ifying Complete!");
            } else {
                println!("{}", uwu_text);
            }
        } else {
            // Handle File I/O
            if UwUOutFile::exists(&self.output) {
                return Err(
                    format!("File '{}' already exists, aborting operation", &self.output).into(),
                );
            };

            let mut uwu_in_file = match UwUInFile::new(&self.input) {
                Ok(uwu_in_file) => uwu_in_file,
                Err(err) => return Err(err.into()),
            };
            let mut uwu_out_file = match UwUOutFile::new(&self.output) {
                Ok(uwu_out_file) => uwu_out_file,
                Err(err) => return Err(err.into()),
            };
            let mut uwu_progress_bar = UwUProgressBar::new(uwu_in_file.get_file_bytes());

            loop {
                let bytes_read_in = match uwu_in_file.read_until_newline() {
                    Ok(bytes_read_in) => bytes_read_in,
                    Err(err) => return Err(err.into()),
                };
                if bytes_read_in == 0 {
                    break;
                }

                let utf8_str = uwu_in_file.get_buffer_as_utf8_str();
                let uwu_sentence = self.uwuify_sentence(&utf8_str);
                match uwu_out_file.write_string_with_newline(&uwu_sentence) {
                    Ok(_) => (),
                    Err(err) => return Err(err.into()),
                };

                uwu_progress_bar.update_progess(bytes_read_in);
                uwu_in_file.clear_buffer();
            }

            uwu_progress_bar.finish("UwU'ifying Complete!");
        }

        Ok(())
    }

    fn uwuify_sentence(&self, text: &str) -> String {
        text.split_whitespace()
            .map(|word| {
                let uwu_word = self.uwuify_word(word.to_string());
                self.uwuify_spaces(uwu_word)
            })
            .collect::<Vec<String>>()
            .join(" ")
    }

    fn uwuify_word(&self, word: String) -> String {
        if self.linkify.links(&word).count() > 0 {
            return word;
        }

        let mut seeder = UwUSeeder::new(&word, self.random);
        if seeder.random() > self.modifiers.words {
            return word;
        }

        let word_bytes = word.as_bytes();
        let uwu_text_count = word.len();
        let mut uwu_text = String::new();

        for index in 0..uwu_text_count {
            let previous_previous_char =
                *word_bytes.get(index - 2).unwrap_or_else(|| &word_bytes[0]) as char;
            let previous_char =
                *word_bytes.get(index - 1).unwrap_or_else(|| &word_bytes[0]) as char;
            let current_char = word_bytes[index] as char;

            match current_char {
                'L' | 'R' => uwu_text.push('W'),
                'l' | 'r' => uwu_text.push('w'),
                'E' | 'e' => match previous_char {
                    'N' | 'n' => uwu_text.push_str(format!("y{}", current_char).as_str()),
                    'v' => match previous_previous_char {
                        'o' => {
                            uwu_text.pop();
                            uwu_text.pop();
                            uwu_text.push_str("uv");
                        }
                        _ => uwu_text.push(current_char),
                    },
                    _ => uwu_text.push(current_char),
                },
                'A' | 'I' | 'O' | 'U' | 'a' | 'i' | 'o' | 'u' => match previous_char {
                    'N' | 'n' => uwu_text.push_str(format!("y{}", current_char).as_str()),
                    _ => uwu_text.push(current_char),
                },
                _ => uwu_text.push(current_char),
            }
        }

        uwu_text
    }

    fn uwuify_spaces(&self, mut word: String) -> String {
        let mut seeder = UwUSeeder::new(&word, self.random);
        let random_value = seeder.random();

        if !self.modifiers.supplied_at_runtime {
            if random_value <= self.modifiers.faces {
                word = format!("{} {}", FACES[seeder.random_int(0, FACES_SIZE)], word);
            } else if random_value <= self.modifiers.actions {
                word = format!("{} {}", ACTIONS[seeder.random_int(0, ACTIONS_SIZE)], word);
            } else if random_value <= self.modifiers.stutters {
                let first_char_stutter = format!("{}-", word.chars().next().unwrap());
                word = format!(
                    "{}{}",
                    first_char_stutter.repeat(seeder.random_int(1, 2)),
                    word
                );
            }
        } else {
            if random_value <= self.modifiers.stutters {
                let first_char_stutter = format!("{}-", word.chars().next().unwrap());
                word = format!(
                    "{}{}",
                    first_char_stutter.repeat(seeder.random_int(1, 2)),
                    word
                );
            }
            if random_value <= self.modifiers.faces {
                word = format!("{} {}", FACES[seeder.random_int(0, FACES_SIZE)], word);
            }
            if random_value <= self.modifiers.actions {
                word = format!("{} {}", ACTIONS[seeder.random_int(0, ACTIONS_SIZE)], word);
            }
        }

        word
    }
}
