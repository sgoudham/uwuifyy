#![cfg_attr(all(feature = "bench", test), feature(test))]

use std::fs::File;
use std::io::{BufWriter, Error, stdin, stdout, Write};
use std::path::Path;
use std::str::from_utf8_unchecked;

use ahash::RandomState;
use linkify::{LinkFinder, LinkKind};
use memmap::Mmap;

use constants::{
    ACTIONS, ACTIONS_SIZE, ASCII_FACES, ASCII_FACES_SIZE, MIXED_FACES, MIXED_FACES_SIZE,
    UNICODE_FACES, UNICODE_FACES_SIZE,
};

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

macro_rules! new_seeder {
    ($word:expr,$seeder:expr) => {
        <rand_xoshiro::Xoshiro256Plus as rand::SeedableRng>::seed_from_u64(
            <[u8] as ahash::CallHasher>::get_hash($word, $seeder),
        )
    };
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

macro_rules! write {
    ($out:expr, $bytes:expr) => {
        $out.write_all($bytes)
    };
}

#[derive(Debug)]
pub struct UwUify<'a> {
    text: &'a str,
    infile: &'a str,
    outfile: &'a str,
    ascii_only: bool,
    unicode_only: bool,
    random: RandomState,
    words: f64,
    faces: f64,
    actions: f64,
    stutters: f64,
    supplied_at_runtime: bool,
    linkify: LinkFinder,
}

impl<'a> Default for UwUify<'a> {
    fn default() -> Self {
        Self {
            text: "",
            infile: "",
            outfile: "",
            ascii_only: false,
            unicode_only: false,
            random: RandomState::with_seeds(69, 420, 96, 84),
            words: 1.0,
            faces: 0.05,
            actions: 0.125,
            stutters: 0.225,
            supplied_at_runtime: false,
            linkify: LinkFinder::new(),
        }
    }
}

impl<'a> UwUify<'a> {
    pub fn new(
        text: Option<&'a str>,
        infile: Option<&'a str>,
        outfile: Option<&'a str>,
        ascii_only: bool,
        unicode_only: bool,
        random: bool,
        words: Option<&'a str>,
        faces: Option<&'a str>,
        actions: Option<&'a str>,
        stutters: Option<&'a str>,
        supplied_at_runtime: bool,
    ) -> UwUify<'a> {
        let mut linkify = LinkFinder::new();
        linkify.kinds(&[LinkKind::Email, LinkKind::Url]);
        linkify.url_must_have_scheme(false);

        let mut uwuify = UwUify {
            text: text.unwrap_or_default(),
            infile: infile.unwrap_or_default(),
            outfile: outfile.unwrap_or_default(),
            ascii_only,
            unicode_only,
            supplied_at_runtime,
            linkify,
            ..Default::default()
        };

        if random {
            uwuify.random = RandomState::new();
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

    pub fn uwuify(&self) -> Result<(), Error> {
        // Handle Text
        if !self.text.is_empty() {
            // Handle Text Output
            if !self.outfile.is_empty() {
                if Path::new(&self.outfile).exists() {
                    let mut overwrite_file = String::new();
                    print!(
                        "File '{}' already exists, would you like to overwrite this file? [Y/n] ",
                        self.outfile
                    );

                    stdout().flush()?;
                    stdin().read_line(&mut overwrite_file)?;

                    if overwrite_file.to_lowercase().trim() != "y" {
                        println!("Exiting...");
                        return Ok(());
                    }
                }

                let uwu_progress_bar = progress_bar!();
                self.uwuify_sentence(self.text, &mut BufWriter::new(File::create(&self.outfile)?))?;
                uwu_progress_bar.finish_with_message("UwU'ifying Complete!");
            } else {
                #[cfg(not(test))]
                self.uwuify_sentence(self.text, &mut BufWriter::new(std::io::stdout().lock()))?;
                #[cfg(test)]
                self.uwuify_sentence(self.text, &mut std::io::sink())?;
            }
        } else {
            // Handle File I/O
            if Path::new(&self.outfile).exists() {
                let mut overwrite_file = String::new();
                print!(
                    "File '{}' already exists, would you like to overwrite this file? [Y/n] ",
                    self.outfile
                );

                stdout().flush()?;
                stdin().read_line(&mut overwrite_file)?;

                if overwrite_file.to_lowercase().trim() != "y" {
                    println!("Exiting...");
                    return Ok(());
                }
            }

            let uwu_progress_bar = progress_bar!();
            self.uwuify_sentence(
                unsafe { from_utf8_unchecked(Mmap::map(&File::open(&self.infile)?)?.as_ref()) },
                &mut BufWriter::new(File::create(&self.outfile)?),
            )?;
            uwu_progress_bar.finish_with_message("UwU'ifying Complete!");
        }

        Ok(())
    }

    pub fn uwuify_sentence<T: Write>(&self, text: &str, out: &mut T) -> Result<(), Error> {
        text.lines().try_for_each(|line| {
            line.split_whitespace()
                .map(|word_str| word_str.as_bytes())
                .try_for_each(|word| {
                    let mut seeder = new_seeder!(word, &self.random);
                    let random_value = random_float!(&mut seeder);

                    if !self.supplied_at_runtime {
                        if random_value <= self.faces {
                            if self.ascii_only {
                                write!(
                                    out,
                                    ASCII_FACES[random_int!(&mut seeder, 0..ASCII_FACES_SIZE)]
                                )?;
                            } else if self.unicode_only {
                                write!(
                                    out,
                                    UNICODE_FACES[random_int!(&mut seeder, 0..UNICODE_FACES_SIZE)]
                                )?;
                            } else {
                                write!(
                                    out,
                                    MIXED_FACES[random_int!(&mut seeder, 0..MIXED_FACES_SIZE)]
                                )?;
                            }
                        } else if random_value <= self.actions {
                            write!(out, ACTIONS[random_int!(&mut seeder, 0..ACTIONS_SIZE)])?;
                        } else if random_value <= self.stutters {
                            match word[0] {
                                b'L' | b'R' => write!(out, b"W"),
                                b'l' | b'r' => write!(out, b"w"),
                                byte => write!(out, &[byte]),
                            }?;
                            write!(out, b"-")?;
                        }
                    } else {
                        if random_value <= self.faces {
                            if self.ascii_only {
                                write!(
                                    out,
                                    ASCII_FACES[random_int!(&mut seeder, 0..ASCII_FACES_SIZE)]
                                )?;
                            } else if self.unicode_only {
                                write!(
                                    out,
                                    UNICODE_FACES[random_int!(&mut seeder, 0..UNICODE_FACES_SIZE)]
                                )?;
                            } else {
                                write!(
                                    out,
                                    MIXED_FACES[random_int!(&mut seeder, 0..MIXED_FACES_SIZE)]
                                )?;
                            }
                        }
                        if random_value <= self.actions {
                            write!(out, ACTIONS[random_int!(&mut seeder, 0..ACTIONS_SIZE)])?;
                        }
                        if random_value <= self.stutters {
                            match word[0] {
                                b'L' | b'R' => write!(out, b"W"),
                                b'l' | b'r' => write!(out, b"w"),
                                byte => write!(out, &[byte]),
                            }?;
                            write!(out, b"-")?;
                        }
                    }

                    if self
                        .linkify
                        .links(unsafe { from_utf8_unchecked(word) })
                        .count()
                        > 0
                        || random_value > self.words
                    {
                        write!(out, word)?;
                    } else {
                        (0..word.len()).try_for_each(|index| match word[index] {
                            b'L' | b'R' => write!(out, b"W"),
                            b'l' | b'r' => write!(out, b"w"),
                            b'A' | b'E' | b'I' | b'O' | b'U' | b'a' | b'e' | b'i' | b'o' | b'u' => {
                                match word.get(index - 1).unwrap_or(&word[0]) {
                                    b'N' | b'n' => write!(out, &[b'y', word[index]]),
                                    _ => write!(out, &[word[index]]),
                                }
                            }
                            byte => write!(out, &[byte]),
                        })?;
                    }
                    write!(out, b" ")
                })?;
            write!(out, b"\n")
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
        let uwuify = super::UwUify::new(
            Some(include_str!("test.txt")),
            None,
            None,
            false,
            true,
            false,
            None,
            None,
            None,
            None,
            false,
        );
        b.iter(|| uwuify.uwuify());
    }
}