use clap::{ArgGroup, ErrorKind, IntoApp, Parser};

use uwuify::UwUify;

macro_rules! modifiers_supplied_at_runtime {
    ($faces_occurrences:expr,$actions_occurrences:expr,$stutters_occurrences:expr) => {
        $faces_occurrences > 0 || $actions_occurrences > 0 || $stutters_occurrences > 0
    };
}

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(group(ArgGroup::new("uwu").required(true).args(& ["text", "infile"]),))]
struct Args {
    /// Text to uwu'ify
    #[clap(short, long, required_unless_present_all = ["infile", "outfile"], display_order = 1)]
    text: Option<String>,

    /// The file to uwu'ify
    #[clap(short, long, parse(from_os_str), conflicts_with = "text", requires = "outfile", value_name = "FILE", value_hint = clap::ValueHint::FilePath, display_order = 2)]
    infile: Option<std::path::PathBuf>,

    /// The file to output uwu'ified text
    #[clap(short, long, value_name = "FILE", value_hint = clap::ValueHint::FilePath, display_order = 3)]
    outfile: Option<String>,

    /// The modifier to determine how many words to be uwu'ified
    #[clap(short, long, value_name = "VALUE", default_value = "1", validator = is_between_zero_and_one, display_order = 4)]
    words: f64,

    /// The modifier for uwu faces e.g hello -> hewwo
    #[clap(short, long, value_name = "VALUE", default_value = "0.05", validator = is_between_zero_and_one, display_order = 5)]
    faces: f64,

    /// The modifier for actions e.g *shuffles over*
    #[clap(short, long, value_name = "VALUE", default_value = "0.125", validator = is_between_zero_and_one, display_order = 6)]
    actions: f64,

    /// The modifier for stutters e.g b-baka!
    #[clap(short, long, value_name = "VALUE", default_value = "0.225", validator = is_between_zero_and_one, display_order = 7)]
    stutters: f64,

    /// Flag to enable/disable random uwu'ifying
    #[clap(short, long, display_order = 8)]
    random: bool,
}

fn main() {
    let matches = Args::into_app().get_matches();

    let uwuify = UwUify::new(
        matches.value_of("text"),
        matches.value_of("infile").map(|f| std::path::Path::new(f)),
        matches.value_of("outfile"),
        modifiers_supplied_at_runtime!(
            matches.occurrences_of("faces"),
            matches.occurrences_of("actions"),
            matches.occurrences_of("stutters")
        ),
        matches.value_of_t("words").unwrap_or_else(|e| e.exit()),
        matches.value_of_t("faces").unwrap_or_else(|e| e.exit()),
        matches.value_of_t("actions").unwrap_or_else(|e| e.exit()),
        matches.value_of_t("stutters").unwrap_or_else(|e| e.exit()),
        matches.value_of_t("random").unwrap_or_else(|e| e.exit()),
    );
    match uwuify.uwuify() {
        Ok(_) => (),
        Err(err) => {
            let mut app = Args::into_app();
            app.error(ErrorKind::DisplayHelp, err).exit();
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
