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

fn run<T: TimeZone>(args: Args, now: DateTime<T>)
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
            println!("{:x}", now.timestamp());
        } else {
            println!("{}", now.timestamp());
        }
    } else if args.local {
        println!("{}", now.to_rfc3339());
    } else {
        println!("{:?}", now);
    }
}

fn main() {
    let args = Args::parse_args_default_or_exit();

    if args.local {
        run(args, Local::now());
    } else {
        run(args, Utc::now());
    }
}
