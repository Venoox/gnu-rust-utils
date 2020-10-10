extern crate clap;
use clap::{App, Arg};

use std::str::FromStr;

#[derive(PartialEq)]
enum NumberingStyle {
    All,
    NonEmpty,
    None,
    #[allow(unused)]
    Regex(String),
}

impl FromStr for NumberingStyle {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "a" => Ok(Self::All),
            "t" => Ok(Self::NonEmpty),
            "n" => Ok(Self::None),
            p if p.starts_with('p') => Err(String::from("BRE search is not yet implemented.")),
            // when basic regular expressions get implemented, use Ok(Self::Regex(String::from(&p[1..])))
            _ => Err(format!(
                "Unknown STYLE \"{}\", must be one of a, t, n, pBRE.",
                s
            )),
        }
    }
}

fn style_validator(s: String) -> Result<(), String> {
    NumberingStyle::from_str(&s).map(|_| ())
}

fn main() -> std::io::Result<()> {
    const FORMAT: [&str; 3] = ["ln", "rn", "rz"];
    let matches = App::new("nl")
        .version("v0.1.0")
        .author("Johann150")
        .about("line numbering filter")
        .setting(clap::AppSettings::UnifiedHelpMessage)
        .arg(
            Arg::with_name("body-num")
                .short("b")
                .long("body-numbering")
                .takes_value(true)
                .value_name("STYLE")
                .validator(style_validator)
                .default_value("t")
                .help("use STYLE for numbering body lines"),
        )
        .arg(
            Arg::with_name("delim")
                .short("d")
                .long("section-delimiter")
                .takes_value(true)
                .value_name("C[C]")
                .validator(|s| {
                    if s.chars().count() <= 2 {
                        Ok(())
                    } else {
                        Err(String::from("At most 2 characters."))
                    }
                })
                .default_value(r"\:")
                .help("use CC for logical page delimiters. If the second character is not specified, it is ':' by default"),
        )
        .arg(
            Arg::with_name("foot-num")
                .short("f")
                .long("footer-numbering")
                .takes_value(true)
                .value_name("STYLE")
                .validator(style_validator)
                .default_value("n")
                .help("use STYLE for numbering footer lines"),
        )
        .arg(
            Arg::with_name("head-num")
                .short("h")
                .long("header-numbering")
                .takes_value(true)
                .value_name("STYLE")
                .validator(style_validator)
                .default_value("n")
                .help("use STYLE for numbering header lines"),
        )
        .arg(
            Arg::with_name("incr")
                .short("i")
                .long("line-increment")
                .takes_value(true)
                .value_name("NUMBER")
                .validator(|s| {
                    s.parse::<usize>()
                        .map(|_| ())
                        .map_err(|_| String::from("Must be a number."))
                })
                .default_value("1")
                .help("line increment at each line"),
        )
        .arg(
            Arg::with_name("join")
                .short("l")
                .long("join-blank-lines")
                .takes_value(true)
                .value_name("NUMBER")
                .validator(|s| {
                    s.parse::<usize>()
                        .map(|_| ())
                        .map_err(|_| String::from("Must be a number."))
                })
                .default_value("1")
                .help("group of NUMBER empty lines counted as one"),
        )
        .arg(
            Arg::with_name("fmt")
                .short("n")
                .long("number-format")
                .takes_value(true)
                .value_name("FORMAT")
                .possible_values(&FORMAT)
                .default_value("rn")
                .help("insert line numbers according to FORMAT"),
        )
        .arg(
            Arg::with_name("no-renum")
                .short("p")
                .long("no-renumber")
                .help("do not reset line numbers for each section"),
        )
        .arg(
            Arg::with_name("sep")
                .short("s")
                .long("number-separator")
                .takes_value(true)
                .value_name("STRING")
                .default_value("\t")
                .help("add STRING after a (possible) line number"),
        )
        .arg(
            Arg::with_name("start")
                .short("v")
                .long("starting-line-number")
                .takes_value(true)
                .value_name("NUMBER")
                .validator(|s| {
                    s.parse::<usize>()
                        .map(|_| ())
                        .map_err(|_| String::from("Must be a number."))
                })
                .default_value("1")
                .help("first line number for each section"),
        )
        .arg(
            Arg::with_name("width")
                .short("w")
                .long("number-width")
                .takes_value(true)
                .value_name("NUMBER")
                .validator(|s| {
                    s.parse::<usize>()
                        .map(|_| ())
                        .map_err(|_| String::from("Must be a number."))
                })
                .default_value("6")
                .help("use NUMBER columns for line numbers"),
        )
        .arg(
            Arg::with_name("FILE")
                .help("Input files. With no FILE or when FILE is -, read standard input.")
                .index(1)
                .multiple(true),
        )
        .after_help(
            "Default options are: -bt -d'\\:' -fn -hn -i1 -l1 -n'rn' -s<TAB> -v1 -w6

CC are two delimiter characters used to construct logical page delimiters;
a missing second character implies ':'.

STYLE is one of:
\ta\tnumber all lines
\tt\tnumber only nonempty lines
\tn\tnumber no lines
\tpBRE\tnumber only lines that contain a match for the basic regular expression, BRE

FORMAT is one of:
\tln\tleft justified, no leading zeroes
\trn\tright justified, no leading zeroes
\trz\tright justified, leading zeroes
",
        )
        .get_matches();

    let reset = !matches.is_present("no-renum");
    let start = matches.value_of("start").unwrap().parse::<usize>().unwrap();
    let width = matches.value_of("width").unwrap().parse::<usize>().unwrap();
    let incr = matches.value_of("incr").unwrap().parse::<usize>().unwrap();
    let empties_join = matches.value_of("join").unwrap().parse::<usize>().unwrap();
    let format = matches.value_of("fmt").unwrap();
    let styles: [NumberingStyle; 3] = [
        matches.value_of("head-num").unwrap().parse().unwrap(),
        matches.value_of("body-num").unwrap().parse().unwrap(),
        matches.value_of("foot-num").unwrap().parse().unwrap(),
    ];
    let separator = matches.value_of("sep").unwrap();
    let delimiter = {
        match matches.value_of("delim").unwrap() {
            // if only one character is supplied, the other is still the default
            s if s.chars().count() == 1 => format!("{}:", s),
            s => String::from(s),
        }
    };

    let mut line = String::new();
    let mut line_no = start;
    let mut empties = 0;

    // in body by default
    let mut style = &styles[1];

    let files = matches
        .values_of("FILE")
        .map_or(vec!["-"], |values| values.collect());
    let stdin = std::io::stdin();
    for path in files {
        let mut reader: Box<dyn std::io::BufRead> = if path == "-" {
            Box::new(stdin.lock())
        } else {
            Box::new(std::io::BufReader::new(std::fs::File::open(path)?))
        };

        while reader.read_line(&mut line)? > 0 {
            if line.ends_with('\n') {
                line.pop(); // remove '\n'
            }

            if line == delimiter {
                // start of footer
                if reset {
                    line_no = start
                }
                style = &styles[2];
                println!();
            } else if line == delimiter.repeat(2) {
                // start of body
                if reset {
                    line_no = start
                }
                style = &styles[1];
                println!();
            } else if line == delimiter.repeat(3) {
                // start of header
                if reset {
                    line_no = start
                }
                style = &styles[0];
                println!();
            } else {
                macro_rules! lineno {
                    (do_incr) => {{
                        // print and also increment line number
                        let res = match format {
                            "ln" => format!("{0:<1$}{2}", line_no, width, separator),
                            "rn" => format!("{0:>1$}{2}", line_no, width, separator),
                            "rz" => format!("{0:0>1$}{2}", line_no, width, separator),
                            _ => unreachable!(),
                        };
                        line_no += incr;
                        res
                    }};
                    (empty_line) => {
                        // don't increment line number
                        format!("{0:1$}{0:2$}", "", width, separator.len());
                    };
                }

                if style == &NumberingStyle::None {
                    print!("{}", lineno!(empty_line));
                } else if line.is_empty() {
                    empties += 1;
                    if empties == empties_join {
                        if style == &NumberingStyle::All {
                            print!("{}", lineno!(do_incr));
                        } else {
                            print!("{}", lineno!(empty_line));
                        }
                        empties = 0;
                    }
                } else {
                    empties = 0;
                    print!("{}", lineno!(do_incr));
                }
                println!("{}", line);
            }

            line.clear();
        }
    }

    Ok(())
}
