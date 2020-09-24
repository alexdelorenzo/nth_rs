use std::io::{
  self, 
  Read, 
  BufRead, 
  Result as IoResult,
  Write
};
use std::collections::HashSet;
use std::iter::Iterator;
use std::str::FromStr;

use byte_string::ByteString;
use clap::{ArgMatches, clap_app};


const LOWEST: usize = 0;
const LINES: &str = "LINES";
const REVERSE: &str = "reverse";

const NEW_LINE: u8 = b'\n';
const CARRIAGE_RETURN: u8 = b'\r';

type LineNums = Vec<usize>;
type ByteLine = ByteString;
type ByteLineResult = IoResult<ByteLine>;
type NthStr = (usize, ByteLine);

#[derive(Debug, Copy, Clone)]
pub struct ByteLines<B> {
    buf: B,
}

impl<B: BufRead> Iterator for ByteLines<B> {
  type Item = ByteLineResult;

  fn next(&mut self) -> Option<ByteLineResult> {
    let mut buf = vec![];
    let mut bytes = self.buf.by_ref().bytes();

    while let Some(Ok(byte)) = bytes.next() {
      buf.push(byte);

      if is_newline(byte) {
        break;
      }
    }

    let byte_str = ByteString::new(buf);
    Some(Ok(byte_str))
  }
}

trait ReadByteLines<T> {
  fn byte_lines(self: Self) -> ByteLines<T>;
}

impl<T> ReadByteLines<T> for T {
  fn byte_lines(self: T) -> ByteLines<T> {
    ByteLines { buf: self }
  }
}

fn to_num(num: &str) -> usize {
  FromStr::from_str(num).unwrap()
}

fn is_newline(chr: u8) -> bool {
  chr == NEW_LINE || chr == CARRIAGE_RETURN
}

fn to_stdout(byte_str: &ByteString) {
  let bytes = &byte_str.0;

  io::stdout()
    .write(bytes)
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
    (version: "0.2.0")
    (author: "AlexDeLorenzo.dev")
    (about: 
      "Return the contents of stdin from the line numbers supplied as arguments.")
    (@arg LINES: +required ... "Line numbers to select")
    (@arg reverse: -r --reverse 
      "Write every line, except the line numbers supplied as LINES, from stdin to stdout.")
  )
  .get_matches()
}
