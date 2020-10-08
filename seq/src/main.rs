//! # seq - print a sequence of numbers
//!
//! A re-implementation of the `seq` command line tool from the **GNU coreutils** package. Currently
//! only a subset of the original options are supported.
//!
//! See <https://linux.die.net/man/1/seq> for further information about the original `seq` program.

use std::fmt::Write as FmtWrite;
use std::io::Write as IoWrite;
use std::iter::Peekable;

use anyhow::{bail, Context, Result};

/// Structure to hold all supported options of the application.
struct Options {
    separator: String,
}

impl Options {
    /// Create a new instance with default values.
    fn new() -> Self {
        Self {
            separator: "\n".to_owned(),
        }
    }
}

fn main() -> Result<()> {
    let mut args = std::env::args().skip(1).peekable();

    if args.peek().is_none() {
        print_help();
        return Ok(());
    }

    let options = match parse_options(&mut args)? {
        Some(options) => options,
        None => return Ok(()),
    };

    let (first, increment, last) = parse_arguments(&mut args)?;

    let mut current = first;
    let mut buf = String::new();

    while current <= last {
        if !buf.is_empty() {
            buf.write_str(&options.separator).unwrap();
        }

        write!(buf, "{}", current).unwrap();
        current += increment;
    }

    std::io::stdout()
        .write_all(&buf.as_bytes())
        .map_err(Into::into)
}

/// Check wheather an argument is identified as option, that is it starts with a dash (`-`) or
/// double dash (`--`) and the following character is a letter.
fn is_option(arg: Option<&String>) -> bool {
    let mut chars = arg.map(|v| v.as_str()).unwrap_or_default().chars();

    match (chars.next(), chars.next(), chars.next()) {
        (Some('-'), Some('-'), Some(third)) => third.is_alphabetic(),
        (Some('-'), Some(second), _) => second.is_alphabetic(),
        _ => false,
    }
}

/// Print the help message to stdout.
fn print_help() {
    println!(
        "\
{name} - {}

Usage: {name} [OPTION]... LAST
  or:  {name} [OPTION]... FIRST LAST
  or:  {name} [OPTION]... FIRST INCREMENT LAST

Print numbers from FIRST to LAST, in steps of INCREMENT.

Mandatory arguments to long options are mandatory for short options too.
  -s, --separator STRING   use STRING to separate numbers (default: \\n)
  -h, --help               display this help and exit
  -V, --version            output version information and exit

If FIRST or INCREMENT is omitted, it defaults to 1. That is, an
omitted INCREMENT defaults to 1 even when LAST is smaller than FIRST.
The sequence of numbers ends when the sum of the current number and
INCREMENT would become greater than LAST.
FIRST, INCREMENT, and LAST are interpreted as floating point values.
INCREMENT is usually positive if FIRST is smaller than LAST, and
INCREMENT is usually negative if FIRST is greater than LAST.
INCREMENT must not be 0; none of FIRST, INCREMENT and LAST may be NaN.\
        ",
        env!("CARGO_PKG_DESCRIPTION"),
        name = env!("CARGO_BIN_NAME"),
    );
}

/// Print version information to stdout.
fn print_version() {
    println!("{} {}", env!("CARGO_BIN_NAME"), env!("CARGO_PKG_VERSION"));
}

/// Parse all possible options that are supported. If `Ok(None)` is returned, the program should
/// exit as that means either the help message or version information was printed.
///
/// Possible options are:
/// - `-s / --separator` to pick another separator than `\n`.
/// - `-h / --help` to show the help message.
/// - `-V / --version` to show version information.
fn parse_options(args: &mut Peekable<impl Iterator<Item = String>>) -> Result<Option<Options>> {
    let mut options = Options::new();

    while is_option(args.peek()) {
        match args.next().unwrap().as_str() {
            "-s" | "--separator" => {
                options.separator = args
                    .next()
                    .context("expected STRING value for separator option")?
            }
            "-h" | "--help" => {
                print_help();
                return Ok(None);
            }
            "-V" | "--version" => {
                print_version();
                return Ok(None);
            }
            s => bail!("unknown option '{}'", s),
        }
    }

    Ok(Some(options))
}

/// Parse all possible combinations of positional arguments.
///
/// The possible combinations are:
/// - `LAST` to count from 1 to LAST with increments of 1.
/// - `FIRST LAST` to count from FIRST to LAST with increments of 1.
/// - `FIRST INCREMENT LAST` to count from FIRST to LAST with increments of INCREMENT.
fn parse_arguments(args: &mut impl Iterator<Item = String>) -> Result<(f64, f64, f64)> {
    Ok(match (args.next(), args.next(), args.next()) {
        (Some(last), None, None) => (1.0, 1.0, last.parse()?),
        (Some(first), Some(last), None) => (first.parse()?, 1.0, last.parse()?),
        (Some(first), Some(increment), Some(last)) => {
            (first.parse()?, increment.parse()?, last.parse()?)
        }
        _ => bail!("too many arguments"),
    })
}
