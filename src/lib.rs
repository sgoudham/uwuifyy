#![cfg_attr(test, feature(test))]

use indicatif::{ProgressBar, ProgressStyle};
use linkify::{LinkFinder, LinkKind};
use std::fs::File;
use std::io::{BufWriter, Error, Write};
use std::path::Path;

use constants::ACTIONS;
use constants::ACTIONS_SIZE;
use constants::FACES;
use constants::FACES_SIZE;
use seeder::UwUSeeder;

mod constants;
mod seeder;

macro_rules! progress_bar {
    () => {{
        let progress_bar = ProgressBar::new_spinner();
        progress_bar.set_style(
            ProgressStyle::default_spinner().template("{spinner:.green} [{elapsed_precise}] {msg}"),
        );
        progress_bar.enable_steady_tick(100);

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
    ) -> UwUify<'a> {
        // I dislike this

        let mut linkify = LinkFinder::new();
        linkify.kinds(&[LinkKind::Email, LinkKind::Url]);
        linkify.url_must_have_scheme(false);
        let mut uwuify = UwUify {
            text: text.unwrap_or_default(),
            input: infile.unwrap_or_default(),
            output: outfile.unwrap_or_default(),
            random,
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

    pub fn uwuify(&self) -> Result<(), Error> {
        // Handle Text
        if !self.text.is_empty() {
            // Handle Text Output
            if !self.output.is_empty() {
                if Path::new(&self.output).exists() {
                    return Err(Error::new(
                        std::io::ErrorKind::AlreadyExists,
                        format!("File '{}' already exists, aborting operation", &self.output),
                    ));
                }

                let mut uwu_out_file = BufWriter::new(File::create(&self.output)?);
                let uwu_progress_bar = progress_bar!();
                self.uwuify_sentence(self.text, &mut uwu_out_file)?;

                uwu_progress_bar.finish_with_message("UwU'ifying Complete!");
                Ok(())
            } else {
                #[cfg(not(test))]
                let stdout = std::io::stdout();
                #[cfg(not(test))]
                let mut out = BufWriter::new(stdout.lock());
                #[cfg(test)]
                let mut out = std::io::sink();
                self.uwuify_sentence(self.text, &mut out)?;
                #[cfg(not(test))]
                out.write_all(b"\n")?;
                Ok(())
            }
        } else {
            // Handle File I/O
            if Path::new(&self.output).exists() {
                return Err(Error::new(
                    std::io::ErrorKind::AlreadyExists,
                    format!("File '{}' already exists, aborting operation", &self.output),
                ));
            }

            let uwu_progress_bar = progress_bar!();
            self.uwuify_sentence(
                unsafe {
                    std::str::from_utf8_unchecked(
                        memmap::Mmap::map(&File::open(&self.input)?)?.as_ref(),
                    )
                },
                &mut BufWriter::new(File::create(&self.output)?),
            )?;
            uwu_progress_bar.finish_with_message("UwU'ifying Complete!");
            Ok(())
        }
    }

    fn uwuify_sentence<T: Write>(&self, text: &str, out: &mut T) -> Result<(), std::io::Error> {
        text.as_bytes()
            .split(|w| matches!(*w, b'\t' | b'\x0C' | b'\r' | b' '))
            .scan([].as_ref(), |scan, i| {
                let ret = i != &[b' '];
                *scan = i;
                if ret {
                    Some(i)
                } else {
                    None
                }
            })
            .try_for_each(|word| {
                let mut seeder = UwUSeeder::new(word, self.random);
                let random_value = seeder.random();

                if random_value <= self.faces {
                    out.write_all(FACES[seeder.random_int(0..FACES_SIZE)])?;
                    out.write_all(b" ")?;
                } else if random_value <= self.actions {
                    out.write_all(ACTIONS[seeder.random_int(0..ACTIONS_SIZE)])?;
                    out.write_all(b" ")?;
                } else if random_value <= self.stutters {
                    (0..seeder.random_int(1..2)).into_iter().try_for_each(|_| {
                        out.write_all(&[word[0]])?;
                        out.write_all(b"-")
                    })?;
                }

                if self
                    .linkify
                    .links(unsafe { std::str::from_utf8_unchecked(word) })
                    .count()
                    > 0
                    || random_value > self.words
                {
                    out.write_all(word)?;
                } else {
                    (0..word.len()).try_for_each(|index| match word[index] {
                        b'L' | b'R' => out.write_all(b"W"),
                        b'l' | b'r' => out.write_all(b"w"),
                        b'E' | b'e' | b'A' | b'I' | b'O' | b'U' | b'a' | b'i' | b'o' | b'u' => {
                            match word.get(index - 1).unwrap_or(&word[0]) {
                                b'N' | b'n' => out.write_all(&[b'y', word[index]]),
                                _ => out.write_all(&[word[index]]),
                            }
                        }
                        _ => out.write_all(&[word[index]]),
                    })?;
                }
                out.write_all(b" ")
            })
    }
}

#[cfg(test)]
mod tests {
    extern crate test;
    use linkify::{LinkFinder, LinkKind};

    #[bench]
    fn uwu_bench(b: &mut test::Bencher) {
        let mut linkify = LinkFinder::new();
        linkify.kinds(&[LinkKind::Email, LinkKind::Url]);
        linkify.url_must_have_scheme(false);
        let uwuify = super::UwUify {
            text: include_str!("test.txt"),
            linkify,
            ..Default::default()
        };
        b.iter(|| uwuify.uwuify());
    }
}
