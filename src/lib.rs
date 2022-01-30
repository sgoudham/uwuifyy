#![cfg_attr(test, feature(test))]

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
                #[cfg(not(test))]
                let stdout = std::io::stdout();
                #[cfg(not(test))]
                let mut out = UwUOutFile::new(stdout.lock());
                #[cfg(test)]
                let mut out = UwUOutFile::new(std::io::sink());
                self.uwuify_sentence(&self.text, &mut out)?;
                #[cfg(not(test))]
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
        text.split_whitespace().try_for_each(|word| {
            self.uwuify_word(word, out)?;
            out.write_string(" ")
        })
    }

    fn uwuify_word<T: Write>(
        &self,
        word: &str,
        out: &mut UwUOutFile<T>,
    ) -> Result<(), std::io::Error> {
        let mut seeder = UwUSeeder::new(word, self.random);
        let random_value = seeder.random();

        if !self.modifiers.supplied_at_runtime {
            if random_value <= self.modifiers.faces {
                out.write_string(FACES[seeder.random_int(0..FACES_SIZE)])?;
                out.write_bytes(b" ")?;
            } else if random_value <= self.modifiers.actions {
                out.write_string(ACTIONS[seeder.random_int(0..ACTIONS_SIZE)])?;
                out.write_bytes(b" ")?;
            } else if random_value <= self.modifiers.stutters {
                (0..seeder.random_int(1..2)).into_iter().try_for_each(|_| {
                    out.write_bytes(&word.as_bytes()[0..1])?;
                    out.write_bytes(b"-")
                })?;
            }
        } else {
            if random_value <= self.modifiers.stutters {
                (0..seeder.random_int(1..2)).into_iter().try_for_each(|_| {
                    out.write_bytes(&word.as_bytes()[0..1])?;
                    out.write_bytes(b"-")
                })?;
            }
            if random_value <= self.modifiers.faces {
                out.write_string(FACES[seeder.random_int(0..FACES_SIZE)])?;
                out.write_bytes(b" ")?;
            }
            if random_value <= self.modifiers.actions {
                out.write_string(ACTIONS[seeder.random_int(0..ACTIONS_SIZE)])?;
                out.write_bytes(b" ")?;
            }
        }

        if self.linkify.links(word).count() > 0 {
            return out.write_string(word);
        }

        let mut seeder = UwUSeeder::new(word, self.random);
        if seeder.random() > self.modifiers.words {
            return out.write_string(word);
        }

        let word_bytes = word.as_bytes();
        let uwu_text_count = word.len();

        (0..uwu_text_count).try_for_each(|index| {
            let previous_char =
                *word_bytes.get(index - 1).unwrap_or_else(|| &word_bytes[0]) as char;
            let current_char = word_bytes[index] as char;

            match current_char {
                'L' | 'R' => out.write_bytes(b"W"),
                'l' | 'r' => out.write_bytes(b"w"),
                'E' | 'e' => match previous_char {
                    'N' | 'n' => out.write_fmt(format_args!("y{}", current_char)),
                    _ => out.write_fmt(format_args!("{}", current_char)),
                },
                'A' | 'I' | 'O' | 'U' | 'a' | 'i' | 'o' | 'u' => match previous_char {
                    'N' | 'n' => out.write_fmt(format_args!("y{}", current_char)),
                    _ => out.write_fmt(format_args!("{}", current_char)),
                },
                _ => out.write_fmt(format_args!("{}", current_char)),
            }
        })
    }
}

#[cfg(test)]
mod tests {
    extern crate test;

    #[bench]
    fn uwu_bench(b: &mut test::Bencher) {
        let uwuify = super::UwUify::new(
            Some(include_str!("test.txt").to_string()),
            None,
            None,
            false,
            1.0,
            0.05,
            0.125,
            0.225,
            false,
        );
        b.iter(|| uwuify.uwuify());
    }
}
