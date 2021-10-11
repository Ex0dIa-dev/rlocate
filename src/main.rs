use clap::{App, Arg};
use rlocate;

const VERSION: &str = env!("CARGO_PKG_VERSION");

const PATTERN: &str = "PATTERN";
const COUNT: &str = "count";
const IGNORE_CASE: &str = "ignore-case";
const NO_COLOR: &str = "no-color";

fn main() {
    let matches = App::new("rlocate")
        .version(VERSION)
        .author("Ex0dIa-dev")
        .about("mlocate like tool. Search for PATTERN in the filesystem")
        .arg(
            Arg::new(PATTERN)
                .required(true)
                .about("pattern to use to search entries"),
        )
        .arg(
            Arg::new(COUNT)
                .long("count")
                .short('c')
                .about("only print the number of found entries"),
        )
        .arg(
            Arg::new(IGNORE_CASE)
                .long("ignore-case")
                .short('i')
                .about("ignore case distinctions when matching PATTERN"),
        )
        .arg(
            Arg::new(NO_COLOR)
                .long("no-color")
                .short('n')
                .about("print the found entries without colored string"),
        )
        .get_matches();

    let cfg = rlocate::Config::new(
        matches.value_of(PATTERN).unwrap(),
        matches.is_present(COUNT),
        matches.is_present(IGNORE_CASE),
        matches.is_present(NO_COLOR),
    );

    rlocate::rlocate(cfg);
}
