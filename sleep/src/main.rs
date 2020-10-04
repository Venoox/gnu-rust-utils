use std::env;
use std::error::Error;
use std::process::exit;
use std::thread::sleep;
use std::time::Duration;

const DAY_MILLIS: f64 = 86400000.0;
const HOUR_MILLIS: f64 = 3600000.0;
const MINUTE_MILLIS: f64 = 60000.0;
const SECOND_MILLIS: f64 = 1000.0;

type ParseResult<T> = Result<T, Box<dyn Error>>;

/// parse one token from the user as a duration
fn parse_time(dur: String) -> ParseResult<f64> {
    let mut vec_multiplier: Vec<char> = vec![];
    let mut duration: ParseResult<f64> = Ok(SECOND_MILLIS);
    // split into float characters (multiplier) and duration (minute, second, hour)
    for c in dur.chars() {
        if c.is_numeric() || c == '.' {
            vec_multiplier.push(c)
        } else {
            duration = match c {
                's' => Ok(SECOND_MILLIS),
                'm' => Ok(MINUTE_MILLIS),
                'h' => Ok(HOUR_MILLIS),
                'd' => Ok(DAY_MILLIS),
                _ => Err(format!("invalid time interval '{}'", dur).into()),
            };
            break; // break out of loop once we've found a non numeric character for this token
        }
    }
    let multiplier: f64 = vec_multiplier
        .iter()
        .collect::<String>()
        .parse()?;
    match duration {
        Ok(dur) => Ok(dur * multiplier),
        Err(e) => Err(e),
    }
}

/// get all arguments from user
/// if arguments can be parsed as time durations,
/// return the sum of all parsed durations as milliseconds
fn parse_args() -> ParseResult<u64> {
    let durations: Result<Vec<f64>, _> = env::args().skip(1).map(parse_time).collect();
    match durations {
        Ok(durs) => {
            if durs.is_empty() {
                Err("missing operand".into())
            } else {
                Ok(durs.iter().sum::<f64>() as u64)
            }
        },
        Err(e) => Err(e),
    }
}

fn main() {
    match parse_args() {
        Ok(sleep_duration_millis) => sleep(Duration::from_millis(sleep_duration_millis)),
        Err(e) => {
            eprintln!("sleep error: {}", e);
            exit(1)
        }
    };
}
