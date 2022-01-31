use std::fs::File;
use std::io::{stdout, BufWriter, Error, ErrorKind, Write};
use std::path::Path;
use std::str::from_utf8_unchecked;

use indicatif::{ProgressBar, ProgressStyle};
use linkify::{LinkFinder, LinkKind};
use memmap::Mmap;

use constants::ACTIONS;
use constants::ACTIONS_SIZE;
use constants::FACES;
use constants::FACES_SIZE;
use seeder::UwUSeeder;

mod constants;
mod seeder;

macro_rules! progress_bar {
    ($bytes:expr) => {{
        let progress_bar = ProgressBar::new($bytes);
        progress_bar.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner:.magenta} [{elapsed_precise:.bold}] {msg:.green.bold}"),
        );
        progress_bar.set_message("UwU'ifying In Progress...");
        progress_bar.enable_steady_tick(30);

        progress_bar
    }};
}

#[derive(Debug)]
pub struct UwUify<'a> {
    text: &'a str,
    input: &'a str,
    output: &'a str,
    words: f64,
    faces: f64,
    actions: f64,
    stutters: f64,
    random: bool,
    is_runtime: bool,
    linkify: LinkFinder,
}

impl<'a> Default for UwUify<'a> {
    fn default() -> Self {
        Self {
            text: "",
            input: "",
            output: "",
            words: 1.0,
            faces: 0.05,
            actions: 0.125,
            stutters: 0.225,
            random: false,
            is_runtime: false,
            linkify: LinkFinder::new(),
        }
    }
}

impl<'a> UwUify<'a> {
    pub fn new(
        text: Option<&'a str>,
        infile: Option<&'a str>,
        outfile: Option<&'a str>,
        words: Option<&'a str>,
        faces: Option<&'a str>,
        actions: Option<&'a str>,
        stutters: Option<&'a str>,
        random: bool,
        is_runtime: bool,
    ) -> UwUify<'a> {
        let mut linkify = LinkFinder::new();
        linkify.kinds(&[LinkKind::Email, LinkKind::Url]);
        linkify.url_must_have_scheme(false);

        let mut uwuify = UwUify {
            text: text.unwrap_or_default(),
            input: infile.unwrap_or_default(),
            output: outfile.unwrap_or_default(),
            random,
            is_runtime,
            linkify,
            ..Default::default()
        };

        if let Some(words) = words {
            uwuify.words = words.parse::<f64>().unwrap();
        }
        if let Some(faces) = faces {
            uwuify.faces = faces.parse::<f64>().unwrap();
        }
        if let Some(actions) = actions {
            uwuify.actions = actions.parse::<f64>().unwrap();
        }
        if let Some(stutters) = stutters {
            uwuify.stutters = stutters.parse::<f64>().unwrap();
        }

        uwuify
    }

    pub fn uwuify(&mut self) -> Result<(), Error> {
        // Handle Text
        if !self.text.is_empty() {
            // Handle Text Output
            if !self.output.is_empty() {
                if Path::new(&self.output).exists() {
                    return Err(Error::new(
                        ErrorKind::AlreadyExists,
                        format!("File '{}' already exists, aborting operation", &self.output),
                    ));
                }

                let uwu_progress_bar = progress_bar!(self.text.len() as u64);
                self.uwuify_sentence(self.text, &mut BufWriter::new(File::create(&self.output)?))?;
                uwu_progress_bar.finish_with_message("UwU'ifying Complete!");
            } else {
                self.uwuify_sentence(self.text, &mut BufWriter::new(stdout().lock()))?;
            }
        } else {
            // Handle File I/O
            if Path::new(&self.output).exists() {
                return Err(Error::new(
                    ErrorKind::AlreadyExists,
                    format!("File '{}' already exists, aborting operation", &self.output),
                ));
            }

            let infile = File::open(&self.input)?;
            let uwu_progress_bar = progress_bar!(infile.metadata()?.len());
            self.uwuify_sentence(
                unsafe { from_utf8_unchecked(Mmap::map(&infile)?.as_ref()) },
                &mut BufWriter::new(File::create(&self.output)?),
            )?;
            uwu_progress_bar.finish_with_message("UwU'ifying Complete!");
        }

        Ok(())
    }

    pub fn uwuify_sentence<T: Write>(&mut self, text: &str, out: &mut T) -> Result<(), Error> {
        text.lines().try_for_each(|line| {
            line.split_whitespace()
                .map(|word_str| word_str.as_bytes())
                .try_for_each(|word| {
                    let mut seeder = UwUSeeder::new(word, self.random);
                    let random_value = seeder.random_float();

                    if !self.is_runtime {
                        if random_value <= self.faces {
                            out.write_all(FACES[seeder.random_int(0..FACES_SIZE)])?;
                        } else if random_value <= self.actions {
                            out.write_all(ACTIONS[seeder.random_int(0..ACTIONS_SIZE)])?;
                        } else if random_value <= self.stutters {
                            match word[0] {
                                b'L' | b'R' => out.write_all(b"W"),
                                b'l' | b'r' => out.write_all(b"w"),
                                byte => out.write_all(&[byte]),
                            }?;
                            out.write_all(b"-")?;
                        }
                    } else {
                        if random_value <= self.faces {
                            out.write_all(FACES[seeder.random_int(0..FACES_SIZE)])?;
                        }
                        if random_value <= self.actions {
                            out.write_all(ACTIONS[seeder.random_int(0..ACTIONS_SIZE)])?;
                        }
                        if random_value <= self.stutters {
                            match word[0] {
                                b'L' | b'R' => out.write_all(b"W"),
                                b'l' | b'r' => out.write_all(b"w"),
                                byte => out.write_all(&[byte]),
                            }?;
                            out.write_all(b"-")?;
                        }
                    }

                    if self
                        .linkify
                        .links(unsafe { from_utf8_unchecked(word) })
                        .count()
                        > 0
                        || random_value > self.words
                    {
                        out.write_all(word)?;
                    } else {
                        (0..word.len()).try_for_each(|index| match word[index] {
                            b'L' | b'R' => out.write_all(b"W"),
                            b'l' | b'r' => out.write_all(b"w"),
                            b'A' | b'E' | b'I' | b'O' | b'U' | b'a' | b'e' | b'i' | b'o' | b'u' => {
                                match word.get(index - 1).unwrap_or(&word[0]) {
                                    b'N' | b'n' => out.write_all(&[b'y', word[index]]),
                                    _ => out.write_all(&[word[index]]),
                                }
                            }
                            byte => out.write_all(&[byte]),
                        })?;
                    }
                    out.write_all(b" ")
                })?;
            out.write_all(b"\n")
        })
    }
}