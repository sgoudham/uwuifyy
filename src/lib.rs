use linkify::{LinkFinder, LinkKind};
use std::error::Error;
use std::fs::File;
use std::io::Write;
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
    words: f64,
    faces: f64,
    actions: f64,
    stutters: f64,
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
        words: f64,
        faces: f64,
        actions: f64,
        stutters: f64,
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
            // Handle Text Output
            if !self.output.is_empty() {
                if std::path::Path::new(&self.output).exists() {
                    return Err(format!(
                        "File '{}' already exists, aborting operation",
                        &self.output
                    )
                    .into());
                }

                let mut uwu_out_file = UwUOutFile::new(File::create(&self.output)?);
                self.uwuify_sentence(&self.text, &mut uwu_out_file)?;

                let mut uwu_progress_bar = UwUProgressBar::new(self.text.len() as u64);

                uwu_progress_bar.update_progess(self.text.len());
                uwu_progress_bar.finish("UwU'ifying Complete!");
                Ok(())
            } else {
                let stdout = std::io::stdout();
                let mut out = UwUOutFile::new(stdout.lock());
                self.uwuify_sentence(&self.text, &mut out)?;
                out.write_newline()?;
                Ok(())
            }
        } else {
            // Handle File I/O
            if std::path::Path::new(&self.output).exists() {
                return Err(
                    format!("File '{}' already exists, aborting operation", &self.output).into(),
                );
            }

            let mut uwu_in_file = UwUInFile::new(&self.input)?;
            let mut uwu_out_file = UwUOutFile::new(File::create(&self.output)?);
            let mut uwu_progress_bar = UwUProgressBar::new(uwu_in_file.get_file_bytes());

            loop {
                let bytes_read_in = uwu_in_file.read_until_newline()?;
                if bytes_read_in != 0 {
                    self.uwuify_sentence(&uwu_in_file.buffer, &mut uwu_out_file)?;
                    uwu_out_file.write_newline()?;

                    uwu_progress_bar.update_progess(bytes_read_in);
                    uwu_in_file.clear_buffer();
                } else {
                    uwu_progress_bar.finish("UwU'ifying Complete!");
                    return Ok(());
                }
            }
        }
    }

    fn uwuify_sentence<T: Write>(
        &self,
        text: &str,
        out: &mut UwUOutFile<T>,
    ) -> Result<(), std::io::Error> {
        text.split_whitespace()
            .map(|word| {
                let uwu_word = self.uwuify_word(word.to_string());
                self.uwuify_spaces(uwu_word)
            })
            .try_for_each(|f| {
                out.write_string(&f)?;
                out.write_string(" ")
            })
    }

    fn uwuify_word(&self, word: String) -> String {
        use std::fmt::Write;
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
                    'N' | 'n' => uwu_text
                        .write_fmt(format_args!("y{}", current_char))
                        .unwrap(),
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
                    'N' | 'n' => uwu_text
                        .write_fmt(format_args!("y{}", current_char))
                        .unwrap(),
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
                word = format!("{} {}", FACES[seeder.random_int(0..FACES_SIZE)], word);
            } else if random_value <= self.modifiers.actions {
                word = format!("{} {}", ACTIONS[seeder.random_int(0..ACTIONS_SIZE)], word);
            } else if random_value <= self.modifiers.stutters {
                let first_char_stutter = format!("{}-", word.chars().next().unwrap());
                word = format!(
                    "{}{}",
                    first_char_stutter.repeat(seeder.random_int(1..2)),
                    word
                );
            }
        } else {
            if random_value <= self.modifiers.stutters {
                let first_char_stutter = format!("{}-", word.chars().next().unwrap());
                word = format!(
                    "{}{}",
                    first_char_stutter.repeat(seeder.random_int(1..2)),
                    word
                );
            }
            if random_value <= self.modifiers.faces {
                word = format!("{} {}", FACES[seeder.random_int(0..FACES_SIZE)], word);
            }
            if random_value <= self.modifiers.actions {
                word = format!("{} {}", ACTIONS[seeder.random_int(0..ACTIONS_SIZE)], word);
            }
        }

        word
    }
}
