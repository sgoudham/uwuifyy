use std::panic::set_hook;

use clap::{Arg, ArgGroup, ErrorKind};

use uwuifyy::UwUify;

macro_rules! app {
    () => {
        clap::App::new(env!("CARGO_PKG_NAME"))
            .author(env!("CARGO_PKG_AUTHORS"))
            .version(env!("CARGO_PKG_VERSION"))
            .about(env!("CARGO_PKG_DESCRIPTION"))
            .long_about(None)
            .group(
                ArgGroup::new("uwu")
                    .required(true)
                    .args(&["text", "infile"]),
            )
            .arg(
                Arg::new("text")
                    .help("Text to uwu'ify")
                    .short('t')
                    .long("text")
                    .value_name("TEXT")
                    .required_unless_present_all(["infile", "outfile"])
                    .display_order(1),
            )
            .arg(
                Arg::new("infile")
                    .help("The file to uwu'ify")
                    .short('i')
                    .long("infile")
                    .conflicts_with("text")
                    .requires("outfile")
                    .value_name("FILE")
                    .value_hint(clap::ValueHint::FilePath)
                    .display_order(2),
            )
            .arg(
                Arg::new("outfile")
                    .help("The file to output uwu'ified text")
                    .short('o')
                    .long("outfile")
                    .value_name("FILE")
                    .value_hint(clap::ValueHint::FilePath)
                    .display_order(3),
            )
            .arg(
                Arg::new("ascii-only")
                    .help("The output file will only include ASCII faces")
                    .long("ascii-only")
                    .conflicts_with("unicode-only")
                    .display_order(4),
            )
            .arg(
                Arg::new("unicode-only")
                    .help("The output file will only include UTF-8 faces")
                    .long("unicode-only")
                    .display_order(5),
            )
            .arg(
                Arg::new("random")
                    .help("Flag to enable/disable random uwu'ifying")
                    .short('r')
                    .long("random")
                    .display_order(6),
            )
            .arg(
                Arg::new("words")
                    .help("The modifier to determine how many words to be uwu'ified")
                    .short('w')
                    .long("words")
                    .value_name("VALUE")
                    .default_value("1")
                    .validator(is_between_zero_and_one)
                    .display_order(7),
            )
            .arg(
                Arg::new("faces")
                    .help("The modifier for uwu faces e.g hello -> hewwo")
                    .short('f')
                    .long("faces")
                    .value_name("VALUE")
                    .default_value("0.05")
                    .validator(is_between_zero_and_one)
                    .display_order(8),
            )
            .arg(
                Arg::new("actions")
                    .help("The modifier for actions e.g *shuffles over*")
                    .short('a')
                    .long("actions")
                    .value_name("VALUE")
                    .default_value("0.125")
                    .validator(is_between_zero_and_one)
                    .display_order(9),
            )
            .arg(
                Arg::new("stutters")
                    .help("The modifier for stutters e.g b-baka!")
                    .short('s')
                    .long("stutters")
                    .value_name("VALUE")
                    .default_value("0.225")
                    .validator(is_between_zero_and_one)
                    .display_order(10),
            )
    };
}

macro_rules! clap_panic {
    ($e:expr) => {
        app!().error(ErrorKind::DisplayHelp, $e).exit()
    };
}

macro_rules! is_runtime {
    ($faces:expr, $actions:expr, $stutters:expr) => {
        $faces > 0 || $actions > 0 || $stutters > 0
    };
}

fn main() {
    set_hook(Box::new(|info| clap_panic!(info)));

    let matches = app!().get_matches();
    if let Err(err) = UwUify::new(
        matches.value_of("text"),
        matches.value_of("infile"),
        matches.value_of("outfile"),
        matches.is_present("ascii-only"),
        matches.is_present("unicode-only"),
        matches.is_present("random"),
        matches.value_of("words"),
        matches.value_of("faces"),
        matches.value_of("actions"),
        matches.value_of("stutters"),
        is_runtime!(
            matches.occurrences_of("faces"),
            matches.occurrences_of("actions"),
            matches.occurrences_of("stutters")
        ),
    )
    .uwuify()
    {
        clap_panic!(err);
    }
}

fn is_between_zero_and_one(input: &str) -> Result<(), &'static str> {
    let value = match input.parse::<f32>() {
        Ok(value) => value,
        Err(_) => return Err("The value must be a decimal number"),
    };

    if (0.0..=1.0).contains(&value) {
        return Ok(());
    }
    Err("The value must be between 0.0 and 1.0")
}