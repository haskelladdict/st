// Copyright 2015 Markus Dittrich
// Licensed under BSD license, see LICENSE file for details
//
// st is a simple commandline helper script for calculating basic
// statistics on a data file consisting of column oriented
// floating point numbers.
// NOTE: Currently stats will read in all the data to compute the statistics
// and thus require memory on the order of the data set size.

use std::env;
use std::f64;
use std::fs::File;
use std::io::{BufRead, BufReader};

mod stats;


fn main() {
  let args: Vec<_> = env::args().collect();
  if args.len() <= 1 {
    usage();
    return;
  }
  let file_name = &args[1];
  let file = File::open(file_name);
  if file.is_err() {
    usage();
    return;
  }

  let buf = BufReader::new(file.unwrap());
  let out = compute_statistics(buf);
  let mut count = 0;
  for o in out {
    println!("Column {}: \n", count);
    println!("#elems : {}", o.count);
    println!("min    : {}", o.min);
    println!("max    : {}", o.max);
    println!("mean   : {}", o.mean);
    println!("median : {}", o.median);
    println!("std    : {}", o.sd);
    println!("\n\n");
    count += 1;
  }
}


// compute_statistics computes basic statistics for the content of the
// provided BufReader
fn compute_statistics(mut buf: BufReader<File>) -> Vec<Output> {

  // read first line to determine number of data columns
  let mut out = Vec::new();
  let s = &mut String::new();
  let r = buf.read_line(s);
  if r.is_err () {
    return out;
  }

  let tokens : Vec<&str> = s.split(" ").collect();
  for i in 0..tokens.len() {
    out.push(Output::new());
    let n_r = tokens[i].trim().parse::<f64>();
    if n_r.is_err() {
      println!("Warning {} in line {} is not a number. Skipping", s, out[i].count);
      continue;
    }
    let n = n_r.unwrap();
    out[i].update(n);
  }

  for line_r in buf.lines() {
    let line = line_r.unwrap();
    let tokens : Vec<&str> = line.split(" ").collect();
    for i in 0..tokens.len() {
      let n_r = tokens[i].trim().parse::<f64>();
      if n_r.is_err() {
        println!("Warning {} in line {} is not a number. Skipping", line, out[i].count);
        continue;
      }
      let n = n_r.unwrap();
      out[i].update(n);
    }
  }
  for o in out.iter_mut() { //&mut out {
    o.finalize();
  }
  out
}

// usage prints a short message describing the usage of the function
fn usage() {
  println!("usage: stats <filename>");
}


// Output keeps track of the per column statistics
#[derive(Default)]
struct Output {
  count: i64,
  min: f64,
  max: f64,
  mean: f64,
  median: f64,
  sd: f64,
  qk: f64,
  mk: f64,
  med: stats::Median,
}

impl Output {

  fn new() -> Output {
    Output { count: 0, min: f64::MAX, max: -f64::MAX, mean: 0.0,
      median: 0.0, sd: 0.0, qk: 0.0, mk: 0.0, med: stats::Median::new()}
  }

  fn update(&mut self, v: f64) {
    self.count += 1;

    // update median
    self.med.update(stats::FloatVal::new(v));

    // update variance
    let k: f64 = self.count as f64;
    self.qk += (k - 1.0) * (v - self.mk) * (v - self.mk) / k;
    self.mk += (v - self.mk) / k;

    // update min, max, and mean
    self.mean += v;
    self.min = self.min.min(v);
    self.max = self.max.max(v);
  }

  fn finalize(&mut self) {
    let k: f64 = self.count as f64;
    self.sd = (self.qk/(k-1.0)).sqrt();
    self.mean /= k;
    self.median = self.med.get();
  }
}
