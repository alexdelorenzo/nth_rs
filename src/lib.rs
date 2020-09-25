use std::io::{
  self, 
  Write
};
use std::collections::HashSet;
use std::iter::Iterator;
use std::str::FromStr;

use clap::{ArgMatches, clap_app};

// Check out https://github.com/alexdelorenzo/byte_lines
use byte_lines::{
  ByteLine,
  ReadByteLines,
};

const LOWEST: usize = 0;
const LINES: &str = "LINES";
const REVERSE: &str = "reverse";

type LineNums = Vec<usize>;
type NthStr = (usize, ByteLine);

fn to_num(num: &str) -> usize {
  FromStr::from_str(num).unwrap()
}

fn to_stdout(byte_line: &ByteLine) {
  let bytes = &byte_line.0;

  io::stdout()
    .write_all(bytes)
    .expect("Could not write bytes to stdout.");
}

fn get_line_nums(matches: &ArgMatches) -> LineNums {
  let mut line_nums =
    if let Some(lines) = matches.values_of(LINES) {
      lines.map(to_num).collect() 
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
      to_stdout(&line);
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
  let mut lines: HashSet<&usize> = 
    lines.iter().collect();

  for (nth, line) in content {
    if lines.contains(&nth) {
      lines.remove(&nth);
    } else {
      to_stdout(&line);
    }
  }
}

pub fn run() {
  let matches = get_matches_from_cli();
  let mut line_nums = get_line_nums(&matches);

  let stdin = io::stdin();
  let nth_lines = stdin
    .lock()
    .byte_lines()
    .map(|line| line.unwrap())
    .enumerate();

  if matches.is_present(REVERSE) {
    exclude_lines(&line_nums, nth_lines);
  } else {
    include_lines(&mut line_nums, nth_lines);
  }
}

fn get_matches_from_cli<'a>() -> ArgMatches<'a> {
  clap_app!(nth =>
    (version: "0.2.2")
    (author: "AlexDeLorenzo.dev")
    (about: 
      "Return the contents of stdin from the line numbers supplied as arguments.")
    (@arg LINES: +required ... "Line numbers to select")
    (@arg reverse: -r --reverse 
      "Write every line, except the line numbers supplied as LINES, from stdin to stdout.")
  )
  .get_matches()
}
