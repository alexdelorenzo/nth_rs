use std::io::{self, BufRead};
use std::collections::HashSet;
use std::iter::{FromIterator, Iterator};
use std::str::FromStr;
// use rayon::prelude

use clap::ArgMatches;
use clap::clap_app;

const LOWEST: usize = 0;
const LINES: &str = "LINES";
const REVERSE: &str = "reverse";

type LineNums = Vec<usize>;
type NthStr = (usize, String);
// type NthStrIterable = Box<dyn Iterator<Item = NthStr>>;

fn get_matches_from_cli<'a>() -> ArgMatches<'a> {
    clap_app!(nth =>
        (version: "0.0.1")
        (author: "AlexDeLorenzo.dev")
        (about: 
            "Return the contents of stdin from the line numbers supplied as arguments.")
        (@arg LINES: +required ... "Line numbers to select")
        (@arg reverse: -r --reverse 
            "Write every line, except the line numbers supplied as LINES, from stdin to stdout.")
    )
    .get_matches()
}

pub fn run() {
    let matches = get_matches_from_cli();
    let mut line_nums = get_line_nums(&matches);

    let stdin = io::stdin();
    let nth_lines = stdin.lock()
        .lines()
        .map(|line| line.unwrap())
        .enumerate();

    if matches.is_present(REVERSE) {
        exclude_lines(&line_nums, nth_lines);
    } else {
        include_lines(&mut line_nums, nth_lines);
    }
}

fn get_line_nums(matches: &ArgMatches) -> LineNums {
    let mut line_nums = 
        if let Some(lines) = matches.values_of(LINES) {
            Vec::from_iter(lines.map(|line| 
                FromStr::from_str(line).unwrap()
            )) 
        } else {
            vec![]
        };

    line_nums.sort();
    line_nums
}

fn include_lines<T: IntoIterator<Item = NthStr>>(
    lines: &mut LineNums, 
    content: T
) {
    for (nth, line) in content {
        if nth == lines[LOWEST] {
            println!("{}", line);
            lines.remove(LOWEST);
        }

        if lines.is_empty() {
            break
        }
    } 
}

fn exclude_lines<T: Iterator<Item = NthStr>>(
    lines: &LineNums, 
    content: T
) {
    let mut lines: HashSet<usize> = 
      HashSet::from_iter(lines.iter().cloned());
    
    for (nth, line) in content {
        if lines.contains(&nth) {
            lines.remove(&nth);
        } else {
            println!("{}", line);
        }
    }
}
