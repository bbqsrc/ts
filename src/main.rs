use std::io::Read;

use atty::Stream;
use chrono::prelude::*;
use gumdrop::Options;

#[derive(Options)]
#[doc = "\
Prints timestamps in various formats. Default output is ISO 8601 UTC date and
time format. <https://github.com/bbqsrc/ts>\
"]
struct Args {
    #[options(help = "print help message")]
    help: bool,

    #[options(help = "timestamp in local time")]
    local: bool,

    #[options(help = "print Unix epoch timestamp")]
    unix_epoch: bool,

    #[options(help = "print Unix epoch in hex")]
    hex_unix_epoch: bool,
}

fn run<T: TimeZone>(args: &Args, now: DateTime<T>)
where
    T::Offset: std::fmt::Display,
{
    let is_unix = args.unix_epoch;
    let is_hex = args.hex_unix_epoch;

    if is_unix || is_hex {
        if args.local {
            eprintln!("warning: local flag ignored.");
        }
        if is_hex {
            print!("{:x}", now.timestamp());
        } else {
            print!("{}", now.timestamp());
        }
    } else if args.local {
        print!("{}", now.to_rfc3339());
    } else {
        print!("{:?}", now);
    }
}

fn main() {
    let args = Args::parse_args_default_or_exit();

    if atty::is(Stream::Stdin) {
        if args.local {
            run(&args, Local::now());
        } else {
            run(&args, Utc::now());
        }
        println!();
    } else {
        let mut buf = String::new();
        let stdin = std::io::stdin();
        while let Ok(n) = stdin.read_line(&mut buf) {
            if n == 0 {
                break;
            }

            if args.local {
                run(&args, Local::now());
            } else {
                run(&args, Utc::now());
            }
            print!(": {}", buf);
            buf.clear();
        }
    }
}
