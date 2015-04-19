
use std::collections::BinaryHeap;
use std::cmp::Ordering;

#[derive(Copy, Clone, Default, PartialOrd, PartialEq)]
pub struct FloatVal {
  v: f64,
}

impl FloatVal {

  pub fn new(x: f64) -> FloatVal {
    FloatVal{v: x}
  }

  fn neg(&self) -> FloatVal {
    FloatVal{v : -self.v}
  }
}

// We implement ordering such the BinaryHeap corresponds to a minHeap
impl Ord for FloatVal {
  fn cmp(&self, other: &FloatVal) -> Ordering {
    self.v.partial_cmp(&other.v).unwrap_or(Ordering::Equal)
  }
}

impl Eq for FloatVal {}


// Median implements a running median on top of two binary heaps
#[derive(Default)]
pub struct Median {
  sm: BinaryHeap<FloatVal>,
  lg: BinaryHeap<FloatVal>,
  pub med: FloatVal,
}

impl Median {

  pub fn new() -> Median {
    Median {
      sm: BinaryHeap::new(),
      lg: BinaryHeap::new(),
      med: FloatVal{v: 0.0},
    }
  }


  pub fn get(&self) -> f64 {
    self.med.v
  }


  pub fn update(&mut self, item: FloatVal) {
    // insert first element
    if self.sm.len() == 0 && self.lg.len() == 0 {
      self.sm.push(item);
    // insert second element
    } else if self.sm.len() == 0 {
      if &item.neg() < self.lg.peek().unwrap() {
        let f = self.lg.pop().unwrap();
        self.lg.push(item.neg());
        self.sm.push(f.neg());
      } else {
        self.sm.push(item);
      }
    } else if self.lg.len() == 0 {
      if &item < self.sm.peek().unwrap() {
        let f = self.sm.pop().unwrap();
        self.sm.push(item);
        self.lg.push(f.neg());
      } else {
        self.lg.push(item.neg());
      }
    // third and other element
    } else {
      if item > self.med {
        self.lg.push(item.neg());
      } else if item < self.med {
        self.sm.push(item);
      } else {
        if self.sm.len() <= self.lg.len() {
          self.sm.push(item);
        } else {
          self.lg.push(item.neg());
        }
      }
    }
    // fix up heaps if their length differs by more than 2
    if self.sm.len() == self.lg.len() + 2 {
      self.lg.push(self.sm.pop().unwrap().neg());
    } else if self.lg.len() == self.sm.len() + 2 {
      self.sm.push(self.lg.pop().unwrap().neg());
    }

    // compute new median
    if self.sm.len() == self.lg.len() {
      self.med.v = 0.5 * (self.sm.peek().unwrap().v - self.lg.peek().unwrap().v);
    } else if self.sm.len() < self.lg.len() {
      self.med.v = -self.lg.peek().unwrap().v;
    } else {
      self.med.v = self.sm.peek().unwrap().v;
    }
  }
}






