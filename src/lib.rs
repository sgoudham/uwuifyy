#![cfg_attr(all(feature = "bench", test), feature(test))]

use std::fs::File;
use std::io::{BufWriter, Error, ErrorKind, Write};
use std::path::Path;
use std::str::from_utf8_unchecked;

use linkify::{LinkFinder, LinkKind};
use memmap::Mmap;

use constants::ACTIONS_SIZE;
use constants::FACES;
use constants::FACES_SIZE;
use constants::{ACTIONS, ASCII, ASCII_SIZE};
use rand::SeedableRng;
use rand_xoshiro::Xoshiro256Plus;

mod constants;

macro_rules! progress_bar {
    () => {{
        let progress_bar = indicatif::ProgressBar::new_spinner()
            .with_style(
                indicatif::ProgressStyle::default_spinner()
                    .template("{spinner:.magenta} [{elapsed_precise:.bold}] {msg:.green.bold}"),
            )
            .with_message("UwU'ifying In Progress...");
        progress_bar.enable_steady_tick(30);

        progress_bar
    }};
}

macro_rules! random_float {
    ($seeder:expr) => {
        rand::Rng::gen_range($seeder, 0.0..1.0)
    };
}

macro_rules! random_int {
    ($seeder:expr, $range:expr) => {
        rand::Rng::gen_range($seeder, $range)
    };
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
    random: Xoshiro256Plus,
    ascii: bool,
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
            random: Xoshiro256Plus::seed_from_u64(69420),
            is_runtime: false,
            ascii: false,
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
        ascii: bool,
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
            ascii,
            is_runtime,
            linkify,
            ..Default::default()
        };

        if random {
            uwuify.random = Xoshiro256Plus::from_entropy();
        }

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

                let uwu_progress_bar = progress_bar!();
                self.uwuify_sentence(self.text, &mut BufWriter::new(File::create(&self.output)?))?;
                uwu_progress_bar.finish_with_message("UwU'ifying Complete!");
            } else {
                #[cfg(not(test))]
                self.uwuify_sentence(self.text, &mut BufWriter::new(std::io::stdout().lock()))?;
                #[cfg(test)]
                self.uwuify_sentence(self.text, &mut std::io::sink())?;
            }
        } else {
            // Handle File I/O
            if Path::new(&self.output).exists() {
                return Err(Error::new(
                    ErrorKind::AlreadyExists,
                    format!("File '{}' already exists, aborting operation", &self.output),
                ));
            }

            let uwu_progress_bar = progress_bar!();
            self.uwuify_sentence(
                unsafe { from_utf8_unchecked(Mmap::map(&File::open(&self.input)?)?.as_ref()) },
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
                    let random_value = random_float!(&mut self.random);

                    if !self.is_runtime {
                        if random_value <= self.faces {
                            out.write_all(FACES[random_int!(&mut self.random, 0..FACES_SIZE)])?;
                            out.write_all(b" ")?;
                        } else if random_value <= self.actions {
                            out.write_all(ACTIONS[random_int!(&mut self.random, 0..ACTIONS_SIZE)])?;
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
                            if self.ascii {
                                out.write_all(ASCII[random_int!(&mut self.random, 0..ASCII_SIZE)])?;
                            } else {
                                out.write_all(FACES[random_int!(&mut self.random, 0..FACES_SIZE)])?;
                            }
                            out.write_all(b" ")?;
                        }
                        if random_value <= self.actions {
                            out.write_all(ACTIONS[random_int!(&mut self.random, 0..ACTIONS_SIZE)])?;
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
#[cfg(test)]
mod tests {
    #[cfg(feature = "bench")]
    extern crate test;

    #[cfg(feature = "bench")]
    #[bench]
    fn uwu_bench(b: &mut test::Bencher) {
        let mut uwuify = super::UwUify::new(
            Some(include_str!("test.txt")),
            None,
            None,
            None,
            None,
            None,
            None,
            false,
            false,
        );
        b.iter(|| uwuify.uwuify());
    }
}
