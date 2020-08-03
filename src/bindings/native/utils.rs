#![allow(dead_code)]

use std::time::{Duration, Instant};
use rand;

#[macro_export]
macro_rules! log {
    ($format:literal, $( $t:tt )* ) => {
        println!($format, $( $t )*);
    }
}

pub struct Timer<'a> {
    name: &'a str,
    start: Instant,
}

impl<'a> Timer<'a> {
    pub fn new(name: &'a str) -> Timer<'a> {
        let start = Instant::now();
        Timer { name, start }
    }
}

impl<'a> Drop for Timer<'a> {
    fn drop(&mut self) {
        log!("{} - {}", self.name, self.start.elapsed().as_millis());
    }
}

pub fn set_panic_hook() {}


pub fn random() -> f64 {
    rand::random()  // TODO(ted): Is this the same as js_sys::Math::random()?
}