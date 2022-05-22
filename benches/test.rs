use brunch::Bench;
use std::time::Duration;

fn new_join(a: &str, b: &str) -> String {
  let buffer = [a, b].join("");
  buffer
}

fn new() -> String {
  let mut string: String = String::new();
  string = new_join(&string, "💖");
  string = new_join(&string, "👉👈");
  string
}

fn new2_join(a: &str, b: &str) -> String {
  let mut buffer: String = String::with_capacity(a.len() + b.len());
  buffer += a;
  buffer += b;
  buffer
}

fn new2() -> String {
  let mut string: String = String::new();
  string = new2_join(&string, "💖");
  string = new2_join(&string, "👉👈");
  string
}

fn old() -> String {
  let mut buffer: String = String::new();
  buffer.push_str("💖");
  buffer.push_str("👉👈");
  buffer
}

brunch::benches!(
  Bench::new("Insert", "old")
    .timed(Duration::from_secs(1))
    .with(|| old()),

  Bench::new("Insert", "new")
    .timed(Duration::from_secs(1))
    .with(|| new()),

  Bench::new("Insert", "new alternate")
    .timed(Duration::from_secs(1))
    .with(|| new2()),
);