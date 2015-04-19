
use std::env;
use std::f64;
use std::fs::File;
use std::io::{BufRead, BufReader};

mod stats;

#[derive(Default)]
struct Output {
  count: i64,
  min: f64,
  max: f64,
  mean: f64,
  median: f64,
  sd: f64,
}

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
  println!("#elems : {}", out.count);
  println!("min    : {}", out.min);
  println!("max    : {}", out.max);
  println!("mean   : {}", out.mean);
  println!("median : {}", out.median);
  println!("std    : {}", out.sd);
}


// compute_statistics computes basic statistics for the content of the
// provided BufReader
fn compute_statistics(buf: BufReader<File>) -> Output {
  let mut out = Output { count: 0, min: f64::MAX, max: -f64::MAX, mean: 0.0,
    median: 0.0, sd: 0.0};

  let mut qk = 0.0;
  let mut mk = 0.0;
  let mut med = stats::Median::new();
  for line_r in buf.lines() {
    let line = line_r.unwrap();
    let n_r = line.to_string().parse::<f64>();
    if n_r.is_err() {
      println!("Warning {} in line {} is not a number. Skipping", line, out.count);
      continue;
    }
    let n = n_r.unwrap();
    out.count += 1;

    // update median
    med.update(stats::FloatVal::new(n));

    // update variance
    let k: f64 = out.count as f64;
    qk += (k - 1.0) * (n - mk) * (n - mk) / k;
    mk += (n - mk) / k;

    // update min, max, and mean
    out.mean += n;
    out.min = out.min.min(n);
    out.max = out.max.max(n);
  }
  let k: f64 = out.count as f64;
  out.sd = (qk/(k-1.0)).sqrt();
  out.mean /= k;
  out.median = med.get();

  out
}

// usage prints a short message describing the usage of the function
fn usage() {
  println!("usage: stats <filename>");
}
