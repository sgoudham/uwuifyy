use clap::{Arg, ArgGroup, ErrorKind};

use uwuify::UwUify;

macro_rules! modifiers_supplied_at_runtime {
    ($faces_occurrences:expr,$actions_occurrences:expr,$stutters_occurrences:expr) => {
        $faces_occurrences > 0 || $actions_occurrences > 0 || $stutters_occurrences > 0
    };
}

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
                Arg::new("words")
                    .help("The modifier to determine how many words to be uwu'ified")
                    .short('w')
                    .long("words")
                    .value_name("VALUE")
                    .default_value("1")
                    .validator(is_between_zero_and_one)
                    .display_order(4),
            )
            .arg(
                Arg::new("faces")
                    .help("The modifier for uwu faces e.g hello -> hewwo")
                    .short('f')
                    .long("faces")
                    .value_name("VALUE")
                    .default_value("0.05")
                    .validator(is_between_zero_and_one)
                    .display_order(5),
            )
            .arg(
                Arg::new("actions")
                    .help("The modifier for actions e.g *shuffles over*")
                    .short('a')
                    .long("actions")
                    .value_name("VALUE")
                    .default_value("0.125")
                    .validator(is_between_zero_and_one)
                    .display_order(6),
            )
            .arg(
                Arg::new("stutters")
                    .help("The modifier for stutters e.g b-baka!")
                    .short('s')
                    .long("stutters")
                    .value_name("VALUE")
                    .default_value("0.225")
                    .validator(is_between_zero_and_one)
                    .display_order(7),
            )
            .arg(
                Arg::new("random")
                    .help("Flag to enable/disable random uwu'ifying")
                    .short('r')
                    .long("random")
                    .display_order(8),
            )
    };
}

fn main() {
    let matches = app!().get_matches();

    match UwUify::new(
        matches.value_of("text"),
        matches.value_of("infile").map(|f| std::path::Path::new(f)),
        matches.value_of("outfile"),
        modifiers_supplied_at_runtime!(
            matches.occurrences_of("faces"),
            matches.occurrences_of("actions"),
            matches.occurrences_of("stutters")
        ),
        matches.value_of_t_or_exit("words"),
        matches.value_of_t_or_exit("faces"),
        matches.value_of_t_or_exit("actions"),
        matches.value_of_t_or_exit("stutters"),
        matches.is_present("random"),
    )
    .uwuify()
    {
        Ok(_) => (),
        Err(err) => {
            app!().error(ErrorKind::DisplayHelp, err).exit();
        }
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
