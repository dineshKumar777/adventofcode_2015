extern crate clap;

use clap::{App, Arg};

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

fn main() {
    let matches = App::new("Advent of code")
        .author("Dinesh Kumar")
        .version("0.1.0")
        .about("Advent of code solutions in Rust")
        .arg(
            Arg::with_name("day")
                .required(true)
                .index(1)
                .help("specified which days challenge to run")
                .validator(|str| {
                    str.parse::<u32>()
                        .or(Err("day must be an integer").to_owned())
                        .and_then(|v| match v {
                            0..=25 => Ok(()),
                            _ => Err("day must be between 1 and 25"),
                        })
                }),
        )
        .get_matches();

    let day = matches.value_of("day").unwrap().parse::<u32>().unwrap();
    let now = std::time::Instant::now();
    match day {
        1 => day1::solve(),
        2 => day2::solve(),
        3 => day3::solve(),
        4 => day4::solve(),
        5 => day5::solve(),
        _ => println!("oops! Day {} isnt implemented yet!", day),
    }
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
